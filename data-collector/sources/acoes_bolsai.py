"""Cliente da API da bolsai (fundamentos e dividendos de ações BR).

Endpoints e formato de resposta confirmados em usebolsai.com/docs
(2026-07-10): base URL https://api.usebolsai.com/api/v1, autenticação via
header X-API-Key (chave gratuita obtida com login Google no dashboard deles
— sem tickers de teste sem chave, diferente da brapi).

GET /fundamentals/{ticker} — snapshot atual com ~27 indicadores; usamos só
lpa, vpa (Graham) e roe (Bancos).

GET /dividends/{ticker}?years=N — histórico de proventos com `annual_summary`
(um item por ano-calendário, mais recente primeiro). Usamos pra calcular o
"dividendo médio por ação, últimos 5 anos" que o Bazin pede.

**Achado testando contra a API real (2026-07-10)**: `/dividends` retorna 403
no plano Free — é exclusivo do plano Pro (a doc marca isso com uma badge
"PRO" que passou despercebido na primeira leitura). `fetch_dividends_avg`
trata esse 403 como esperado e levanta `RuntimeError` (mesmo tipo de erro de
`BOLSAI_API_KEY` ausente), pra `main.py` pular só essa coleta sem derrubar o
resto. Decisão do usuário (Sessão 5): não assinar o Pro por enquanto — o
dividendo médio do Bazin continua manual até isso mudar.
"""

import os
from datetime import datetime, timezone

import requests

BOLSAI_BASE_URL = "https://api.usebolsai.com/api/v1"

# Buffer acima de 5 pra sobrar anos completos depois de descartar o ano
# corrente (parcial — só teria os dividendos pagos até a data de hoje, o que
# puxaria a média pra baixo sem motivo).
DIVIDENDS_YEARS_REQUESTED = 6
DIVIDENDS_YEARS_AVERAGED = 5


def _headers() -> dict:
    api_key = os.environ.get("BOLSAI_API_KEY")
    if not api_key:
        raise RuntimeError(
            "BOLSAI_API_KEY not set — get a free key at usebolsai.com/dashboard "
            "and add it to data-collector/.env"
        )
    return {"X-API-Key": api_key}


def fetch_fundamentals(tickers: list[str]) -> list[dict]:
    """Busca LPA, VPA e ROE atuais de uma lista de tickers.

    Retorna uma lista de {"ticker": str, "lpa": float, "vpa": float, "roe": float}.
    Um ticker que falhar (404, etc.) é ignorado — não derruba o restante.
    """
    headers = _headers()
    results = []

    for ticker in tickers:
        response = requests.get(
            f"{BOLSAI_BASE_URL}/fundamentals/{ticker}",
            headers=headers,
            timeout=10,
        )
        if response.status_code == 404:
            continue
        response.raise_for_status()
        payload = response.json()

        results.append(
            {
                "ticker": payload["ticker"],
                "lpa": payload["lpa"],
                "vpa": payload["vpa"],
                "roe": payload["roe"],
            }
        )

    return results


def fetch_dividends_avg(tickers: list[str]) -> list[dict]:
    """Busca o dividendo médio por ação dos últimos 5 anos completos.

    Retorna uma lista de {"ticker": str, "avg_dividend_5y": float}. Tickers
    sem nenhum ano completo de histórico (ex: IPO recente) são ignorados.
    """
    headers = _headers()
    current_year = datetime.now(timezone.utc).year
    results = []

    for ticker in tickers:
        response = requests.get(
            f"{BOLSAI_BASE_URL}/dividends/{ticker}",
            params={"years": DIVIDENDS_YEARS_REQUESTED},
            headers=headers,
            timeout=10,
        )
        if response.status_code == 404:
            continue
        if response.status_code == 403:
            raise RuntimeError(
                "GET /dividends requires a bolsai Pro plan (usebolsai.com/pricing) "
                "— skipping average dividend collection"
            )
        response.raise_for_status()
        payload = response.json()

        complete_years = [
            entry
            for entry in payload.get("annual_summary", [])
            if entry["year"] != current_year
        ][:DIVIDENDS_YEARS_AVERAGED]

        if not complete_years:
            continue

        avg = sum(entry["total_per_share"] for entry in complete_years) / len(
            complete_years
        )
        results.append({"ticker": payload["ticker"], "avg_dividend_5y": avg})

    return results
