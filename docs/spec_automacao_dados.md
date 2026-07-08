> **Nota (Sessão 1, ver `PROJECT_STATE.md`)**: este spec foi escrito quando o plano era
> automatizar a escrita numa planilha do Google Sheets. Essa rota foi abandonada em favor
> de um app desktop com banco de dados local (Practice Valuation). O levantamento de fontes
> de dados abaixo (bolsai, brapi, CVM, CoinGecko, DefiLlama, ultrasound.money, Etherscan,
> extração de PDF) continua válido — só o destino final dos dados mudou de "planilha" para
> "banco de dados local do app". Mantido aqui como referência histórica.

# Spec: Automação de Coleta de Dados (Ações BR + Cripto) → Google Sheets

## Objetivo
Um app Python local, rodando na máquina do Fabio (Arch Linux), que puxa dados de
ações brasileiras e de cripto via API e escreve automaticamente nas abas da
planilha de valuation e da futura planilha de cripto no Google Sheets — sem
copiar/colar manual de release em PDF.

## Arquitetura (visão geral)

```
data-collector/
├── main.py                  # orquestrador — roda tudo ou um módulo específico
├── config.yaml               # lista de tickers/moedas a acompanhar, chaves de API
├── sources/
│   ├── acoes_bolsai.py        # cliente da API bolsai (fundamentos B3)
│   ├── acoes_brapi.py         # cliente da API brapi (cotação + fallback)
│   ├── cripto_coingecko.py    # preço, market cap, volume
│   ├── cripto_defillama.py    # TVL
│   ├── cripto_ultrasound.py   # emissão líquida ETH (issuance - burn)
│   └── cripto_etherscan.py    # endereços ativos / transações
├── sheets/
│   └── writer.py              # escreve nas abas via Google Sheets API
├── .credentials/
│   └── service_account.json   # chave de service account do Google (git-ignored)
└── requirements.txt
```

## Autenticação no Google Sheets (a parte que costuma travar todo mundo)
Não usar OAuth por usuário (exige login toda hora). Usar **Service Account**:
1. Criar um projeto no Google Cloud Console (gratuito).
2. Ativar a API do Google Sheets + Google Drive.
3. Criar uma Service Account, baixar o JSON de credenciais.
4. **Compartilhar a planilha** com o e-mail da service account (tipo
   `nome@projeto.iam.gserviceaccount.com`) como Editor — igual convidar uma pessoa.
5. No Python, usar `gspread` + `google-auth` pra ler o JSON e escrever direto
   nas células (`worksheet.update()`), sem precisar de token renovado manualmente.

Isso resolve 100% do "como eu jogo pra planilha automaticamente" sem precisar
de Apps Script (que aí sim seria JavaScript).

## Fontes de dados

### Ações BR (fundamentos)
| Dado | Fonte primária | Fallback |
|---|---|---|
| P/L, P/VP, ROE, ROIC, margens, EV/EBITDA (27 indicadores TTM) | bolsai (200 req/dia grátis) | brapi (`modules=defaultKeyStatistics`) |
| Cotação atual | brapi (15.000 req/dia grátis) | bolsai |
| Balanço/DRE/DFC históricos (contas CVM brutas) | bolsai | brapi (`balanceSheetHistory`) |
| Dividendos históricos | ambas têm | — |

→ Cobre bem os indicadores de **triagem** (P/L, P/VP, ROE, DY, EV/EBITDA, CAGR
receita). Pros inputs finos do DCF completo (Capex de expansão vs manutenção,
ΔNWC detalhado) o script deixa pré-preenchido com o dado bruto da CVM, mas
ainda vale conferir contra o release nos casos historicamente problemáticos
(banco, incorporadora).

### Cripto (foco inicial: Ethereum)
| Indicador | Fonte | Gratuita? |
|---|---|---|
| Preço, market cap, volume | CoinGecko API | Sim |
| TVL (DeFi) | DefiLlama API | Sim |
| Emissão líquida (issuance − burn) | ultrasound.money API | Sim |
| Staking yield | stakingrewards.com | Tier free limitado |
| Endereços ativos / transações | Etherscan API | Sim (rate limit baixo) |
| Exchange Netflow | CryptoQuant / Glassnode | Pago (sem alternativa gratuita boa) |
| MVRV Z-Score, Puell Multiple | Glassnode | Pago |

→ 6 de 8 indicadores dá pra automatizar de graça. Os 2 restantes (Netflow,
MVRV/Puell) entram como célula manual com link direto pro dashboard de
referência, até decidir se vale assinar algo.

## Extração de dados de relatórios (pra parar de preencher manual)

Esse é o módulo que resolve o "mandar o PDF e sair sozinho". Dois caminhos,
em ordem de preferência:

