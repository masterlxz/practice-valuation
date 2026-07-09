"""Cliente da API da brapi (cotação atual de ações BR).

Endpoint e formato de resposta confirmados em brapi.dev/docs (2026-07-09):
GET /api/v2/stocks/quote?symbols=PETR4,VALE3 — aceita vários tickers numa
só chamada. Token via header Authorization: Bearer é opcional para 4
tickers de teste (PETR4, MGLU3, VALE3, ITUB4); qualquer outro ticker exige
token (BRAPI_TOKEN em .env).
"""

import os

import requests

BRAPI_QUOTE_URL = "https://brapi.dev/api/v2/stocks/quote"


def fetch_quotes(tickers: list[str]) -> list[dict]:
    """Busca a cotação atual de uma lista de tickers.

    Retorna uma lista de {"ticker": str, "price": float}.
    """
    token = os.environ.get("BRAPI_TOKEN")
    headers = {"Authorization": f"Bearer {token}"} if token else {}

    response = requests.get(
        BRAPI_QUOTE_URL,
        params={"symbols": ",".join(tickers)},
        headers=headers,
        timeout=10,
    )
    response.raise_for_status()
    payload = response.json()

    return [
        {
            "ticker": result["symbol"],
            "price": result["data"]["regularMarketPrice"],
        }
        for result in payload["results"]
    ]
