"""Orquestrador da coleta de dados (Fase 2).

Lê config.yaml, chama cada fonte configurada e grava os resultados direto no
SQLite compartilhado com o app Tauri. Chamado pelo comando Rust
`run_stock_collector` (src-tauri/src/commands/collector.rs) como subprocess,
mas também roda direto (`python3 main.py`) pra depurar sem precisar do app.
"""

import sqlite3
import sys
from datetime import datetime, timezone
from pathlib import Path

import yaml
from dotenv import load_dotenv

from sources import acoes_bolsai, acoes_brapi, cripto_defillama

BASE_DIR = Path(__file__).parent
DB_PATH = BASE_DIR / "practice_valuation.db"
CONFIG_PATH = BASE_DIR / "config.yaml"


def load_config() -> dict:
    with open(CONFIG_PATH) as f:
        return yaml.safe_load(f)


def collect_stock_quotes(tickers: list[str]) -> list[dict]:
    if not tickers:
        return []

    quotes = acoes_brapi.fetch_quotes(tickers)

    conn = sqlite3.connect(DB_PATH)
    conn.execute("PRAGMA journal_mode=WAL")
    now = datetime.now(timezone.utc).isoformat()
    conn.executemany(
        "INSERT INTO stock_quotes (ticker, price, source, fetched_at) VALUES (?, ?, ?, ?)",
        [(quote["ticker"], quote["price"], "brapi", now) for quote in quotes],
    )
    conn.commit()
    conn.close()

    return quotes


def collect_stock_fundamentals(tickers: list[str]) -> list[dict]:
    if not tickers:
        return []

    fundamentals = acoes_bolsai.fetch_fundamentals(tickers)

    conn = sqlite3.connect(DB_PATH)
    conn.execute("PRAGMA journal_mode=WAL")
    now = datetime.now(timezone.utc).isoformat()
    conn.executemany(
        "INSERT INTO stock_fundamentals (ticker, lpa, vpa, roe, source, fetched_at) "
        "VALUES (?, ?, ?, ?, ?, ?)",
        [
            (item["ticker"], item["lpa"], item["vpa"], item["roe"], "bolsai", now)
            for item in fundamentals
        ],
    )
    conn.commit()
    conn.close()

    return fundamentals


def collect_stock_dividends_avg(tickers: list[str]) -> list[dict]:
    if not tickers:
        return []

    dividends = acoes_bolsai.fetch_dividends_avg(tickers)

    conn = sqlite3.connect(DB_PATH)
    conn.execute("PRAGMA journal_mode=WAL")
    now = datetime.now(timezone.utc).isoformat()
    conn.executemany(
        "INSERT INTO stock_dividends_avg (ticker, avg_dividend_5y, source, fetched_at) "
        "VALUES (?, ?, ?, ?)",
        [(item["ticker"], item["avg_dividend_5y"], "bolsai", now) for item in dividends],
    )
    conn.commit()
    conn.close()

    return dividends


def _classify_signal(raw_value: float, green_boundary: float, red_boundary: float) -> str:
    """Mirrors `src-tauri/src/domain/crypto_score.rs::classify`.

    Duplicated here (not called via Tauri) because Python writes straight to
    the shared SQLite file with no IPC into Rust — same "no API between the
    two processes" architecture used for every other collector table. Keep
    both in sync if the classification rule ever changes.
    """
    higher_is_better = green_boundary > red_boundary
    if higher_is_better:
        if raw_value >= green_boundary:
            return "GREEN"
        if raw_value <= red_boundary:
            return "RED"
        return "NEUTRAL"
    if raw_value <= green_boundary:
        return "GREEN"
    if raw_value >= red_boundary:
        return "RED"
    return "NEUTRAL"


def collect_crypto_tvl_trend() -> dict:
    conn = sqlite3.connect(DB_PATH)
    conn.execute("PRAGMA journal_mode=WAL")

    threshold = conn.execute(
        "SELECT green_boundary, red_boundary FROM indicator_thresholds WHERE indicator = ?",
        ("tvl_trend",),
    ).fetchone()
    if threshold is None:
        conn.close()
        raise RuntimeError("No threshold configured for 'tvl_trend' — run migrations first")
    green_boundary, red_boundary = threshold

    raw_value = cripto_defillama.fetch_tvl_trend_mom()
    signal = _classify_signal(raw_value, green_boundary, red_boundary)
    reading_date = datetime.now(timezone.utc).date().isoformat()
    now = datetime.now(timezone.utc).isoformat()

    conn.execute(
        "INSERT INTO crypto_indicators "
        "(coin, indicator, reading_date, raw_value, signal, source, created_at) "
        "VALUES (?, ?, ?, ?, ?, ?, ?)",
        ("ETH", "tvl_trend", reading_date, raw_value, signal, "defillama", now),
    )
    conn.commit()
    conn.close()

    return {"indicator": "tvl_trend", "raw_value": raw_value, "signal": signal}


def main_crypto() -> int:
    reading = collect_crypto_tvl_trend()
    print(
        f"TVL Trend (MoM): {reading['raw_value']:.2f}% -> {reading['signal']}"
    )
    return 0


def main() -> int:
    load_dotenv(BASE_DIR / ".env")
    config = load_config()

    tickers = config.get("stocks", [])
    if not tickers:
        print("No stocks configured in config.yaml")
        return 0

    quotes = collect_stock_quotes(tickers)
    for quote in quotes:
        print(f"{quote['ticker']}: R$ {quote['price']}")
    print(f"Updated {len(quotes)} quote(s)")

    # bolsai requires a signed-up API key (unlike brapi's test tickers) — if
    # it's missing, skip this source with a clear message instead of failing
    # the whole run and losing the quotes collected above.
    try:
        fundamentals = collect_stock_fundamentals(tickers)
        for item in fundamentals:
            print(
                f"{item['ticker']}: LPA {item['lpa']} / VPA {item['vpa']} / ROE {item['roe']}%"
            )
        print(f"Updated {len(fundamentals)} fundamentals record(s)")

        dividends = collect_stock_dividends_avg(tickers)
        for item in dividends:
            print(f"{item['ticker']}: avg dividend/share (5y) R$ {item['avg_dividend_5y']:.4f}")
        print(f"Updated {len(dividends)} dividend average record(s)")
    except RuntimeError as err:
        print(f"Skipping bolsai collection: {err}")

    return 0


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "crypto":
        sys.exit(main_crypto())
    sys.exit(main())
