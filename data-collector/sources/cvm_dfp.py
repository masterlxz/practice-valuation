"""Cliente dos Dados Abertos da CVM (fundamentos pro DCF/FCFF).

Não é uma API REST como brapi/bolsai — a CVM publica **um zip por ano** com
as demonstrações financeiras de todas as ~870 companhias abertas juntas:
https://dados.cvm.gov.br/dados/CIA_ABERTA/DOC/DFP/DADOS/dfp_cia_aberta_{ano}.zip

Confirmado contra o arquivo real (2026-07-10): dentro do zip, vários CSVs
`;`-delimitados, encoding `latin1`, um por demonstração — `DRE_con` (resultado),
`BPA_con`/`BPP_con` (balanço, ativo/passivo), `DFC_MI_con` (fluxo de caixa,
método indireto) — cada linha é uma conta (`CD_CONTA`, código fixo e igual
pra qualquer empresa) de uma companhia (`CNPJ_CIA`/`CD_CVM`) num período
(`ORDEM_EXERC` = 'ÚLTIMO'/'PENÚLTIMO' — o próprio arquivo anual já traz os
dois últimos exercícios).

**Achado testando de verdade**: o `cvm_code` que a bolsai devolve (ex.:
"9512") vem sem zero à esquerda; o `CD_CVM` da CVM vem com zero à esquerda
("009512"). Comparar como texto não bate nunca — os dois precisam virar
`int` antes de comparar.

**Nem toda conta é tão padronizada quanto parece.** EBIT, dívida, caixa e as
peças do ΔNWC usam o mesmo `CD_CONTA` pra ~850 das ~870 empresas — seguro
ler direto. D&A e Capex não: o código varia empresa a empresa (a WEG rotula
a linha de Capex só como "Imobilizado"/"Intangível", sem a palavra
"Aquisição" que a maioria usa). Pra essas duas, a extração busca por
palavra-chave no texto da conta (`DS_CONTA`) em vez de um código fixo — e
se não achar exatamente um grupo de linhas confiável, devolve `None` em vez
de arriscar um número errado.

**Nº de ações não vem daqui.** O arquivo `composicao_capital` da CVM tinha
esse dado, mas testando contra a VALE3 real ele veio 1000x menor que o
número de ações de verdade (4,5 milhões em vez de ~4,5 bilhões — um erro
nos próprios dados da CVM pra essa companhia especificamente, não um bug de
leitura). A bolsai já devolve `shares_outstanding` correto e conferido no
`/fundamentals` (mesma chamada que já busca LPA/VPA/ROE/cvm_code) — reaproveitado
em `main.py` em vez de extrair (e arriscar) esse campo da CVM.
"""

import csv
import io
import zipfile
from pathlib import Path

import requests

CVM_ZIP_URL_TEMPLATE = (
    "https://dados.cvm.gov.br/dados/CIA_ABERTA/DOC/DFP/DADOS/"
    "dfp_cia_aberta_{year}.zip"
)
CACHE_DIR = Path(__file__).parent.parent / ".cache" / "cvm_dfp"

LATEST = "ÚLTIMO"
PRIOR = "PENÚLTIMO"

# ESCALA_MOEDA -> multiplicador pra converter VL_CONTA em reais.
_CURRENCY_SCALE = {"MIL": 1_000, "MILHAO": 1_000_000, "UNIDADE": 1}


def _zip_path(year: int) -> Path:
    return CACHE_DIR / f"dfp_cia_aberta_{year}.zip"


def _download_zip(year: int) -> Path:
    path = _zip_path(year)
    if path.exists():
        return path

    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    response = requests.get(CVM_ZIP_URL_TEMPLATE.format(year=year), timeout=60)
    response.raise_for_status()
    path.write_bytes(response.content)
    return path


def _resolve_zip_path() -> Path:
    """CVM nomeia o zip pelo ano do exercício coberto, não pelo ano de
    publicação — em julho/2026 o zip mais recente é o de 2025 (referente ao
    exercício fiscal encerrado em 2025-12-31, arquivado no início de 2026),
    não um zip "2026" (que só existiria a partir de ~março/2027). Tenta o
    ano anterior ao atual primeiro, cai pro ano anterior a esse em caso de
    404 (ex.: início de ano, antes da CVM publicar o zip mais recente).
    """
    from datetime import datetime, timezone

    candidate_year = datetime.now(timezone.utc).year - 1
    try:
        return _download_zip(candidate_year)
    except requests.HTTPError:
        return _download_zip(candidate_year - 1)


def _read_csv_from_zip(zf: zipfile.ZipFile, filename: str) -> list[dict]:
    with zf.open(filename) as raw:
        text = io.TextIOWrapper(raw, encoding="latin1")
        return list(csv.DictReader(text, delimiter=";"))


def _index_by_cvm_code(rows: list[dict]) -> dict[int, list[dict]]:
    index: dict[int, list[dict]] = {}
    for row in rows:
        index.setdefault(int(row["CD_CVM"]), []).append(row)
    return index


def _latest_version_rows(rows: list[dict]) -> list[dict]:
    """Uma companhia pode ter mais de uma `VERSAO` (retificação) pro mesmo
    exercício — mantém só as linhas da versão mais recente, senão contas
    duplicadas de uma retificação inflariam qualquer soma."""
    if not rows:
        return []
    max_version = max(int(row["VERSAO"]) for row in rows)
    return [row for row in rows if int(row["VERSAO"]) == max_version]


def _to_millions_brl(row: dict) -> float:
    scale = _CURRENCY_SCALE[row["ESCALA_MOEDA"]]
    return float(row["VL_CONTA"]) * scale / 1_000_000


