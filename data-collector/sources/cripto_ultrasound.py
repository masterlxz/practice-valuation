"""Cliente da API do ultrasound.money (supply de ETH, pra net issuance).

Confirmado direto contra a API real (2026-07-10) — público, sem chave. A
suposição inicial de "só WebSocket, sem API REST" estava errada: o backend
é open source (github.com/ultrasoundmoney/eth-analysis-rs, `axum`) e expõe
rotas REST de verdade em ultrasound.money/api/v2/fees/*.

GET /api/v2/fees/supply-over-time — devolve várias janelas de supply total
de ETH (m5, h1, d1, d7, d30, since_merge, since_burn), cada uma já cortada
pro período certo pelo próprio servidor. Usamos `d30` (últimos 30 dias) pro
indicador `net_issuance` do score cripto (Fase 3): variação % da supply
nesse período, anualizada.
"""

import requests

ULTRASOUND_BASE_URL = "https://ultrasound.money/api/v2/fees"

DAYS_IN_WINDOW = 30
DAYS_IN_YEAR = 365


def fetch_net_issuance_annualized_pct() -> float:
    """Retorna a variação % anualizada da supply de ETH nos últimos 30 dias.

    Positivo = supply crescendo (issuance > burn); negativo = "ultra sound"
    (burn > issuance).
    """
    response = requests.get(
        f"{ULTRASOUND_BASE_URL}/supply-over-time", timeout=15
    )
    response.raise_for_status()
    window = response.json()["d30"]

    supply_start = window[0]["supply"]
    supply_end = window[-1]["supply"]

    pct_over_window = (supply_end - supply_start) / supply_start * 100
    return pct_over_window * (DAYS_IN_YEAR / DAYS_IN_WINDOW)