### Caminho 1 (principal): CVM Dados Abertos — estruturado, sem ler PDF nenhum
A CVM publica o **DFP/ITR de toda empresa aberta em formato padronizado**
(plano de contas fixo: ex. conta `3.11` = Lucro Líquido, `2.03` = Patrimônio
Líquido, `2.01.04` = Estoques, igual pra Unifique, Grendene, BRBI ou Lavvi).
Isso é o dataset que bolsai/brapi consultam por trás pra calcular os 27
indicadores. Baixando direto (`dados.cvm.gov.br/dataset/cia_aberta-doc-dfp`),
dá pra montar seu próprio mapeamento de contas → campos do DCF (Receita,
EBIT, D&A, Capex, Dívida, etc.) sem depender de a API "empacotar" exatamente
o campo que você precisa. É estruturado, versionado por trimestre/ano, e não
quebra quando o layout do PDF muda.

```
sources/
└── cvm_dfp.py   # baixa o zip trimestral da CVM, filtra pela empresa,
                 # mapeia código de conta → campo do modelo (DCF/RNAV/Bancos)
```

### Caminho 2 (fallback): quando o dado só existe no texto/apresentação
Coisas como composição do landbank de uma incorporadora, ou um comentário
qualitativo do release, não vêm no DFP estruturado — só no PDF/apresentação
mesmo. Pra esses casos:
1. `pdfplumber` ou `PyMuPDF` extrai o texto e as tabelas do PDF.
2. Esse texto bruto vai pra API da Anthropic (Claude) com um prompt fixo
   pedindo pra devolver **só JSON** com os campos que faltam (mesma coisa que
   eu faço manualmente quando você me manda o PDF no chat — só que rodando
   como script, sem gastar sua mensagem).
3. O script valida o JSON e escreve na planilha, junto com a fonte
   ("Source: Release 4T25, pág. X") pra você conferir rapidinho se quiser.

```
sources/
└── pdf_extractor.py   # pdfplumber + chamada à API Claude com schema fixo
```

### Como funciona o download da CVM, na prática
Não é uma API tipo REST (você não chama `/empresa/FIQE3`). É um **arquivo zip
por ano**, com o balanço de todas as ~500 empresas abertas dentro:

```
https://dados.cvm.gov.br/dados/CIA_ABERTA/DOC/DFP/DADOS/dfp_cia_aberta_2025.zip
```

Dentro do zip, vários CSVs (um por demonstração: Balanço Ativo `BPA`, Balanço
Passivo `BPP`, Resultado `DRE`, Fluxo de Caixa `DFC_MI`, etc — sempre com
versão `_con` = consolidado e `_ind` = individual). Cada linha:

```
CNPJ_CIA | DENOM_CIA | CD_CVM | DT_REFER | CD_CONTA | DS_CONTA | VL_CONTA
```

`CD_CONTA` é o código fixo da conta (ex: `3.11` = Lucro Líquido, igual pra
qualquer empresa) e `VL_CONTA` o valor daquele período.

Fluxo do script:
1. Baixa o zip do ano (1x, todas as empresas vêm juntas).
2. Abre os CSVs com pandas, filtra pelas linhas da(s) empresa(s) de interesse
   (por `CNPJ_CIA` ou `DENOM_CIA`).
3. Pivota só os `CD_CONTA` que interessam pro modelo (mapeamento fixo, ex:
   `3.11`→Lucro Líquido, `2.01.04`→Estoques).
4. Escreve na planilha via `sheets/writer.py`.

Único detalhe: a CVM identifica empresa por CNPJ, não por ticker. Resolve com
uma chamada rápida à API bolsai/brapi só pra traduzir ticker → CNPJ, e usa a
CVM pro dado pesado.

```
sources/
└── cvm_dfp.py
    ├── baixar_zip_ano(ano)
    ├── ticker_para_cnpj(ticker)   # via bolsai/brapi
    └── extrair_contas(cnpj, lista_codigos_conta)
```


Pra maioria das empresas "normais" (que é o grosso da sua lista), o Caminho 1
sozinho já cobre praticamente tudo — Capex, D&A, ΔNWC, dívida, tudo isso vem
de contas padronizadas do DFP. O Caminho 2 só entra pontualmente, pra aquele
dado específico de incorporadora/banco que só está descrito em texto.

## Scheduling (rodando local)
Como é local e não precisa rodar toda hora, `cron` resolve sem infra extra:
```
# Ações: 1x por ano (quando sair balanço anual) — mas rodar mensal de checagem não custa nada
0 9 1 * * cd ~/data-collector && python main.py --modulo acoes

# Cripto: 1x por dia
0 8 * * * cd ~/data-collector && python main.py --modulo cripto
```

## Próximo passo sugerido
Levar este arquivo pro Claude Code e pedir pra ele:
1. Escrever o `main.py` e os módulos de `sources/`.
2. Criar o passo a passo de setup da Service Account (ele consegue te guiar
   clicando no Cloud Console).
3. Testar contra 2-3 tickers/moedas reais antes de rodar full.

Esse é um projeto separado do TruthID — pode ser um repo próprio ou uma pasta
`tools/` dentro dele, como preferir.
