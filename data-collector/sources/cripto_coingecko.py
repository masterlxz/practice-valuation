"""Cliente da API do CoinGecko (dado de mercado: preço, market cap, volume).

Endpoint confirmado direto contra a API real (2026-07-16) — público, sem
chave: GET /api/v3/coins/{id}/market_chart devolve séries diárias de
market cap e volume de trading (mesmo shape, [[timestamp_ms, valor], ...]).

Usado aqui só pro indicador `nvt_ratio` do score cripto (Fase 3). Ressalva
importante, decidida com o usuário: o "volume" usado aqui é volume de
EXCHANGE (trading), não volume liquidado on-chain (a definição original de
Willy Woo pro NVT). Blockchair tem o dado on-chain de verdade, mas só o
valor de hoje, sem histórico grátis — não dá pra calcular a média móvel de
90 dias que a regra de sinal pede sem esperar ~90 dias de coleta acumulada.
Essa proxy é menos "pura", mas funciona com uma chamada só, desde o
primeiro dia.
"""

import requests

COINGECKO_BASE_URL = "https://api.coingecko.com/api/v3"

NVT_MA_WINDOW_DAYS = 90


def fetch_nvt_ratio_vs_ma90(coin_id: str = "ethereum") -> float:
    """Retorna o NVT de hoje dividido pela média móvel dos últimos 90 dias.

    <1.0 = NVT de hoje abaixo da média (rede "barata" perto do volume) — bom.
    >1.0 = NVT de hoje acima da média (rede "cara" perto do volume) — alerta.
    """
    response = requests.get(
        f"{COINGECKO_BASE_URL}/coins/{coin_id}/market_chart",
        params={"vs_currency": "usd", "days": NVT_MA_WINDOW_DAYS, "interval": "daily"},
        timeout=15,
    )
    response.raise_for_status()
    data = response.json()

    market_caps = [point[1] for point in data["market_caps"]]
    volumes = [point[1] for point in data["total_volumes"]]
    daily_nvt = [cap / vol for cap, vol in zip(market_caps, volumes)]

    # O ponto mais recente é "hoje" (dia ainda em andamento) — a média dos
    # 90 dias fechados anteriores é o que compõe a MA de referência.
    nvt_today = daily_nvt[-1]
    nvt_ma90 = sum(daily_nvt[:-1]) / len(daily_nvt[:-1])

    return nvt_today / nvt_ma90
