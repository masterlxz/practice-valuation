# data-collector

Coletores de dados em Python (ações BR + cripto) que escrevem direto no banco SQLite
compartilhado com o app (`../desktop`). Ver `PROJECT_STATE.md` na raiz do repo — Fase 2
(desenho completo das fontes está lá, seção "Fase 2 — Coleta de Dados").

## Setup

```
python3 -m venv .venv
.venv/bin/pip install -r requirements.txt
cp .env.example .env  # preencher BRAPI_TOKEN se for usar tickers além dos 4 de teste
```

## Rodar manualmente (sem passar pelo app)

```
.venv/bin/python3 main.py
```

Disparado pelo app via o comando Tauri `run_stock_collector`
(`../desktop/src-tauri/src/commands/collector.rs`), que roda esse mesmo
`main.py` como subprocess.

## Implementado

- `sources/acoes_brapi.py` — cotação atual de ações BR (Fase 2.2, parcial)

## Ainda não implementado

- `acoes_bolsai.py`, `cvm_dfp.py` (fundamentos de ações — Fase 2.2)
- Fontes de cripto (Fase 2.3), extração de PDF (Fase 2.4)