def _find_exact(rows: list[dict], cd_conta: str, orden_exerc: str = LATEST) -> float:
    """Lê uma conta de código estável (EBIT, dívida, caixa, ...). Levanta
    `LookupError` se a conta não existir pra essa empresa/período — sinal
    de dado incomum (ex.: setor financeiro, taxonomia diferente), tratado
    pelo chamador como "pula essa empresa", não como zero."""
    candidates = _latest_version_rows(
        [r for r in rows if r["CD_CONTA"] == cd_conta and r["ORDEM_EXERC"] == orden_exerc]
    )
    if not candidates:
        raise LookupError(f"conta {cd_conta!r} ({orden_exerc}) não encontrada")
    return _to_millions_brl(candidates[0])


def _find_by_keyword(
    rows: list[dict],
    code_prefix: str,
    keywords: list[str],
    orden_exerc: str = LATEST,
) -> float | None:
    """Busca linhas por prefixo de código + palavra-chave no texto da conta
    (D&A, Capex — códigos não padronizados entre empresas). Descarta linhas
    "pai" (cujo código é prefixo do código de outra linha já casada, ex.:
    total `6.02` quando `6.02.02`/`6.02.03` também casaram) pra não somar
    subtotal + detalhe. Devolve `None` se sobrar zero linhas — nunca chuta."""
    matched = _latest_version_rows(
        [
            r
            for r in rows
            if r["CD_CONTA"].startswith(code_prefix)
            and r["ORDEM_EXERC"] == orden_exerc
            and any(keyword in r["DS_CONTA"].lower() for keyword in keywords)
        ]
    )
    if not matched:
        return None

    codes = {r["CD_CONTA"] for r in matched}
    leaves = [
        r
        for r in matched
        if not any(other != r["CD_CONTA"] and other.startswith(r["CD_CONTA"] + ".") for other in codes)
    ]
    return abs(sum(_to_millions_brl(r) for r in leaves))


def _nwc_change(bpa_rows: list[dict], bpp_rows: list[dict]) -> float:
    """ΔNWC = (Contas a Receber + Estoques − Fornecedores) no exercício
    atual menos o mesmo cálculo no exercício anterior — usa só os 3 códigos
    estáveis do balanço, evitando os códigos instáveis do fluxo de caixa
    (mesmo problema de instabilidade do Capex)."""

    def nwc_at(orden_exerc: str) -> float:
        receivables = _find_exact(bpa_rows, "1.01.03", orden_exerc)
        inventory = _find_exact(bpa_rows, "1.01.04", orden_exerc)
        payables = _find_exact(bpp_rows, "2.01.02", orden_exerc)
        return receivables + inventory - payables

    return nwc_at(LATEST) - nwc_at(PRIOR)


def fetch_dcf_fundamentals(ticker_cvm_codes: dict[str, str]) -> list[dict]:
    """Busca os 6 campos do DCF derivados de dados contábeis da CVM (EBIT,
    D&A, Capex, ΔNWC, dívida total, caixa) pra cada ticker. `shares_outstanding`
    não vem daqui — ver nota no topo do arquivo.

    `ticker_cvm_codes` é {ticker: cvm_code} — já resolvido via
    `acoes_bolsai.fetch_fundamentals` (mesma chamada que já busca LPA/VPA/ROE,
    sem chamada extra só pra achar a empresa). Retorna uma lista de dicts
    com `ticker`, `reference_year`, `ebit`, `depreciation_amortization`
    (pode ser `None`), `capex` (pode ser `None`), `nwc_change`, `total_debt`,
    `cash`. Um ticker sem dado encontrável (ex.: banco — taxonomia de DRE
    diferente, ver domain/dcf.rs) é ignorado, não derruba o restante.
    """
    zip_path = _resolve_zip_path()
    results = []

    with zipfile.ZipFile(zip_path) as zf:
        year = int(zip_path.stem.rsplit("_", 1)[-1])
        dre_rows = _read_csv_from_zip(zf, f"dfp_cia_aberta_DRE_con_{year}.csv")
        bpa_rows = _read_csv_from_zip(zf, f"dfp_cia_aberta_BPA_con_{year}.csv")
        bpp_rows = _read_csv_from_zip(zf, f"dfp_cia_aberta_BPP_con_{year}.csv")
        dfc_rows = _read_csv_from_zip(zf, f"dfp_cia_aberta_DFC_MI_con_{year}.csv")

    dre_by_cvm_code = _index_by_cvm_code(dre_rows)
    bpa_by_cvm_code = _index_by_cvm_code(bpa_rows)
    bpp_by_cvm_code = _index_by_cvm_code(bpp_rows)
    dfc_by_cvm_code = _index_by_cvm_code(dfc_rows)

    for ticker, cvm_code in ticker_cvm_codes.items():
        try:
            company_dre = dre_by_cvm_code[int(cvm_code)]
            company_bpa = bpa_by_cvm_code[int(cvm_code)]
            company_bpp = bpp_by_cvm_code[int(cvm_code)]
            company_dfc = dfc_by_cvm_code[int(cvm_code)]

            results.append(
                {
                    "ticker": ticker,
                    "reference_year": year,
                    "ebit": _find_exact(company_dre, "3.05"),
                    "depreciation_amortization": _find_by_keyword(
                        company_dfc,
                        "6.01.01",
                        ["depreciaç", "amortiza", "exaust"],
                    ),
                    "capex": _find_by_keyword(
                        company_dfc, "6.02", ["imobilizado", "intangív", "intangiv"]
                    ),
                    "nwc_change": _nwc_change(company_bpa, company_bpp),
                    "total_debt": (
                        _find_exact(company_bpp, "2.01.04")
                        + _find_exact(company_bpp, "2.02.01")
                    ),
                    "cash": _find_exact(company_bpa, "1.01.01"),
                }
            )
        except (KeyError, LookupError, IndexError):
            continue

    return results
