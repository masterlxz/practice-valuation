"""Cliente da API do DefiLlama (TVL de chains/protocolos DeFi).

Endpoint confirmado direto contra a API real (2026-07-10) — público, sem
chave, sem cadastro: GET /v2/historicalChainTvl/{chain} devolve uma série
diária [{"date": <unix seconds>, "tvl": <float>}, ...], mais antiga primeiro.

Usado aqui só pro indicador `tvl_trend` do score cripto (Fase 3) — variação
percentual do TVL da Ethereum nos últimos ~30 dias.
"""

import requests

DEFILLAMA_BASE_URL = "https://api.llama.fi"

# A série é diária (confirmado contra a API real — datas consecutivas
# distam exatamente 86400s), então "30 dias atrás" é só contar 30 posições
# a partir do fim, sem precisar comparar datas.
TVL_TREND_LOOKBACK_DAYS = 30


def fetch_tvl_trend_mom(chain: str = "Ethereum") -> float:
    """Retorna a variação % do TVL da chain entre hoje e ~30 dias atrás."""
    response = requests.get(
        f"{DEFILLAMA_BASE_URL}/v2/historicalChainTvl/{chain}", timeout=15
    )
    response.raise_for_status()
    history = response.json()

    latest_tvl = history[-1]["tvl"]
    previous_tvl = history[-1 - TVL_TREND_LOOKBACK_DAYS]["tvl"]

    return (latest_tvl - previous_tvl) / previous_tvl * 100
