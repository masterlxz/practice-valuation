"""Cliente da API não-oficial do Yahoo Finance (cotação atual + histórico de
dividendos).

Endpoint confirmado direto contra a API real (2026-07-10, recheckado
2026-07-12 pra cotação) — público, sem chave, sem cadastro, funciona pra
qualquer ticker da B3 (diferente da brapi, cuja cotação de graça só cobre 4
tickers de teste sem token — ver histórico em `PROJECT_STATE.md`, Sessão 4/6):
GET /v8/finance/chart/{ticker}.SA devolve um bloco `chart.result[0].meta`
com `regularMarketPrice` (usado por `fetch_quotes`) e, quando
`events=div` é passado, também `chart.result[0].events.dividends` — um
dict de {epoch_do_anuncio: {"amount": <R$/ação>, "date":
<epoch_do_pagamento>}} (usado por `fetch_dividends_avg`).

**Achado**: essa rota não é documentada oficialmente pelo Yahoo (é a mesma
usada por baixo dos panos pela lib `yfinance`) — sem contrato formal de
estabilidade, mas amplamente usada e sem exigir cadastro nenhum.

`fetch_dividends_avg` calcula o "dividendo médio por ação, últimos 5 anos
completos" que o Bazin pede (soma por ano, descarta o ano corrente parcial,
média dos últimos 5 anos completos). `fetch_quotes` substituiu totalmente
`acoes_brapi.py` (removido) — a brapi exigia token pago pra qualquer ticker
fora da demo, o que quebrava a busca ad hoc por ticker (Fase "fetch por
ticker" nos formulários de valuation).
"""

from collections import defaultdict
from datetime import datetime, timezone

import requests

YAHOO_CHART_URL = "https://query1.finance.yahoo.com/v8/finance/chart"

# Cobre o suficiente pra sempre sobrar 5 anos completos depois de descartar
# o ano corrente (parcial).
HISTORY_RANGE = "10y"
DIVIDENDS_YEARS_AVERAGED = 5


def fetch_quotes(tickers: list[str]) -> list[dict]:
    """Busca a cotação atual de uma lista de tickers.

    Retorna uma lista de {"ticker": str, "price": float}. Tickers que
    falharem na API são ignorados — não derrubam o resto, mesmo padrão de
    `fetch_dividends_avg`.
    """
    results = []

    for ticker in tickers:
        try:
            response = requests.get(
                f"{YAHOO_CHART_URL}/{ticker}.SA",
                params={"range": "5d", "interval": "1d"},
                headers={"User-Agent": "Mozilla/5.0"},
                timeout=15,
            )
            response.raise_for_status()
            price = response.json()["chart"]["result"][0]["meta"]["regularMarketPrice"]
        except (requests.RequestException, KeyError, TypeError, IndexError):
            continue

        results.append({"ticker": ticker, "price": price})

    return results


def fetch_dividends_avg(tickers: list[str]) -> list[dict]:
    """Busca o dividendo médio por ação dos últimos 5 anos completos.

    Retorna uma lista de {"ticker": str, "avg_dividend_5y": float}. Tickers
    sem dividendo nenhum registrado (ex: growth stock, IPO recente sem ano
    completo) ou que falharem na API são ignorados — não derrubam o resto.
    """
    current_year = datetime.now(timezone.utc).year
    results = []

    for ticker in tickers:
        try:
            response = requests.get(
                f"{YAHOO_CHART_URL}/{ticker}.SA",
                params={"range": HISTORY_RANGE, "interval": "3mo", "events": "div"},
                headers={"User-Agent": "Mozilla/5.0"},
                timeout=15,
            )
            response.raise_for_status()
            chart_result = response.json()["chart"]["result"][0]
        except (requests.RequestException, KeyError, TypeError, IndexError):
            continue

        dividends = chart_result.get("events", {}).get("dividends", {})
        if not dividends:
            continue

        yearly_totals: dict[int, float] = defaultdict(float)
        for entry in dividends.values():
            year = datetime.fromtimestamp(entry["date"], tz=timezone.utc).year
            if year == current_year:
                continue
            yearly_totals[year] += entry["amount"]

        complete_years = sorted(yearly_totals, reverse=True)[:DIVIDENDS_YEARS_AVERAGED]
        if not complete_years:
            continue

        avg = sum(yearly_totals[year] for year in complete_years) / len(complete_years)
        results.append({"ticker": ticker, "avg_dividend_5y": avg})

    return results
