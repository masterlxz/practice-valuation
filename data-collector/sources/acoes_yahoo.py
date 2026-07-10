"""Cliente da API não-oficial do Yahoo Finance (histórico de dividendos).

Endpoint confirmado direto contra a API real (2026-07-10) — público, sem
chave, sem cadastro, funciona pra qualquer ticker da B3 (não só os 4
tickers de teste da brapi): GET /v8/finance/chart/{ticker}.SA com
range=10y&events=div devolve, além da série de preço (ignorada aqui), um
bloco `chart.result[0].events.dividends` — um dict de
{epoch_do_anuncio: {"amount": <R$/ação>, "date": <epoch_do_pagamento>}}.

**Achado**: essa rota não é documentada oficialmente pelo Yahoo (é a mesma
usada por baixo dos panos pela lib `yfinance`) — sem contrato formal de
estabilidade, mas amplamente usada e sem exigir cadastro nenhum, diferente
da bolsai (que bloqueia esse mesmo dado — `GET /dividends` — no plano
Free) e da brapi (que só libera dividendos de graça pros 4 tickers de
teste; qualquer outro ticker real exige plano pago, R$99,99+/mês).

Usado aqui só pro "dividendo médio por ação, últimos 5 anos completos" que
o Bazin pede — mesmo cálculo já usado com a bolsai (soma por ano, descarta
o ano corrente parcial, média dos últimos 5 anos completos).
"""

from collections import defaultdict
from datetime import datetime, timezone

import requests

YAHOO_CHART_URL = "https://query1.finance.yahoo.com/v8/finance/chart"

# Cobre o suficiente pra sempre sobrar 5 anos completos depois de descartar
# o ano corrente (parcial).
HISTORY_RANGE = "10y"
DIVIDENDS_YEARS_AVERAGED = 5


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
