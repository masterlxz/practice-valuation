# Practice Valuation — Estado do Projeto

> Este arquivo é o centro de controle do projeto. Atualizado a cada sessão de trabalho.
> Pode ser lido por qualquer instância do Claude Code em qualquer máquina para retomar o contexto.
> Última atualização: 2026-07-11 (Sessão 8, fim — **Fase 5.2 completa**: verificação periódica dos `alert_rule` ativos. Background task (`tauri::async_runtime::spawn` + `tokio::time::interval`, 5 min, primeiro tick imediato na inicialização) reavalia cada regra ativa contra o dado mais recente já coletado (`stock_quotes`/`crypto_indicators`) — nunca dispara o coletor Python sozinho. Estado rastreado via tabela nova append-only `alert_event` (mesma filosofia "nunca sobrescreve" de `valuation`): só grava uma linha quando o estado disparado/não-disparado muda, nos dois sentidos. `AlertRuleView`/`list_alert_rules` estendidos com `is_triggered`/`last_message`; `AlertsPanel.tsx` ganhou polling (`refetchInterval` 30s) e badge "Triggered". `cargo check`/`cargo test --lib` (42 testes) e `npx tsc --noEmit` limpos; usuário testou de verdade com `docker compose restart` forçando o tick imediato — confirmou os 4 cenários (sem dado ainda não dispara, alerta cripto dispara com dado real existente, alerta de ação dispara e depois limpa quando o preço volta, pausa mantém o último estado conhecido). Falta só 5.3 (notificação nativa do SO) pra fechar a Fase 5.)

---

## Diretriz de código (IMPORTANTE — sempre seguir)

**Todo código novo deve ser escrito em inglês — sem exceção.** O repositório será público no GitHub.
- Strings visíveis ao usuário (UI, mensagens de erro, labels, placeholders): inglês
- Nomes de variáveis, funções, classes, arquivos: inglês
- Comentários no código: podem ficar em português (não são visíveis ao usuário e facilitam o aprendizado)
- Esta regra vale para qualquer linguagem que venha a ser usada (Python, TypeScript, Rust, etc. — a decidir)

---

## Diretriz de segurança (IMPORTANTE — repo público no GitHub)

O projeto vai para um repositório **público**. Cuidados a partir do primeiro commit:
- **Nunca commitar chaves de API, tokens ou credenciais.** Tudo isso vai em `.env` (ou keyring do SO), com `.env.example` no repo mostrando as variáveis esperadas sem valores reais.
- `.gitignore` deve cobrir `.env`, banco de dados local (`*.db`, `*.sqlite`), pastas de build/dist, e qualquer arquivo de credencial.
- **Nunca hardcodar dados pessoais de portfólio** (valores investidos, quantidades, saldo) em código de exemplo, testes ou fixtures — usar dados fictícios.
- Antes de cada `git push` para o remoto público, revisar `git status`/`git diff` procurando por segredos, mesmo em arquivos com nome inofensivo.
- Ao decidir o banco de dados local (Fase 1), garantir que o arquivo do banco fique fora do controle de versão por padrão.

---

## Diretriz de ensino (IMPORTANTE — ler antes de cada sessão)

O usuário não é iniciante em programação (ver Fase 2 abaixo — já desenhou sozinho uma arquitetura de coleta de dados via APIs, com bom domínio de Python). O ponto de partida é **iniciante em construir uma aplicação desktop completa**: escolha de framework de UI, empacotamento/distribuição, banco de dados local, e organização de um projeto maior que um script. **Tem pouca intimidade com Rust especificamente** (pedido explícito do usuário, Sessão 1) — Python é a linguagem de referência pra analogias.

**Regras para o Claude:**
- Ir com calma — construir aos poucos, sessão a sessão, sem pressa pra "terminar tudo de uma vez"
- Antes de decidir framework/lib/arquitetura, explicar as opções e trade-offs e esperar a decisão do usuário — nunca decidir sozinho por ele quando a decisão for de gosto/direção do projeto
- Explicar o conceito novo (de UI, empacotamento, banco de dados, etc.) antes de escrever o código que o usa
- Não assumir conhecimento prévio de frameworks de UI desktop, ORMs/SQL local, ou empacotamento de apps — mas pode assumir Python e lógica de programação em geral
- **No código Rust especificamente**: ir com calma e explicar de verdade — o quê cada trecho faz, por que a sintaxe é do jeito que é, e comparar com o equivalente em Python quando ajudar (ex: `struct` ~ `dataclass`, `Result<T, E>` ~ exceção/erro explícito em vez de try/except, ownership/borrow como conceito novo sem equivalente direto em Python)
- Perguntar se o usuário entendeu antes de avançar quando o conceito for novo
- Nunca escrever um bloco grande de código sem explicar o que faz e por quê

---

## Ritmo e Expectativa do Projeto

- **Prioridade do usuário hoje é o TruthID.** O Practice Valuation existe pra resolver uma dor real (organizar preços-teto e acompanhar cripto), mas anda em ritmo de fundo — sessões espaçadas, sem pressa de terminar rápido.
- **Filosofia de construção**: expandir aos poucos, sessão a sessão. Por enquanto o objetivo é **só funcionar** pro uso pessoal do usuário — nada de over-engineering pensando em recursos futuros antes da hora.
- **Visão de longo prazo**: virar uma ferramenta "foda" de verdade (mais fontes de dado, mais metodologias, alertas afinados, talvez sync/mobile — ver "Roadmap de Evoluções Planejadas"), mas isso é destino, não requisito do MVP.
- Ao retomar uma sessão depois de um tempo parado, é normal — não tratar como projeto abandonado, só como projeto em ritmo lento.

---

## O que é o Practice Valuation

App desktop pessoal para acompanhar teses de investimento em ações (B3) e criptoativos.
Substitui a ideia original de planilha (ver Fase 2, histórico) por um app com banco de dados local.

**O que ele precisa fazer (visão do usuário, ainda sendo refinada):**
- Puxar o máximo de dados possível de fontes externas (fundamentos de ações BR, dados on-chain/mercado de cripto), com espaço pra ajuste manual quando necessário
- Guardar **múltiplos preços-teto/cálculos de valuation por ativo**, cada um com seu próprio conjunto de premissas (ex: duas projeções do mesmo ativo com taxas de crescimento diferentes, ambas salvas e comparáveis lado a lado)
- Cadastrar premissas por ativo (incluindo cripto) e monitorar indicadores automaticamente
- Avisar o usuário quando um ativo entrar em "zona de compra" segundo as premissas cadastradas
- Banco de dados **local** por enquanto — sync entre máquinas/nuvem é ideia pra mais adiante (ver Roadmap)

**Decidido até agora** (ver "Decisões de Arquitetura em Aberto"):
- Stack híbrida: app em **Tauri + Rust + React/TypeScript** (reaproveitando o padrão do TruthID), coleta de dados em **Python** (ver Fase 2), os dois se comunicando só através de um banco **SQLite** local compartilhado — sem API/IPC entre eles

- UI: **Tailwind CSS + shadcn/ui (Radix) + TanStack Table**, visual **arejado tipo dashboard** (não denso tipo planilha, apesar da ideia original de "funcionar como planilha" — isso ficou pro comportamento/dado, não pra densidade visual)

**Ainda não decidido**:
- Biblioteca de gráfico (pra tela de cripto/indicadores, Fase 4.3) — avaliar quando chegar lá (candidatos: Recharts, ou lightweight-charts da TradingView, mais voltada pra preço/candlestick)
- ~~Lista exata de metodologias de preço-teto~~ — entregue na Sessão 1, ver Fase 3

---

## Status Geral

```
Fase 0 — Fundamentos & Decisões de Arquitetura   [~] Em andamento (0.1–0.5 ✓, falta 0.6)
Fase 1 — Modelo de Dados (schema do banco local)  [~] Em andamento (migrations rodando normalmente a cada modelo, falta só formalizar 1.3 como concluída)
Fase 2 — Coleta de Dados (ações BR + cripto)      [~] Em andamento (cotação e dividendo médio do Bazin via Yahoo Finance — brapi removida na Sessão 10 —, fundamentos LPA/VPA/ROE via bolsai e todas as entradas contábeis do DCF via CVM funcionando ponta a ponta — nenhuma pendência de dado de ações sobrando; TVL Trend (DefiLlama) e Net Issuance (ultrasound.money) automatizados (2 dos 9 indicadores cripto) — ver Log de Sessões; os outros 7 indicadores cripto seguem sem fonte gratuita conhecida)
Fase 3 — Motor de Cálculo (preço-teto/valuation)  [x] Completa — 7 modelos de ação + score cripto (9 indicadores), todos ponta a ponta
Fase 4 — Interface Desktop                        [~] Em andamento (shadcn/ui + TanStack Table instalados, tela de valuations salvos completa incl. detalhe fino de premissas, 7 formulários + painel cripto vestidos, identidade visual dark+verde definida; Sessão 10: painel cru de tabelas de teste removido, cada formulário ganhou botão de buscar dado por ticker que preenche os campos automaticamente)
Fase 5 — Monitoramento & Alertas                  [~] Em andamento (5.1 e 5.2 completas — cadastro + verificação periódica; falta 5.3 notificação)
Fase 6 — Publicação (GitHub público)               [ ] Não iniciada
Fase 7 — Chat de IA Integrado                      [ ] Não iniciada (planejada na Sessão 10, ver Fases Detalhadas — implementação em fatias, começando pelo Gemini)
```

---

## Ambiente de Desenvolvimento

**Docker** — decidido na Sessão 1, mesmo padrão usado no TruthID: um único container com Node + Rust + WebKitGTK (pra abrir a janela do Tauri) e também **Python3 + venv** (pra rodar os coletores de dados chamados pelo próprio app). `docker compose up` sobe tudo, X11 do host repassado pro container pra a janela do app aparecer na tela — nada precisa ser instalado na máquina.

Diferente do TruthID (que precisava de acesso a USB pra Ledger), este projeto não mexe com hardware — o container fica mais simples e menos privilegiado (sem `privileged: true`, sem montar `/dev`).

Criado na Fase 0.5: `desktop/Dockerfile`, `desktop/docker-compose.yml`, `desktop/dev.sh` (`xhost +local:docker && docker compose up`).

**⚠️ Cuidado (achado na Sessão 1)**: a pasta do app se chama `desktop/`, igual a do TruthID — sem um `name:` explícito no topo do `docker-compose.yml`, o Compose usa o nome da pasta como nome do projeto e **colide** com as imagens/volumes do TruthID (`desktop-desktop`, `desktop_cargo-*`). Por isso o `docker-compose.yml` daqui tem `name: practice-valuation` logo na primeira linha — não remover.

**⚠️ Cuidado com espaço em disco**: a máquina roda os dois projetos (TruthID + Practice Valuation) e o disco de 32GB vive perto do limite por causa dos caches Docker do TruthID (imagens Flutter/Gradle/NDK, cache do cargo). Antes de builds Docker pesados, checar `df -h /` — na Sessão 1 o disco chegou a 100% (0 disponível) durante o setup inicial e isso **causou perda de arquivos** (os 3 arquivos de Docker recém-criados sumiram no meio de uma operação). `docker image prune -f` remove imagens órfãs com segurança (não mexe em nada usado pelo TruthID); ir além disso (limpar volumes/imagens nomeadas do TruthID) é decisão do usuário, não fazer sem perguntar.

---

## Arquitetura de Código

Decidido na Sessão 1: mesmo sendo um projeto pessoal, vale organizar bem desde o início — "fácil manutenção" não significa construir mais funcionalidade agora, significa estruturar bem o pouco que já existe.

**Camadas no lado Rust** (convenção adotada, aplica a partir da Fase 3):
- **`commands/`** — a "cola" exposta ao React via `invoke()`. Fina: só recebe o pedido e chama a camada de baixo, não tem regra de negócio
- **`domain/`** (ou `valuation/`) — as funções puras de cálculo (Bazin, DCF, etc.) e a lógica do score cripto. Não sabem nada de banco nem de Tauri — só recebem números/dados, devolvem números/resultado. É a "função pura" já mencionada na Fase 3, só que com um lugar físico definido
- **Repository/entities (SeaORM)** — só sabe conversar com o banco

Princípio: não misturar regra de negócio com acesso a banco — o mesmo motivo pelo qual qualquer linguagem separa "service layer" de "data layer".

**Tratamento de erro (Rust)**: **`thiserror`** — um enum de erro próprio (`AppError::AssetNotFound`, `AppError::InvalidGuard`, etc.) que serializa pro React como JSON estruturado (`{ code, message }`), em vez de string solta. Decidido na Sessão 1 em vez de `anyhow` (mais genérico/dinâmico, bom pra prototipagem rápida, mas não dá pra distinguir tipos de erro na UI depois).

**Busca de dados no React**: **TanStack Query** pra chamar os comandos Tauri (`invoke()`) — cuida de cache, loading, erro e refetch de forma consistente em toda tela, em vez de cada componente reinventar isso com `useState`/`useEffect`. Decidido na Sessão 1; mesma família do TanStack Table já escolhido pra grid (Fase 0.4).

**Testes**: funções de `domain/` são puras (input → output, sem I/O) — dá pra testar sem precisar de banco nem de mock, então a prática é escrever teste unitário junto de cada função de cálculo conforme ela é escrita (não é uma decisão de infraestrutura, é só disciplina a manter).

---

## Fases Detalhadas

### Fase 0 — Fundamentos & Decisões de Arquitetura

**Objetivo**: decidir, com calma e com explicação de trade-offs, a stack do projeto antes de escrever código de verdade.

**Etapas**:
- [x] 0.1 — Nome do projeto → **Practice Valuation** (repo: `practice-valuation`), decidido na Sessão 1
- [x] 0.2 — Framework do app desktop → **Tauri + Rust + React/TypeScript** (mesmo padrão do TruthID), decidido na Sessão 1
- [x] 0.3 — Banco de dados local → **SQLite** (compartilhado entre o app Tauri/Rust e os coletores em Python), decidido na Sessão 1
- [x] 0.4 — Stack/lib de UI e direção visual → **Tailwind + shadcn/ui + TanStack Table**, visual **arejado tipo dashboard** (não denso), decidido na Sessão 1
- [x] 0.5 — Estrutura inicial do repositório, criada na Sessão 1:
  - `desktop/` — projeto Tauri + React + TS, gerado via `create-tauri-app` e renomeado (`practice-valuation`). Tailwind v4 já plugado (`@tailwindcss/vite`, `src/index.css`) — shadcn/ui e TanStack Table entram quando a Fase 4 começar a construir telas de verdade
  - `desktop/Dockerfile` + `docker-compose.yml` + `dev.sh` — ambiente de dev (ver "Ambiente de Desenvolvimento")
  - `data-collector/` — pasta reservada pro coletor Python (Fase 2), com só um `README.md` e `requirements.txt` vazio por enquanto — implementação real ainda não começou
  - Ainda falta: README.md na raiz do repo, LICENSE
- [ ] 0.6 — Checklist de segurança aplicado desde o primeiro commit (ver "Diretriz de segurança" acima)

---

### Fase 1 — Modelo de Dados

**Objetivo**: desenhar o schema do banco local que sustenta tudo — ativos, premissas, cálculos salvos, indicadores e alertas.

**Fonte da verdade pro schema**: as fórmulas completas de cada metodologia estão na Fase 3 (inputs, cálculo e guarda de erro, modelo por modelo).

**Entidades decididas** (revisado 2x depois da spec chegar — ver as duas notas de mudança de abordagem abaixo):
- `asset` — ativo acompanhado (ação BR ou cripto), com tipo, ticker/símbolo, nome
- **`valuation`** — tabela compartilhada por todos os modelos, com os campos comuns da "regra geral": `ticker`, `ano_ref`, `preco_atual`, `model` (qual dos 7 modelos), `preco_justo` (calculado, cacheado, nulo se a guarda de erro impediu o cálculo), `margem_seguranca`, `veredito`, `data_ultima_atualizacao`. Cada linha é um cálculo salvo — nada é sobrescrito, dá pra ter várias linhas do mesmo ticker com premissas diferentes (o "múltiplos preços-teto salvos" pedido desde o início). É a tabela que alimenta a tela de listagem (Fase 4.1) — uma consulta só, sem `UNION`
- **Uma tabela pequena de inputs por modelo**, ligada a `valuation` por FK (`valuation_id`), só com os campos específicos daquele modelo: `bazin_inputs`, `graham_inputs`, `gordon_ddm_inputs`, `dcf_fcff_inputs`, `bank_pb_roe_gordon_inputs`, `realty_rnav_inputs`, `projected_ceiling_price_inputs` (ver campos de cada um na Fase 3)
- `cripto_indicadores` — série temporal do score cripto: `moeda`, `data`, `indicador`, `valor_bruto`, `sinal` (verde/vermelho), `fonte` — permite plotar a evolução do score, não só o snapshot do dia
- `tracked_indicator` / `alert` — ainda a desenhar (Fase 5), quando entrarmos no motor de monitoramento/zona de compra

**Mudança de abordagem #1 (Sessão 1, depois da spec chegar)**: a ideia anterior de premissas genéricas em JSON (`assumption_set` flexível) foi **substituída** por tabelas rígidas por modelo, como o próprio spec funcional sugere — agora que os campos de cada metodologia são conhecidos e estáveis (não é mais "esperando a lista"), colunas tipadas por modelo são mais simples de validar (ex: as guardas `WACC−g <= 0`, `Ke <= g`) e mais fáceis de consultar do que um blob JSON.

**Mudança de abordagem #2 (Sessão 1, revisão pedida pelo usuário)**: a primeira versão dessa correção tinha virado "uma tabela por modelo" **auto-contida** (7 tabelas, cada uma repetindo os campos comuns tipo `ticker`/`ano_ref`/`preco_justo`). O usuário pediu uma revisão pensando em manutenção de longo prazo, e isso foi trocado por **`valuation` compartilhada + tabela de inputs por modelo** (acima) — evita repetir os campos comuns 7 vezes (mudar um campo comum = 1 migration, não 7) e deixa a tela de "listar tudo" trivial. Os inputs continuam tipados por modelo (não regrediu pra JSON) — só o que era comum foi extraído.

**Regra geral, comum a todos os modelos de ação** (ver spec): `margem_segurança = (preço_justo − preço_atual) / preço_justo`; `veredito` = BARATO se margem > 0, senão CARO. Todo modelo também carrega `ticker`, `ano_ref` (o app calcula `anos_desatualizado = ano_atual − ano_ref` e sinaliza: ≤0 em dia, ==1 atenção, ≥2 desatualizado) e `preço_atual` (API com fallback manual).

**Como Rust e Python acessam o mesmo banco**: os dois já rodam dentro do mesmo container (decisão da Fase 0 — stack híbrida), então não precisa de rede/API entre eles — só apontar os dois pro mesmo arquivo `.db`. Arquivo físico decidido: `data-collector/practice_valuation.db` (já coberto pelo `*.db` do `.gitignore` da raiz; a pasta já é bind-mount, então o arquivo sobrevive entre execuções do container).

**Etapas**:
- [x] 1.1 — Entidades validadas — desbloqueado pela chegada do spec funcional (Sessão 1). Ver "Mudança de abordagem" acima
- [x] 1.2 — Driver/ORM Rust: **SeaORM** — decidido na Sessão 1 (revisado depois de decidir `rusqlite` na mesma sessão). Motivo: o usuário já tem hábito de pensar em ORM (estilo Django/SQLAlchemy/ActiveRecord); `rusqlite` exigiria escrever SQL cru e mapear linha a linha na mão, atrito maior do que ganho de simplicidade pra quem tá aprendendo Rust e banco ao mesmo tempo. SeaORM imita bem esse modelo mental (`Entity::find().all(&db)`, migrations via `sea-orm-cli`, geração de entity a partir do schema). É assíncrono, mas isso não é custo extra real — o Tauri já roda sobre `tokio`. Trade-off aceito: SeaORM é mais novo/menos batalhado que Diesel (a alternativa "ORM maduro", descartada pela sintaxe de query mais macro-pesada e curva de compilador mais dura)
- [x] 1.2b — **Modo WAL** (Write-Ahead Log) do SQLite será ligado por padrão — Rust e Python são processos diferentes lendo/escrevendo o mesmo arquivo, e WAL deixa isso coexistir melhor (menos "database is locked")
- [x] 1.3 — Migrations iniciais (abordagem simples: arquivos SQL versionados aplicados em ordem, sem framework pesado). Rodando normalmente desde a Sessão 2 — cada modelo/indicador novo ganha sua própria migration (`sea-orm-cli migrate generate`), aplicada com `migrate up`. Marco final: 9 migrations aplicadas (`valuation`+`bazin_inputs`, uma por modelo de ação, `indicator_thresholds`+`crypto_indicators`)

---

### Fase 2 — Coleta de Dados

**Objetivo**: puxar o máximo de dado possível de fontes externas, com fallback manual quando a fonte automática não cobre.

**Histórico**: o levantamento abaixo foi desenhado pelo usuário antes deste projeto virar app desktop, pensando em escrever direto numa planilha do Google Sheets (via Service Account + `gspread`). Essa rota foi abandonada na Sessão 1 — o desenho de fontes/APIs e o pipeline de dados continuam válidos, só o destino final mudou de "planilha" pra "banco de dados local do app" (o módulo `sheets/writer.py` e a autenticação via Service Account descritos na ideia original não se aplicam mais).

**Fontes já mapeadas**:
| Categoria | Dado | Fonte primária | Fallback |
|---|---|---|---|
| Ações BR | Fundamentos (P/L, P/VP, ROE, ROIC, margens, EV/EBITDA — 27 indicadores TTM) | bolsai (200 req/dia grátis) | — |
| Ações BR | Cotação atual | Yahoo Finance (não-oficial, grátis pra qualquer ticker — trocou a brapi na Sessão 10, que exigia token pago fora de 4 tickers demo) | — |
| Ações BR | Balanço/DRE/DFC histórico (contas CVM brutas) | bolsai / CVM Dados Abertos (DFP/ITR) | — |
| Ações BR | Dividendos históricos | Yahoo Finance | — |
| Cripto | Preço, market cap, volume | CoinGecko | — |
| Cripto | TVL (DeFi) | DefiLlama | — |
| Cripto | Emissão líquida (issuance − burn, ETH) | ultrasound.money | — |
| Cripto | Endereços ativos/transações diárias | Etherscan (rate limit baixo) | — |
| Cripto | Exchange netflow, MVRV Z-Score, Puell Multiple | CryptoQuant/Glassnode (pago, sem alternativa gratuita boa) | manual, link pro dashboard |
| Cripto | Staking Yield líquido | stakingrewards.com (free tier limitado) | manual |
| PDF/release não estruturado | Campos qualitativos (landbank, comentários) | pdfplumber/PyMuPDF + API Claude (schema fixo) | preenchimento manual |

Cobre bem os indicadores de **triagem** (P/L, P/VP, ROE, DY, EV/EBITDA, CAGR receita) e 6 dos 8 indicadores de cripto de graça. Pros inputs finos do DCF completo (Capex de expansão vs manutenção, ΔNWC detalhado) o script deixa pré-preenchido com o dado bruto da CVM, mas ainda vale conferir contra o release nos casos historicamente problemáticos (banco, incorporadora).

**Estrutura de módulos planejada** (pasta `data-collector/`, ver Fase 0.5):
```
data-collector/
├── main.py                    # orquestrador — roda tudo ou um módulo específico
├── config.yaml                # lista de tickers/moedas a acompanhar, chaves de API
├── requirements.txt
└── sources/
    ├── acoes_bolsai.py         # cliente da API bolsai (fundamentos B3)
    ├── acoes_yahoo.py          # cliente do Yahoo Finance (cotação + dividendo médio)
    ├── cvm_dfp.py              # baixa o zip trimestral da CVM, mapeia conta → campo do modelo
    ├── pdf_extractor.py        # pdfplumber/PyMuPDF + chamada à API Claude com schema fixo
    ├── cripto_coingecko.py     # preço, market cap, volume
    ├── cripto_defillama.py     # TVL
    ├── cripto_ultrasound.py    # emissão líquida ETH (issuance − burn)
    ├── cripto_etherscan.py     # endereços ativos / transações
    └── cripto_stakingrewards.py # staking yield líquido
```
Chaves de API ficam em `.env`/`config.yaml` fora do controle de versão (ver "Diretriz de segurança").

**CVM Dados Abertos — como funciona na prática** (fonte principal pro DCF/RNAV/Bancos): não é uma API tipo REST (não dá pra chamar `/empresa/FIQE3`). É um **arquivo zip por ano**, com o balanço de todas as ~500 empresas abertas dentro:
```
https://dados.cvm.gov.br/dados/CIA_ABERTA/DOC/DFP/DADOS/dfp_cia_aberta_2025.zip
```
Dentro do zip, vários CSVs (um por demonstração: Balanço Ativo `BPA`, Balanço Passivo `BPP`, Resultado `DRE`, Fluxo de Caixa `DFC_MI` — sempre com versão `_con` = consolidado e `_ind` = individual). Cada linha: `CNPJ_CIA | DENOM_CIA | CD_CVM | DT_REFER | CD_CONTA | DS_CONTA | VL_CONTA`. `CD_CONTA` é o código fixo da conta (ex: `3.11` = Lucro Líquido, `2.03` = Patrimônio Líquido, `2.01.04` = Estoques — igual pra qualquer empresa aberta) e `VL_CONTA` o valor daquele período. É o mesmo dataset que bolsai/brapi consultam por trás — baixando direto, dá pra montar o próprio mapeamento conta → campo do DCF (Receita, EBIT, D&A, Capex, Dívida) sem depender de a API "empacotar" exatamente o campo necessário, e não quebra quando o layout de um PDF muda.

Fluxo do `cvm_dfp.py`:
1. `baixar_zip_ano(ano)` — baixa o zip do ano uma vez (todas as empresas vêm juntas)
2. Abre os CSVs com pandas, filtra pelas linhas da(s) empresa(s) de interesse (por `CNPJ_CIA` ou `DENOM_CIA`)
3. `ticker_para_cnpj(ticker)` — a CVM identifica empresa por CNPJ, não por ticker; resolve com uma chamada rápida à API bolsai/brapi só pra traduzir
4. `extrair_contas(cnpj, lista_codigos_conta)` — pivota só os `CD_CONTA` que interessam pro modelo (mapeamento fixo)

Pra maioria das empresas "normais" (o grosso da lista), esse caminho sozinho já cobre praticamente tudo — Capex, D&A, ΔNWC, dívida, tudo vem de contas padronizadas do DFP.

**Extração via PDF (fallback, só quando o dado não é estruturado)**: coisas como composição de landbank de uma incorporadora, ou comentário qualitativo do release, não vêm no DFP — só no PDF/apresentação. Pra esses casos: `pdfplumber`/`PyMuPDF` extrai texto e tabelas → vai pra API da Anthropic (Claude) com um prompt fixo pedindo só JSON com os campos que faltam (a mesma coisa que fazer manualmente mandando o PDF no chat, só que como script) → o script valida o JSON e grava no banco junto com a fonte ("Source: Release 4T25, pág. X") pra conferência rápida.

**Etapas**:
- [x] 2.1 — Decidir onde/como a coleta roda → **processo Python separado**, disparado **manualmente por um botão na UI** ("Run"/"Atualizar dados"), sem cron/scheduler (a ideia original de cron — 1x/ano ações, 1x/dia cripto — foi descartada, ver Sessão 1). Mecanismo:
  - Frontend: botão chama `invoke()` de um comando Tauri
  - Backend (Rust): comando assíncrono dispara o script Python como subprocesso (não trava a UI), espera terminar
  - Python: puxa os dados das fontes e grava direto no SQLite compartilhado
  - Frontend: enquanto roda, mostra spinner; ao terminar, mostra um resumo (quantos ativos, sucesso/erro) — sem log ao vivo linha a linha por enquanto (pode vir depois se sentir falta)
  - **Guarda contra clique duplo/spam**: desabilitar o botão no frontend enquanto roda **e** ter uma trava no lado Rust (ex: `Mutex`/flag no estado do app) que recusa uma segunda chamada concorrente mesmo se disparada rápido demais — evita dois processos Python escrevendo no mesmo SQLite ao mesmo tempo e evita estourar rate limit das APIs gratuitas
  - A Fase 5 (alertas) pode um dia precisar de checagem periódica dos indicadores **já salvos** — isso é diferente de "puxar dado novo" e fica pra quando chegarmos lá
- [x] 2.2 — Implementar clientes de fonte de dados de ações — **`acoes_brapi.py` (cotação) concluído na Sessão 4**; **`acoes_bolsai.py` (fundamentos LPA/VPA/ROE) e `cvm_dfp.py` (fundamentos do DCF, incluindo alíquota efetiva) concluídos na Sessão 5**; **`acoes_yahoo.py` (dividendo médio 5 anos, via API não-oficial do Yahoo Finance) concluído na Sessão 6**, substituindo a bolsai (bloqueada, 403 Pro-only) e resolvendo pra qualquer ticker real (não só demo, diferente da brapi). **Sessão 10**: `acoes_brapi.py` removido inteiramente — sua limitação de token pago fora dos 4 tickers demo quebrava a busca ad hoc por ticker recém-adicionada nos formulários; cotação migrou pra `acoes_yahoo.py::fetch_quotes` (mesmo endpoint não-oficial já usado pro dividendo médio, testado contra ticker real fora da demo — BBAS3 — antes de trocar). Todas as entradas contábeis do DCF automatizadas; só sobram as 5 premissas de mercado (Beta, Rf, prêmio de risco, Kd, g), que nunca vêm de balanço. **Nenhuma pendência de dado de ações sobrando**
- [~] 2.3 — Implementar clientes de fonte de dados de cripto — **`cripto_defillama.py` (TVL Trend) e `cripto_ultrasound.py` (Net Issuance) concluídos na Sessão 5**, ambos sem cadastro. Achado corrigido: a suposição inicial de que ultrasound.money não tinha API pública (só WebSocket) estava **errada** — o backend é open source (`eth-analysis-rs`, axum) e expõe rotas REST reais em `/api/v2/fees/*`, achadas lendo o código-fonte no GitHub em vez de só a doc/site. `cripto_etherscan.py` tentado (Sessão 5) e descartado — todo o módulo `stats` de séries diárias é Etherscan API Pro-only, sem workaround gratuito razoável pra `active_addresses_trend`. `staking_yield` (stakingrewards.com) investigado na Sessão 6 e descartado — sem free tier de verdade, só planos pagos. `cripto_coingecko.py` (dado de referência, não alimenta nenhum dos 9 indicadores) segue faltando, mas sem urgência. `mvrv_z_score`, `puell_multiple` e `exchange_netflow` seguem sem fonte gratuita conhecida — continuam manuais (ver mapeamento de fontes)
- [ ] 2.4 — Fallback de extração via PDF (`pdf_extractor.py` — pdfplumber + Claude), quando necessário

---

### Fase 3 — Motor de Cálculo (Preço-Teto/Valuation)

**Objetivo**: calcular e salvar preços-teto/valuation com premissas customizáveis, permitindo múltiplos cálculos por ativo. Metodologias entregues pelo usuário na Sessão 1 — esta seção é a fonte da verdade completa (não precisa consultar outro arquivo).

#### Regra geral (vale pra todos os modelos de ação)

Todo modelo carrega 3 campos fixos além dos específicos — **ticker** (texto, ex: `FIQE3`), **ano de referência** (`ano_ref`, ano-base dos dados usados) e **preço atual** (R$, de API com fallback manual) — e termina com a mesma "cauda final":
```
margem_seguranca = (preco_justo − preco_atual) / preco_justo
veredito         = "BARATO" se margem_seguranca > 0, senão "CARO"
```
O app também calcula `anos_desatualizado = ano_atual − ano_ref` e sinaliza: `<=0` em dia, `==1` atenção, `>=2` desatualizado — é o campo que avisa quando revisar aquela empresa.

#### 1. DCF / FCFF (empresas "normais")

**Quando usar**: empresa com capital de giro e capex previsíveis (varejo, indústria, tech, utilities). Não usar em banco ou incorporadora.

| Input | Unidade |
|---|---|
| Receita Líquida | R$ milhões |
| EBIT | R$ milhões |
| Alíquota Efetiva de IR | % |
| D&A (Depreciação/Amortização) | R$ milhões |
| Capex | R$ milhões |
| ΔNWC (variação capital de giro) | R$ milhões |
| Dívida Total | R$ milhões |
| Caixa | R$ milhões |
| Nº de Ações | milhões |
| Beta | número |
| Rf (taxa livre de risco) | % |
| Prêmio de Risco de Mercado | % |
| Kd (custo da dívida) | % |
| g (crescimento na perpetuidade) | % |

```
FCFF         = EBIT × (1 − IR) + D&A − Capex − ΔNWC
Ke (CAPM)    = Rf + Beta × Prêmio_Risco_Mercado
E (equity)   = Preço_Atual × Nº_Ações
WACC         = [E / (E + Dívida)] × Ke + [Dívida / (E + Dívida)] × Kd × (1 − IR)
Valor_Firma  = FCFF × (1 + g) / (WACC − g)
Valor_Equity = Valor_Firma − Dívida_Total + Caixa
Preco_Justo  = Valor_Equity / Nº_Ações
```
**Guarda**: se `WACC − g <= 0`, não calcular (modelo quebra matematicamente) — mostrar aviso em vez de número.

#### 2. Gordon / DDM (Dividend Discount Model)

**Quando usar**: boa pagadora de dividendo, crescimento previsível.

| Input | Unidade |
|---|---|
| Dividendo Atual (D0) | R$/ação |
| Crescimento Esperado dos Dividendos (g) | % |
| Ke (retorno exigido) | % |

```
D1          = D0 × (1 + g)
Preco_Justo = D1 / (Ke − g)
```
**Guarda**: `Ke > g`, senão inválido.

#### 3. Bazin

**Quando usar**: "vaca leiteira" (bancão, elétrica, saneamento), foco em yield de dividendo.

| Input | Unidade |
|---|---|
| Dividendo Médio por Ação (últimos 5 anos) | R$/ação |
| Yield Desejado | % (default sugerido: 6%) |

```
Preco_Teto = Dividendo_Médio / Yield_Desejado
```

#### 4. Graham (Graham Number)

**Quando usar**: filtro rápido de margem de segurança, qualquer empresa com lucro e patrimônio positivos.

| Input | Unidade |
|---|---|
| LPA (Lucro por Ação) | R$/ação |
| VPA (Valor Patrimonial por Ação) | R$/ação |

```
Graham_Number = RAIZ(22.5 × LPA × VPA)
```
**Guarda**: se LPA ≤ 0 ou VPA ≤ 0, não calcular (empresa com prejuízo ou PL negativo não se encaixa nesse método).

#### 5. Bancos (P/B via ROE-Gordon)

**Quando usar**: bancos e instituições financeiras — FCFF não serve porque dívida é matéria-prima do negócio, não uma alavancagem a evitar.

| Input | Unidade |
|---|---|
| VPA (Valor Patrimonial por Ação) | R$/ação |
| ROE | % |
| Payout | % |
| Ke (retorno exigido) | % |

```
g_sustentável = ROE × (1 − Payout)
P/B_Justo     = (ROE − g_sustentável) / (Ke − g_sustentável)
Preco_Justo   = P/B_Justo × VPA
```
**Guarda**: `Ke > g_sustentável`.

#### 6. Incorporadoras (RNAV)

**Quando usar**: construtoras/incorporadoras — o "estoque" é imóvel, não dá pra projetar FCFF de forma suave trimestre a trimestre.

| Input | Unidade |
|---|---|
| Landbank a Valor de Mercado | R$ milhões |
| Estoque a Valor de Mercado | R$ milhões |
| Caixa Líquido (caixa − dívida, pode ser negativo) | R$ milhões |
| Nº de Ações | milhões |

```
RNAV_Total = Landbank + Estoque + Caixa_Líquido
RNAV/Ação  = RNAV_Total / Nº_Ações
```
(`RNAV/Ação` entra no lugar de `preco_justo` na regra geral.)

#### 7. Preço Teto Projetivo

**Quando usar**: mesma lógica do Bazin, mas trazendo N anos de crescimento esperado pra frente e descontando a valor presente — útil quando se quer o teto "olhando pra frente", não só o dividendo de hoje.

| Input | Unidade |
|---|---|
| Dividendo Atual (D0) | R$/ação |
| Crescimento Esperado (g) | % |
| Anos de Projeção (N) | inteiro (default sugerido: 5) |
| Yield Desejado (alvo, estilo Bazin) | % (default sugerido: 6%) |
| Ke (taxa de desconto) | % |

```
Dividendo_Projetado_N = D0 × (1 + g)^N
Preco_Teto_Futuro_N   = Dividendo_Projetado_N / Yield_Desejado
Preco_Teto_Projetivo  = Preco_Teto_Futuro_N / (1 + Ke)^N
```
(`Preco_Teto_Projetivo` entra como `preco_justo` na regra geral.)

#### Persistência (ver Fase 1)

Tabela `valuation` compartilhada (`ticker`, `ano_ref`, `preco_atual`, `model`, `preco_justo` cacheado, `margem_seguranca`, `veredito`, `data_ultima_atualizacao`) + uma tabela pequena de inputs por modelo (só os campos específicos, ligada por FK). Permite mostrar histórico ("como essa margem evoluiu ano a ano") sem recalcular tudo toda vez, salvar quantos cálculos o usuário quiser por ativo (premissas diferentes = linhas diferentes, nada sobrescreve), e listar tudo com uma consulta só na `valuation`.

#### Score de Cripto (Ethereum) — score contínuo

Diferente de ação (1x/ano), aqui é um **score contínuo**: cada indicador vira verde (bom pra compra/manter) ou vermelho (sinal de reduzir risco), e o app soma quantos estão verdes de um total de 9 — contagem objetiva, não "vibe".

| # | Indicador | O que mede | Fonte | Automatizável? | Regra de sinal (ponto de partida — ajustável) |
|---|---|---|---|---|---|
| 1 | MVRV Z-Score | Preço vs custo-base médio da rede | Glassnode (pago) | Não (fallback manual) | Verde se < 0 · Vermelho se > 7 |
| 2 | NVT Ratio | "P/L" da rede (valor de mercado / volume transacionado) | Calculável com dado on-chain | Parcial | Verde se abaixo da média móvel de 90d · Vermelho se muito acima |
| 3 | Puell Multiple | Emissão diária (USD) vs média histórica | Glassnode (pago) | Não (fallback manual) | Verde se < 0.5 · Vermelho se > 4 |
| 4 | Emissão Líquida (issuance − burn) | ETH é deflacionário ou não no período | ultrasound.money | Sim | Verde se negativa (deflacionário) · Vermelho se fortemente positiva |
| 5 | Staking Yield Líquido | Retorno real do staking, descontada a diluição | stakingrewards.com | Sim (free tier) | Verde se yield real > 2% · Vermelho se perto de 0 ou negativo |
| 6 | TVL DeFi (Ethereum) | Uso real da rede em DeFi | DefiLlama | Sim | Verde se em tendência de alta (MoM) · Vermelho se queda consistente |
| 7 | Endereços Ativos / Transações Diárias | Adoção/atividade da rede | Etherscan | Sim | Verde se crescendo (MoM/YoY) · Vermelho se caindo |
| 8 | Exchange Netflow | Saída (acumulação) ou entrada (venda) líquida das corretoras | CryptoQuant/Glassnode (pago) | Não (fallback manual) | Verde se saída líquida (negativo) · Vermelho se entrada líquida forte |
| 9 | Fees de Rede vs Emissão | "Receita líquida" real do protocolo pós EIP-1559 | ultrasound.money / Etherscan | Sim | Verde se fees líquidas cobrindo bem a emissão · Vermelho se dependente de emissão alta |

**Score final** = verdes / 9. Leitura sugerida (ajustável depois de ver rodando um tempo): **7-9 verdes** → tese intacta, manter/aportar · **4-6 verdes** → neutro, observar de perto · **0-3 verdes** → considerar reduzir risco/posição.

**Persistência**: tabela `cripto_indicadores` com `moeda`, `data`, `indicador`, `valor_bruto`, `sinal` (verde/vermelho), `fonte` — dá histórico de série temporal, dá pra plotar a evolução do score ao longo do tempo, não só o snapshot do dia.

**⚠️ Importante**: os thresholds acima (`< 0`, `> 7`, `> 2%`, etc.) são ponto de partida razoável baseado em uso histórico de mercado, **não são regra imutável** — o app precisa deixar esses números configuráveis (não hardcoded), porque o usuário provavelmente vai querer calibrar depois de ver como cada indicador se comporta na prática.

**Etapas**:
- [x] 3.1 — Lista de metodologias entregue (Sessão 1) — ver esta seção completa
- [x] 3.2 — Modelar cada metodologia (dos 7 modelos acima) como função pura Rust: inputs (tabela específica do modelo) → resultado (`preco_justo`, `margem_seguranca`, `veredito`), aplicando as guardas de erro — **concluído na Sessão 3**: os 7 modelos (Bazin, Graham, Gordon/DDM, DCF/FCFF, Bancos, RNAV, Preço Teto Projetivo) fechados ponta a ponta
- [x] 3.3 — Motor do score cripto — **concluído na Sessão 4**: sinal verde/neutro/vermelho por indicador com threshold configurável (tabela `indicator_thresholds`), leituras salvas em série temporal (`crypto_indicators`), score somado no front (verdes/9). Ver Log de Sessões pra detalhes de schema/domínio
- [ ] 3.4 — Permitir salvar quantos cálculos o usuário quiser por ativo (já é a natureza do schema — cada linha é um cálculo, nada sobrescreve), todos comparáveis lado a lado na UI — a parte de schema já está resolvida, falta só a tela de comparação (Fase 4)

---

### Fase 4 — Interface Desktop

**Objetivo**: telas simples, "planilha-like", que dão espaço pra edição manual quando preciso.

**⚠️ Nota (Sessão 3)**: as telas dos modelos já implementados (Bazin, Graham) são propositalmente cruas — `<input>` HTML puro com classes utilitárias do Tailwind, sem os componentes do shadcn/ui instalados ainda. É rascunho funcional pra provar a fatia vertical (cálculo → banco → tela) de cada modelo, não a interface final. Decisão: terminar a Fase 3 (os 7 modelos + cripto) com esse padrão cru primeiro, e só então entrar na Fase 4 de verdade — instalar shadcn/ui, desenhar a navegação real (lista de ativos, histórico de cálculos salvos) e vestir os formulários de uma vez, em vez de estilizar um por um sem ainda saber todos os inputs que a navegação final vai precisar acomodar.

**Etapas**:
- [x] 4.1 — Tela: lista de ativos acompanhados — **concluída na Sessão 4** como "Saved Valuations": tickers distintos derivados da tabela `valuation` (sem tabela `asset` própria — ver Log de Sessões)
- [x] 4.2 — Tela: detalhe do ativo (histórico de cálculos salvos) — **concluída na Sessão 4**: comparação lado a lado dos campos comuns, detalhe fino (linha expansível "Assumptions" por cálculo) e **edição/exclusão em lugar** (coluna "Actions": View/Edit/Delete) — corrige um cálculo salvo sem virar linha nova, e remove um cálculo (cascade limpa a tabela de inputs específica sozinho)
- [x] 4.3 — Tela: cripto/indicadores — **vestida com shadcn/ui na Sessão 4** (mesmo painel de registro/placar, agora com Card/Select/Table/Badge em vez de HTML cru)
- [ ] 4.4 — Tela: alertas/zona de compra
- [x] 4.5 — Direção visual → **arejado, tipo dashboard** (Tailwind + shadcn/ui + TanStack Table), decidido na Sessão 1; **identidade de cor definida na Sessão 4** — dark navy + verde claro, inspirada no TruthID (ver Log de Sessões)

---

### Fase 5 — Monitoramento & Alertas

**Objetivo**: cadastrar premissas de compra por ativo e avisar o usuário quando o indicador entrar na zona configurada.

**Etapas**:
- [x] 5.1 — Cadastro de regra de alerta (tabela `alert_rule` polimórfica: `target_type` "stock_price" reusa o `fair_price` já calculado numa valuation salva, "crypto_indicator" reusa o signal GREEN/RED já calculado via `indicator_thresholds`; sem checagem periódica nem notificação ainda — só o CRUD)
- [x] 5.2 — Verificação periódica (background task no próprio app — `tauri::async_runtime::spawn` + `tokio::time::interval` a cada 5 min, reavalia contra dado já coletado, sem disparar o coletor; estado rastreado via tabela append-only `alert_event`)
- [~] 5.3 — Notificação nativa do SO via `tauri-plugin-notification`, disparada só ao entrar em triggered. Código completo e compilando/testando limpo; falta validar de verdade que a notificação aparece (bloqueado por um problema de rede do Docker no host, não relacionado ao projeto — ver Sessão 9)

---

### Fase 6 — Publicação (GitHub Público)

**Etapas**:
- [ ] 6.1 — Checklist de segurança final (ver "Diretriz de segurança") antes do primeiro push público
- [ ] 6.2 — README explicando o projeto (em inglês, já que o repo é público)
- [ ] 6.3 — LICENSE
- [ ] 6.4 — `git init` + primeiro commit
- [ ] 6.5 — **Ideia registrada na Sessão 10** (não planejada em detalhe ainda, só anotada): aviso de atualização disponível — um indicador discreto num cantinho da tela quando existir uma versão nova do app. Candidato natural: `tauri-plugin-updater` (checa contra um manifesto de release, ex. GitHub Releases) — só faz sentido depois que 6.1–6.4 existirem (precisa ter release/versionamento publicado pra ter contra o que comparar)

---

### Fase 7 — Chat de IA Integrado (ideia levantada na Sessão 10, não iniciada)

**Objetivo**: um chat de IA dentro do próprio app, num painel lateral flutuante (sobrepõe a tela atual, não é uma aba nova), onde o usuário usa a própria chave de API (Gemini/Claude/ChatGPT) pra tirar dúvidas sobre suas valuations e alertas salvos ("cara, essa valuation deu estranho, por quê?"). O chat tem acesso **só de leitura** ao banco (sem criar/editar nada) e a um "repertório" fixo de como os modelos do sistema funcionam. Histórico da conversa vive em memória enquanto o app está aberto — reseta só quando o app fecha, não quando o painel é fechado/reaberto.

Planejado com `/plan` na Sessão 10 (pesquisa real contra a doc do Gemini via `WebFetch`, e contra a doc/discussão do GitHub do Tauri sobre storage seguro — Stronghold, a opção mais óbvia, está sendo **descontinuada** na v3, então não é a escolha certa pra começar algo novo agora). Escopo combinado com o usuário: implementar em fatias, começando só pelo Gemini funcionando de ponta a ponta; Claude e OpenAI ficam desenhados na abstração mas com erro claro de "ainda não implementado" até uma sessão futura continuar.

**Decisões já tomadas com o usuário** (Sessão 10, antes de qualquer código):
- Painel lateral flutuante (shadcn `Sheet`, ainda não instalado no projeto — hoje só tem `badge`/`button`/`card`/`input`/`label`/`select`/`table`/`tabs`), não uma aba nova na `Tabs` existente
- Acesso ao banco **só leitura** por enquanto (sem criar valuation/disparar coletor via chat)
- Chave de API guardada no keyring do SO (não em texto puro em lugar nenhum, nunca vai pro SQLite nem pro git) — **risco em aberto**: o Linux usa Secret Service via D-Bus (mesmo barramento que a notificação nativa da Fase 5.3 precisou), mas ter o barramento montado não garante que exista um keyring daemon do outro lado dentro do container Docker de dev. Tratamento combinado: tentar de verdade, e se não achar um keyring disponível, mostrar erro claro na hora de salvar a chave em vez de travar silenciosamente — só descobrimos testando de verdade
- Histórico reseta ao fechar o **app**, não o painel — state React vive acima do `Sheet`, sobrevive abrir/fechar

**Etapas** (cada uma pode ser uma sessão/fatia separada):
- [ ] 7.1 — Storage seguro da chave: crate `keyring-rs` (não `tauri-plugin-stronghold`, que está sendo descontinuado) chamado direto de comandos Tauri novos (`store_api_key`/`get_api_key_status`/`delete_api_key` — a chave em si nunca volta pro frontend, só um booleano "tem chave guardada"). Testar de verdade contra o container de dev **antes** de construir qualquer UI em cima — é o maior risco técnico da feature inteira
- [ ] 7.2 — Cliente HTTP do Gemini em Rust: `reqwest` novo no `Cargo.toml` (nenhum comando hoje faz chamada HTTP de fora, só subprocess Python — primeira vez que o Rust fala com uma API de terceiro direto). Endpoint confirmado na Sessão 10 via doc real: `POST generativelanguage.googleapis.com/v1beta/{model}:generateContent?key=...`, body `{contents: [{role, parts:[{text}]}], systemInstruction: {parts:[{text}]}}` — reconferir o nome exato do modelo (`gemini-*-flash` ou equivalente vigente) contra a doc na hora de implementar, não usar de memória
- [ ] 7.3 — Contexto de leitura do banco: comando novo que busca `valuation` (todas, mesmo padrão "sem paginação" já usado no resto do app) + `alert_event` recentes, formata como texto compacto, concatenado com um repertório fixo (escrito à mão) explicando os 7 modelos de valuation e o score cripto — vira o `systemInstruction` de cada chamada
- [ ] 7.4 — UI do painel: shadcn `Sheet` (novo componente, `radix-ui` já é dependência — só falta gerar o wrapper), botão flutuante de abrir/fechar, lista de mensagens, campo de chave de API (só se ainda não tiver uma guardada), estado do histórico num nível acima do `Sheet` (context ou state em `App.tsx`) pra sobreviver abrir/fechar
- [ ] 7.5 — Abstração multi-provider: enum/trait `ChatProvider` em Rust já desenhado pros 3 desde o início, mas só o braço do Gemini implementado de verdade nesta fatia; Claude e OpenAI retornam um erro tipado claro ("provedor ainda não implementado") em vez de crashar ou fingir que funciona
- [ ] 7.6 (sessão futura) — Implementar Claude (`api.anthropic.com/v1/messages`, header `x-api-key` + `anthropic-version`) — API já bem conhecida, não precisa de pesquisa nova
- [ ] 7.7 (sessão futura) — Implementar OpenAI (`api.openai.com/v1/chat/completions`, header `Authorization: Bearer`) — reconferir se ainda é Chat Completions ou se compensa migrar pra Responses API (a OpenAI está empurrando essa migração, viu isso na pesquisa da Sessão 10, mas não aprofundado)

---

### Fase 8 — Sync Multi-Dispositivo via TruthID + IPFS (ideia levantada na Sessão 11, não iniciada)

**Objetivo**: sincronizar valuations/alertas salvos entre dispositivos (celular, outro PC) de forma descentralizada — sem servidor próprio do Practice Valuation — reaproveitando a identidade e a infraestrutura já existentes no TruthID (outro projeto do usuário, `~/Documents/workspace/truthid`).

**Só brainstorm nesta sessão — nada implementado, nenhum `/plan` rodado ainda.** Registrado porque cruza os dois repositórios e envolveria mudanças no TruthID também (ver `PROJECT_STATE.md` do TruthID, seção "Roadmap de Evoluções Planejadas", entrada da Sessão 94).

**Desenho discutido**:
- **P2P direto entre devices foi descartado** — usuário não gosta da abordagem (conexão ao vivo entre dispositivos, NAT, complexidade). Transporte vira **assíncrono via IPFS**: cada device publica quando muda algo, os outros puxam quando abrem — sem exigir dois devices online ao mesmo tempo.
- **Achado-chave**: o TruthID já tem o mecanismo que esse sync precisaria — `VaultRegistry.sol` (`identityId → {cid, contentHash, version}`), blob cifrado (AES-256-GCM, chave via HKDF, nunca sai do device) publicado no IPFS com múltiplos provedores de pinning (health-checked), já testado ponta a ponta em hardware real (TruthID, Sessão 90 — celular físico + Ledger + Base Mainnet). **Mas** hoje é 1 vault por identidade, dedicado ao password manager (Fase 13 do TruthID) — reaproveitar pro Practice Valuation exigiria generalizar (`identityId + vaultKind/appId → VaultRef`) ou um contrato irmão no mesmo padrão. Nenhuma das duas escolhida ainda.
- O modelo de dados do Practice Valuation já ajuda bastante aqui: `valuation` e `alert_event` já são append-only ("nunca sobrescreve", ver Fase 1/5.2) — formato natural pra um log de eventos com merge por replay causal entre dispositivos, sem precisar desenhar CRDT do zero.
- **Login/pareamento de device**: reaproveitar o fluxo de QR do TruthID em vez de inventar conta própria. Duas opções discutidas — servidor HTTP local na mesma rede (usuário precisaria aceitar exigir mesma Wi-Fi na hora de conectar) vs. polling puro on-chain (não depende de rede local, mas mais lento). Usuário preferiu a primeira, mas com uma ressalva de segurança confirmada no código do TruthID: o callback tem que continuar `https://` quando presente — não dá pra abrir uma exceção pra `http://` na LAN sem reabrir o risco que a checagem atual existe pra evitar (QR malicioso redirecionando a resposta assinada pro servidor de um atacante, ver `approval_screen.dart:88-96` no TruthID).
- Também discutido: o TruthID poderia oferecer os dois canais de login ao mesmo tempo (POST HTTPS preferencial + fallback on-chain), pra atender integradores sem backend público como o Practice Valuation. Confirmado como viável e barato de fazer — a escrita da sessão on-chain **já acontece hoje incondicionalmente** em todo login do TruthID (via `SessionCreator`/UserOperation, antes até do POST); só falta o TruthID tornar `callbackUrl` opcional no payload do QR.

**Em aberto, sem decisão**:
- Reaproveitar `VaultRegistry` generalizado vs. contrato irmão dedicado no TruthID
- Servidor local (mesma rede) vs. reconsiderar chain-only pro pareamento de device
- Se o TruthID vai mesmo ganhar o modo dual-channel de login, ou isso fica só documentado como possibilidade
- Criptografia dos dados do Practice Valuation antes de subir pro IPFS — provavelmente reaproveita o mesmo padrão do Vault (chave derivada do device key, nunca sai do device), não aprofundado ainda

**Próximo passo, quando o usuário voltar a este tópico**: nenhum. Fica atrás de Fase 5.3 e Fase 7 na fila — ritmo do projeto é lento/de fundo (ver "Ritmo e Expectativa do Projeto"), e essa feature em particular também depende de decisões do lado do TruthID.

---

## Decisões de Arquitetura em Aberto

| Decisão | Opções | Status |
|---|---|---|
| Nome do projeto | — | **Practice Valuation** (`practice-valuation`) ✓ — decidido na Sessão 1 |
| Framework do app desktop | Python (PySide6/Qt, Flet, etc.) vs Tauri (Rust+React/TS) vs Electron vs Flutter Desktop | **Tauri + Rust + React/TypeScript** ✓ — decidido na Sessão 1. Motivo: reaproveita o padrão já validado no TruthID (keyring do SO, empacotamento multi-plataforma via GitHub Actions), e React/TS tem ecossistema forte pra UI densa em dados (tabelas, gráficos) |
| Banco de dados local | SQLite vs DuckDB | **SQLite** ✓ — decidido na Sessão 1 junto com a decisão de stack híbrida (precisa ser lido/escrito tanto pelo Rust quanto pelo Python; SQLite é o padrão pra app local com escrita transacional simples de um arquivo só; DuckDB brilha em análise pesada sobre muita linha, não é o caso aqui) |
| Onde roda a coleta de dados | Dentro do app (Rust) vs. processo separado em Python (herdado do desenho original, ver Fase 2) | **Processo separado em Python**, escrevendo no mesmo SQLite ✓ — decidido na Sessão 1. Motivo: evita reescrever em Rust o parsing de CVM/pandas e a extração de PDF, que já foram desenhados em Python e têm bibliotecas maduras lá (pandas, pdfplumber) — Rust ainda não tem equivalente tão bom |
| Driver/ORM SQLite (Rust) | `rusqlite` (SQL cru) vs SeaORM (Active Record, assíncrono) vs Diesel (ORM maduro, macro-pesado) | **SeaORM** ✓ — decidido na Sessão 1 (revisado — a decisão original era `rusqlite`, mudou quando o usuário revelou que já tem hábito de ORM). Motivo: imita o modelo mental de Django/SQLAlchemy/ActiveRecord que o usuário já conhece, reduzindo a quantidade de Rust+SQL novo pra aprender de uma vez. Assíncrono, mas sem custo real já que o Tauri roda sobre `tokio` de qualquer forma |
| Tratamento de erro (Rust) | `thiserror` (enum próprio, serializável) vs `anyhow` (genérico/dinâmico) | **`thiserror`** ✓ — decidido na Sessão 1. Erros viram JSON estruturado (`{code, message}`) pro React, em vez de string solta — importa pra "fácil manutenção" mesmo num projeto pessoal |
| Busca de dados no React | TanStack Query vs `useState`/`useEffect` na mão | **TanStack Query** ✓ — decidido na Sessão 1. Mesma família do TanStack Table (Fase 0.4); evita repetir controle de loading/erro/cache em cada tela |
| Caminho físico do arquivo SQLite (dev) | Dentro de `desktop/` vs `data-collector/` vs pasta `data/` própria | **`data-collector/practice_valuation.db`** ✓ — decidido na Sessão 1. Rust e Python já rodam no mesmo container, então só precisam apontar pro mesmo arquivo — sem API/rede entre eles. Caminho de produção (fora do Docker, pasta de dados do SO) fica pra Fase 6 |
| Forma de guardar premissas/resultados de valuation | JSON genérico vs 7 tabelas auto-contidas vs `valuation` compartilhada + inputs por modelo | **`valuation` compartilhada + tabela de inputs tipada por modelo** ✓ — decidido na Sessão 1, em duas etapas: primeiro trocou JSON por tabelas tipadas (spec chegou com campos conhecidos/estáveis, valida melhor as guardas `WACC−g`/`Ke vs g`), depois o usuário pediu revisão pra evitar repetir os campos comuns (`ticker`/`ano_ref`/`preco_justo`) em 7 tabelas — extraído pra uma tabela só, o que também simplifica a tela de listagem (Fase 4.1) |
| Sync entre dispositivos/nuvem | Adiado — ver Fase 8 | Não é MVP — ideia desenhada na Sessão 11 (descentralizado, via TruthID + IPFS), nada implementado |
| Densidade visual | Denso (planilha) vs meio-termo vs arejado (dashboard) | **Arejado, tipo dashboard** ✓ — decidido na Sessão 1 |
| Biblioteca de tabela/grid | AG Grid Community vs Glide Data Grid vs TanStack Table + shadcn/ui | **TanStack Table + shadcn/ui** ✓ — decidido na Sessão 1. Motivo: headless, visual 100% customizável e consistente com o resto do app (mesma base do shadcn/ui), em troca de implementar edição/filtro na mão em vez de ganhar pronto |
| Sistema de componentes | shadcn/ui vs Mantine vs Ant Design | **shadcn/ui** (Radix + Tailwind) ✓ — decidido na Sessão 1. Componentes copiados pro repo, visual moderno/neutro, fácil de customizar |
| Biblioteca de gráfico | Recharts vs lightweight-charts (TradingView) vs outra | Pendente — avaliar na Fase 4.3 |
| Navegação entre os 7 modelos de valuation | Seletor de modelo numa tela só vs rota própria por modelo (react-router) | **Seletor numa tela só** ✓ — decidido na Sessão 3. Dropdown troca o formulário exibido, sem roteador; mais rápido de replicar a cada novo modelo, revisitar se a navegação ficar densa demais |
| Gatilho da coleta de dados | Botão manual vs cron/scheduler periódico | **Botão manual** ✓ — decidido na Sessão 1. Rust dispara o Python como subprocesso, sem periodicidade automática por enquanto |
| Ambiente de desenvolvimento | Instalar tudo no host vs Docker | **Docker** ✓ — decidido na Sessão 1, mesmo padrão do TruthID (container único com Node+Rust+WebKitGTK+Python), sem precisar instalar nada na máquina |

---

## Roadmap de Evoluções Planejadas

- **Sync entre máquinas/nuvem**: hoje o banco é 100% local; desenho descentralizado via TruthID + IPFS discutido na Sessão 11, ver Fase 8 em "Fases Detalhadas"
- **Mais indicadores de cripto pagos** (Glassnode/CryptoQuant — MVRV, Puell, Netflow) se o usuário decidir assinar
- **Companion mobile** — só se fizer sentido depois do desktop estar redondo
- **Mais metodologias de valuation** conforme o usuário for trazendo (Bazin/preço-teto, Graham, DCF, EV/EBITDA setorial, etc.)

---

## Log de Sessões

### 2026-07-08 — Sessão 1

- Decisão: abandonar a rota de planilha/Google Sheets como destino final (o spec `docs/spec_automacao_dados.md` já escrito é mantido como referência de fontes de dados, não como plano de execução)
- Decisão: construir um app desktop simples, com banco de dados local, evoluindo aos poucos
- Nome do projeto escolhido: **Practice Valuation** (repo: `practice-valuation`)
- Criado o `PROJECT_STATE.md` (este arquivo), modelado a partir do `PROJECT_STATE.md` do projeto TruthID
- Escopo inicial levantado: múltiplos preços-teto salvos por ativo com premissas diferentes, cripto com premissas + monitoramento de indicadores, alerta de zona de compra
- Pendente pro usuário: trazer a lista de metodologias/fórmulas de preço-teto desejadas (ações e cripto)
- Repo público criado no GitHub (`github.com/masterlxz/practice-valuation`), remote conectado, `.gitignore` de segurança criado, primeiro commit (`PROJECT_STATE.md` + `docs/`) feito e pushado
- **Continuação da Sessão 1**: decidida a stack — **Tauri + Rust + React/TypeScript** pro app (mesmo padrão do TruthID) + **coleta de dados em Python** (reaproveita o desenho do `docs/spec_automacao_dados.md`) + **SQLite** como banco local compartilhado entre os dois. Trade-off discutido: reescrever a coleta em Rust custaria abrir mão de pandas/pdfplumber sem ganho real, já que a UI é a parte que se beneficia do React/TS, não a coleta
- **Continuação da Sessão 1 (direção visual/UI, Fase 0.4)**: decidido **Tailwind CSS + shadcn/ui (Radix) + TanStack Table**, com visual **arejado tipo dashboard** — não denso tipo planilha, apesar da ideia original de "planilha" (isso ficou reservado pro comportamento/dado — múltiplos cálculos salvos, edição manual — não pra densidade visual da tela)
- **Continuação da Sessão 1 (Fase 2.1 + ambiente de dev)**: decidido que a coleta de dados roda sob demanda, via **botão manual** na UI (sem cron) — o Rust dispara o script Python como subprocesso assíncrono, e o feedback na tela é spinner + resumo final (sem log ao vivo por enquanto). Ponto de atenção levantado pelo usuário: **evitar spam de clique** — precisa desabilitar o botão no frontend e ter uma trava no Rust (mutex/flag) pra recusar uma segunda execução concorrente. Decidido também: ambiente de desenvolvimento via **Docker**, mesmo padrão do TruthID (container com Node+Rust+WebKitGTK+Python, X11 repassado, sem instalar nada no host)
- **Continuação da Sessão 1 (Fase 0.5 — estrutura real do repositório)**:
  - Pasta local renomeada de `investments` pra `practice-valuation` (pelo usuário, em paralelo à sessão) — refletido neste arquivo e nos exemplos de path
  - `desktop/` criado via `create-tauri-app` (template `react-ts`, manager `npm`), renomeado internamente pra `practice-valuation` (`package.json`, `Cargo.toml`, `tauri.conf.json`, `index.html`, `main.rs`)
  - Tailwind v4 plugado (`@tailwindcss/vite`, `src/index.css` com `@import "tailwindcss";`) — shadcn/ui e TanStack Table ficam pra quando a Fase 4 começar a construir telas de verdade
  - `data-collector/` criado só com `README.md` + `requirements.txt` vazio (placeholder — implementação real é Fase 2.2)
  - `desktop/Dockerfile`, `docker-compose.yml`, `dev.sh` criados (ver "Ambiente de Desenvolvimento")
  - **Incidente**: o disco (32GB, compartilhado com o TruthID) chegou a 100% de uso (0 disponível) durante o build — causou a perda dos 3 arquivos de Docker recém-criados no meio da renomeação da pasta (recriados na sequência, sem perda de mais nada). Limpeza segura rodada (`docker image prune -f`, só imagens órfãs — nada do TruthID foi tocado), liberou 5.8GB
  - **Achado**: `docker-compose.yml` sem `name:` explícito colidiria com o projeto Compose `desktop` do TruthID (mesma pasta `desktop/` nos dois repos) — corrigido com `name: practice-valuation` na primeira linha do arquivo
  - **`docker compose build` FALHOU** (não confundir com "ainda não testado"): o monitor de segurança abortou o build sozinho quando o disco chegou a ~462MB livres, exatamente como planejado — nenhuma imagem `practice-valuation-desktop` foi gerada. O build tinha acabado de terminar a instalação do Rust (etapa 3 de 7 do Dockerfile) quando foi morto; a etapa seguinte (Python3/pip) nem começou
  - Depois do abort, disco ficou em **1.5GB livres (96% de uso)** — pior do que antes de tentar (tínhamos 4.9GB livres depois da 1ª limpeza). `docker image prune -f` rodado de novo não liberou nada a mais (`0B`); sobraram 2 containers parados do próprio build interrompido (`quizzical_bell`, `adoring_booth`) que dá pra remover com segurança, mas a remoção foi **bloqueada pelo classificador de segurança do modo automático** (ação irreversível sem o usuário por perto pra confirmar) — ficou pendente, não é do TruthID, só limpar quando o usuário estiver presente
  - **Conclusão**: os ~4.9GB livres que sobraram da limpeza segura (só imagens órfãs) não foram suficientes pra esse build.
  - **Resolvido, com autorização do usuário**: removida a imagem `mobile-flutter` (5.94GB) + volumes `mobile_gradle_cache` e `mobile_android_ndk` (~7.6GB) do TruthID — liberou ~13GB (disco foi de 1.5GB → 6.9GB livres). Também removidos os 2 containers órfãos do build falhado (`quizzical_bell`, `adoring_booth`), com o usuário presente — mais ~3GB (→ 9.9GB livres). **Trade-off aceito**: o próximo `docker compose up` do mobile do TruthID vai reconstruir Flutter/Gradle/NDK do zero (mais lento na próxima vez que o usuário voltar pro TruthID mobile)
  - `docker compose build` do `desktop/` disparado de novo, com o mesmo monitor de segurança de disco — **dessa vez terminou com sucesso** (`practice-valuation-desktop:latest`, 4.35GB, disco estável em 8.1GB livres)
  - **`docker compose up` (smoke test real) — mais dois problemas encontrados e corrigidos antes de funcionar**:
    1. `npm install` travava indefinidamente (sem erro, sem progresso) tentando alcançar o registro do npm — causa: **sem rota de saída IPv6** nesse ambiente, e o Node por padrão tenta IPv6 antes de cair pro IPv4, esperando o timeout de TCP (minutos) em vez de falhar rápido. Corrigido com `NODE_OPTIONS=--dns-result-order=ipv4first` no `docker-compose.yml` (`environment:`)
    2. Depois desse fix, travava de novo especificamente na etapa de `npm audit` (uma chamada de rede separada, que não respeitava o mesmo fix). Corrigido pulando essa etapa: `command: npm install --no-audit --no-fund && npm run tauri dev`
    3. Como o `node_modules` já tinha passado por várias tentativas interrompidas (kills no meio da instalação, testando os fixes acima), ficou num estado inconsistente — erro `ENOTEMPTY` do npm tentando renomear uma pasta temporária (`vite` → `.vite-XXXX`). Resolvido apagando `node_modules` por completo (via container, já que os arquivos eram donos de `root`) e reinstalando do zero
  - **Resultado final: o app abre de verdade.** `docker compose up` (ou `./desktop/dev.sh`) sobe o container, `npm install` + `cargo build` rodam dentro dele, e a janela do Tauri aparece na tela do usuário via X11 — confirmado visualmente pelo usuário. Smoke test da Fase 0.5/4 considerado **concluído**
  - **Testado pelo usuário**: botão "Greet" da janela funciona — comunicação React↔Rust via `invoke()` confirmada na prática
- **Continuação da Sessão 1 (Fase 1 — início da conversa sobre o SQLite)**: explicado como Rust e Python vão acessar o mesmo banco (mesmo container, mesmo arquivo, sem API entre eles). Decidido inicialmente: driver `rusqlite` no Rust, modo **WAL** ligado por padrão (Rust e Python são processos diferentes tocando o mesmo arquivo), e o arquivo físico mora em **`data-collector/practice_valuation.db`** (dev)
- **Continuação da Sessão 1 (revisão do driver Rust)**: usuário revelou que já tem hábito de pensar em ORM (não é o mesmo que "iniciante em Rust/banco do zero") — isso mudou a escolha de `rusqlite` pra **SeaORM** (estilo Active Record, familiar pra quem vem de Django/SQLAlchemy/ActiveRecord). Diesel foi descartado por ter sintaxe de query mais macro-pesada. Pedido explícito do usuário: **ir com calma explicando o código Rust**, já que ele não tem muita intimidade com a linguagem — reforça a "Diretriz de ensino" do topo deste arquivo, agora com ênfase específica em Rust (não só em decisões de arquitetura)
- **Continuação da Sessão 1 (chegada do spec funcional — desbloqueia Fase 1 e Fase 3)**: usuário trouxe um documento com os 7 modelos de valuation de ação (DCF/FCFF, Gordon/DDM, Bazin, Graham, Bancos, RNAV, Preço Teto Projetivo — cada um com inputs, fórmula e guarda de erro) e o score cripto de 9 indicadores (verde/vermelho, automatizáveis vs fallback manual pago). Isso mudou a decisão anterior sobre o schema: em vez de `assumption_set` genérico em JSON, decidido **uma tabela por modelo** (7 tabelas) — os campos agora são conhecidos e estáveis, então colunas tipadas validam melhor as guardas de erro que um JSON genérico
- **Continuação da Sessão 1 (consolidação em arquivo único)**: usuário pediu pra juntar tudo num arquivo só, bem organizado por seções, em vez de espalhar entre `PROJECT_STATE.md` + 2 arquivos em `docs/`. As duas specs (`spec_automacao_dados.md` e `spec_funcional_valuation_e_cripto.md`) foram incorporadas na íntegra dentro das Fases 2 e 3 (estrutura de módulos, mecânica da CVM, as 7 fórmulas completas, a tabela cheia do score cripto) e os arquivos removidos do repositório — o histórico deles continua acessível via `git log`/`git show` se precisar. A pasta `docs/` deixou de existir
- **Continuação da Sessão 1 (arquitetura de código)**: usuário pediu decisões de arquitetura parecidas com a do SeaORM, pensando em manutenção de longo prazo mesmo sendo projeto pessoal. Criada a seção "Arquitetura de Código": camadas no Rust (`commands/` → `domain/` → repository SeaORM), tratamento de erro com **`thiserror`** (erro estruturado, serializável pro React) em vez de `anyhow`, e **TanStack Query** no React pra chamar os comandos Tauri (mesma família do TanStack Table já escolhido)
- **Continuação da Sessão 1 (sea-orm-cli instalado)**: `sea-orm` adicionado ao `Cargo.toml` (`sqlx-sqlite`, `runtime-tokio-rustls`, `macros` — versão 1.1.20 confirmada contra o registro real, `cargo check` passou). `sea-orm-cli` (ferramenta de dev, não dependência do projeto) instalado direto no Dockerfile — reconstruído e confirmado (`sea-orm-cli 1.1.20`, mesma versão da lib)
- **Continuação da Sessão 1 (reconsideração de framework — Tauri vs Electron vs Python puro)**: usuário questionou se Tauri era mesmo a melhor escolha, dado o atrito de aprender Rust. Explicado que o Tauri exige Rust por natureza (não tem como usar sem escrever Rust); as alternativas reais seriam Electron (mesmo frontend React/TS, mas backend em Node/TS — sem Rust) ou um app 100% Python (PySide6/Flet, reaproveitando SQLAlchemy). Decisão: **manter Tauri + Rust** — o app já funciona, o SeaORM já resolveu o maior atrito (ORM familiar), e o ritmo do projeto é lento de propósito, então o custo de aprender Rust aos poucos é aceitável
- **Continuação da Sessão 1 (revisão do schema de valuation)**: ver "Mudança de abordagem #2" na Fase 1 — schema reformulado de "7 tabelas auto-contidas" pra "`valuation` compartilhada + tabela de inputs por modelo", por pedido do usuário pensando em manutenção de longo prazo
- **Continuação da Sessão 1 (início da fatia vertical do Bazin)**: criada a pasta `migration/` (`sea-orm-cli migrate init`), ajustada pra usar `tokio` em vez de `async-std` (consistência com o resto do projeto). Escrita a primeira migration de verdade (`create_valuation_and_bazin_inputs`): cria `valuation` (campos comuns, nullable onde a guarda pode impedir o cálculo) + `bazin_inputs` (dividendo médio, yield desejado) com FK cascata de volta pra `valuation`. **`cargo check` passou de primeira** — os helpers do schema builder (`pk_auto`, `string`, `integer`, `double`, `*_null`, `foreign_key`) usados corretamente. Nota técnica: arquivos criados pelo `sea-orm-cli` de dentro do container nascem com dono `root` — precisa de `chown -R 1000:1000` depois de cada geração (`migrate init`, `migrate generate`, e futuramente `generate entity`) antes de editar pelo host
- Usuário questionou se Tauri era mesmo a melhor escolha framework-wise — ver entrada acima (reconsideração Tauri vs Electron vs Python), decisão foi manter
- Sessão encerrada aqui a pedido do usuário — clima bom, sessão longa e produtiva. Migration escrita e validada, mas **ainda não rodada** (o arquivo `.db` ainda não existe fisicamente)
- **Sessão 2 (migration rodada + entities geradas)**: `sea-orm-cli migrate up` executado (`docker compose run --rm -w /app/src-tauri desktop sea-orm-cli migrate up -u sqlite:///data-collector/practice_valuation.db?mode=rwc`) — `data-collector/practice_valuation.db` existe fisicamente pela primeira vez, com `valuation` e `bazin_inputs`. Rodar de dentro de `migration/` falha (`sea-orm-cli` espera o `Cargo.toml` relativo a `src-tauri/`, não ao próprio crate `migration/`) — rodar sempre a partir de `src-tauri/`. Em seguida `sea-orm-cli generate entity -u ... -o src/entity --with-serde both` gerou `src/entity/{valuation,bazin_inputs,mod,prelude}.rs`. `chown -R 1000:1000` necessário depois dos dois comandos (mesma nota técnica da Sessão 1)
- **Sessão 2 (fatia vertical do Bazin fechada)**: criadas as camadas decididas em "Arquitetura de Código" — `src/error.rs` (`AppError` via `thiserror`, `Serialize` manual pra virar `{ code, message }` no frontend), `src/db.rs` (conecta no SQLite compartilhado, path hardcoded de dev — path de produção fica pra Fase 6), `src/domain/bazin.rs` (função pura `calculate()`, guarda `desired_yield <= 0.0`, 3 testes unitários passando), `src/commands/bazin.rs` (`calculate_bazin`, comando fino: chama `domain::bazin::calculate` e persiste `valuation` + `bazin_inputs` via SeaORM). `lib.rs` reescrito: conecta o banco no `run()` via `tauri::async_runtime::block_on`, gerencia como `tauri::State`, registra o comando — boilerplate `greet` removido (não usado mais). **Achado sobre `tauri::generate_handler!`**: o macro exige o caminho até o módulo onde a função `#[tauri::command]` foi *definida* (`commands::bazin::calculate_bazin`), não um re-export (`pub use` em `commands::calculate_bazin` quebra o macro, que procura itens ocultos gerados ao lado da definição original)
- **Sessão 2 (tela React)**: `@tanstack/react-query` instalado (`npm install` dentro do container). `main.tsx` ganhou `QueryClientProvider`; `App.tsx` virou um formulário real (ticker, ano, preço atual, dividendo médio, yield desejado) usando `useMutation` pra chamar `invoke("calculate_bazin", { request })`, mostrando preço-teto/margem/veredito ou o erro. Boilerplate do template (`App.css`, `assets/react.svg`, botão "Greet") removido por não ter mais uso
- **Sessão 2 (correção — diretriz de inglês)**: o formulário e a mensagem de `AppError::InvalidGuard` foram escritos em português por engano — a "Diretriz de código" no topo deste arquivo exige inglês em qualquer string visível ao usuário. Corrigido num commit separado logo em seguida
- **Sessão 2 (smoke test real)**: `docker compose up` (mesmo fluxo do `dev.sh`) rodado em background, build do zero (~20s de compilação Rust incremental sobre o cache já quente), janela do Tauri abriu e o usuário confirmou visualmente o formulário calculando corretamente. Avisos de Mesa/GL (`Failed to query drm device`, `libGL error: failed to load driver: iris`) aparecem no log mas não impedem o app de abrir — o `WEBKIT_DISABLE_DMABUF_RENDERER`/`WEBKIT_DISABLE_COMPOSITING_MODE` do `docker-compose.yml` (Sessão 1) já cobre isso. Container parado e removido (`docker compose down`) ao final, ownership dos artefatos gerados corrigida

### 2026-07-09 — Sessão 3

- **Navegação dos 7 modelos (pendência #2 da Sessão 2)**: decidido **seletor de modelo numa tela só** (dropdown trocando o formulário exibido), sem roteador (`react-router`) por enquanto — mais rápido de construir e replicar a cada novo modelo; rota própria por modelo fica pra quando/se a navegação ficar densa demais
- **Fatia vertical do Graham fechada, replicando o padrão do Bazin ponta a ponta**:
  - Migration `create_graham_inputs` gerada via `sea-orm-cli migrate generate` (mantém o padrão de timestamp automático) e editada à mão — tabela `graham_inputs` (`eps`, `book_value_per_share`) com FK cascata pra `valuation`, mesmo molde da `bazin_inputs`
  - `migrate up` + `generate entity` rodados — `chown -R 1000:1000` precisou ser feito via `docker compose run ... chown` (o host não tem permissão de tocar arquivo dono de `root` escrito pelo container mesmo sendo o mesmo UID numérico — resolvido chamando o `chown` de dentro do container, que tem esse privilégio)
  - `src/domain/graham.rs`: função pura `calculate()`, guarda `eps <= 0.0 || book_value_per_share <= 0.0`, fórmula `sqrt(22.5 × eps × book_value_per_share)`, mesmo padrão de veredito Barato/Caro do Bazin (margem de segurança = preço-justo vs preço atual) — 4 testes unitários passando
  - `src/commands/graham.rs`: comando fino `calculate_graham`, mesmo molde do `calculate_bazin` (persiste `valuation` + `graham_inputs`)
  - `domain/mod.rs`, `commands/mod.rs`, `lib.rs` atualizados para registrar o novo módulo/comando
  - `cargo check` e `cargo test --lib domain::graham` passando (4/4)
- **Refatoração do React pro seletor de modelo**: `App.tsx` virou um shell fino (dropdown + renderiza o form escolhido); lógica de formulário movida pra `src/models/BazinForm.tsx` e `src/models/GrahamForm.tsx`; extraído `src/types.ts` (tipos `ValuationModel`/`AppError` compartilhados, já que agora tem 2+ consumidores) e `src/components/ValuationResult.tsx` (bloco de resultado/erro, idêntico nos dois forms — evita duplicar esse JSX a cada novo modelo)
- `npx tsc --noEmit` limpo, sem erros de tipo
- **Smoke test real**: `docker compose up` rodado em background, build terminou (~19s incremental), janela do Tauri abriu com o seletor Bazin/Graham no topo — **usuário confirmou visualmente que os dois modelos calculam corretamente**. Container derrubado (`docker compose down`) ao final, ownership dos artefatos gerados (migration + entities) corrigida antes do commit
- **Nota (ainda na Sessão 3)**: usuário perguntou quando entra a passada visual de verdade — registrado na Fase 4 (ver nota lá) que os formulários atuais são rascunho intencionalmente cru, e a Fase 4 só começa depois da Fase 3 completa (todos os 7 modelos + cripto)
- **Fatia vertical do Gordon/DDM fechada, mesmo padrão do Graham**: migration `create_gordon_inputs` (`current_dividend`/D0, `expected_growth`/g, `ke`) com FK cascata pra `valuation`; `src/domain/gordon.rs` com guarda `ke <= expected_growth` (fórmula `D1 = D0×(1+g)`, `Preço_Justo = D1/(Ke−g)`), 4 testes unitários passando; `src/commands/gordon.rs` (`calculate_gordon`); registrado em `domain/mod.rs`, `commands/mod.rs`, `lib.rs`; `src/models/GordonForm.tsx` criado e adicionado ao seletor em `App.tsx`. `cargo check`, `cargo test --lib domain::gordon` (4/4) e `tsc --noEmit` passando. Smoke test real rodado (3 modelos no seletor) — **usuário confirmou visualmente que os três calculam corretamente**
- **Fatia vertical do DCF/FCFF fechada — planejada com /plan antes de implementar** (modelo mais complexo dos 7, 13 inputs em vez de 2-3): migration `create_dcf_inputs` com FK cascata pra `valuation`; `src/domain/dcf.rs` implementa os 6 passos encadeados (FCFF → Ke via CAPM → E → WACC → Valor da Firma → Valor do Equity → Preço Justo), guarda `WACC − g <= 0`, 3 testes unitários (barato, caro, guarda disparada via beta negativo derrubando o Ke) passando; `src/commands/dcf.rs` (`calculate_dcf`); registrado em `domain/mod.rs`, `commands/mod.rs`, `lib.rs`; `src/models/DcfForm.tsx` criado com os 13 campos agrupados em 3 blocos visuais (Operational / Capital structure / Cost of capital, só `<h2>` + espaçamento, sem componente novo) e adicionado ao seletor em `App.tsx`. **Decisão**: "Receita Líquida", listada na spec como input mas não usada em nenhuma fórmula, ficou de fora do schema/formulário (evita campo morto; fácil adicionar depois se virar métrica de referência). `cargo check`, `cargo test --lib domain::dcf` (3/3) e `tsc --noEmit` passando. Smoke test real rodado (4 modelos no seletor) — **usuário confirmou visualmente que o DCF calcula corretamente**

- **Fatia vertical do Bancos (P/B via ROE-Gordon) fechada — sessão com pedido explícito do usuário pra explicar a estrutura do código Rust arquivo por arquivo**: migration `create_banks_inputs` (`book_value_per_share`/VPA, `roe`, `payout`, `ke`) com FK cascata pra `valuation`; `src/domain/banks.rs` com guarda `ke <= sustainable_growth` (fórmula `g_sustentável = ROE×(1−Payout)`, `P/B_Justo = (ROE−g_sustentável)/(Ke−g_sustentável)`, `Preço_Justo = P/B_Justo×VPA`); `src/commands/banks.rs` (`calculate_banks`); registrado em `domain/mod.rs`, `commands/mod.rs`, `lib.rs`; `src/models/BanksForm.tsx` criado e adicionado ao seletor em `App.tsx`. **Achado no processo**: o primeiro teste unitário "caro" falhou — não por bug no `calculate()`, mas porque o preço de teste escolhido (60.0) coincidia exatamente com o preço justo calculado, dando margem de segurança zero em vez de negativa; corrigido subindo o preço de teste pra 90.0. `cargo check`, `cargo test --lib domain::banks` (3/3) e `tsc --noEmit` passando. Smoke test real rodado (5 modelos no seletor) — **usuário confirmou visualmente que o Bancos calcula corretamente**

- **Fatia vertical do RNAV (incorporadoras) fechada — sessão de explicação do Rust continuou**: migration `create_rnav_inputs` (`landbank`, `inventory_at_market_value`, `net_cash` — pode ser negativo, `shares_outstanding`) com FK cascata pra `valuation`. **Nota**: essa foi a primeira spec dos 7 modelos sem uma "Guarda" explícita no documento original — decidido (sem precisar perguntar, seguindo o padrão já estabelecido no Bazin de proteger o divisor) guardar `shares_outstanding <= 0.0` no `src/domain/rnav.rs`, já que `Nº_Ações` é o denominador de `RNAV/Ação`. Fórmula: `RNAV_Total = Landbank + Estoque + Caixa_Líquido`, `RNAV/Ação = RNAV_Total / Nº_Ações` (substitui `preco_justo` na regra geral, sem mudança de schema). 4 testes unitários passando (incluindo caixa líquido negativo). `src/commands/rnav.rs` (`calculate_rnav`); registrado em `domain/mod.rs`, `commands/mod.rs`, `lib.rs`; `src/models/RnavForm.tsx` criado e adicionado ao seletor. A partir desse modelo o padrão já estava consolidado o bastante pra não precisar mais de explicação linha a linha do Rust — usuário confirmou que já consegue ler os arquivos de domínio sozinho. `cargo check`, `cargo test --lib domain::rnav` (4/4) e `tsc --noEmit` passando. Smoke test real rodado (6 modelos no seletor) — **usuário confirmou visualmente que o RNAV calcula corretamente**

- **Fatia vertical do Preço Teto Projetivo fechada — 7º e último modelo de valuation de ação, Fase 3.2 concluída por completo**: migration `create_projected_ceiling_inputs` (`current_dividend`/D0, `expected_growth`/g, `projection_years`/N, `desired_yield`, `ke`) com FK cascata pra `valuation`. **Primeira vez com input inteiro** entre os 7 modelos: `projection_years` é `integer()`/`i32` (como o `reference_year` da tabela `valuation`), não `double()`/`f64` — e a fórmula usa `powi` (potência com expoente inteiro) em vez de `powf`. Sem guarda explícita na spec (mesma situação do RNAV); guardado `desired_yield <= 0.0` no `src/domain/projected_ceiling.rs`, já que esse modelo é literalmente "Bazin com N anos de projeção e desconto a valor presente" (`Dividendo_Projetado_N = D0×(1+g)^N`, `Preço_Teto_Futuro_N = Dividendo_Projetado_N/Yield_Desejado`, `Preço_Teto_Projetivo = Preço_Teto_Futuro_N/(1+Ke)^N`). 4 testes unitários passando, incluindo um de sanidade (`N=0` colapsa pro Bazin puro). `src/commands/projected_ceiling.rs` (`calculate_projected_ceiling`); registrado em `domain/mod.rs`, `commands/mod.rs`, `lib.rs`; `src/models/ProjectedCeilingForm.tsx` criado e adicionado ao seletor. `cargo check`, `cargo test --lib domain::projected_ceiling` (4/4) e `tsc --noEmit` passando. Smoke test real rodado (**os 7 modelos no seletor de uma vez**) — **usuário confirmou visualmente que todos calculam corretamente**
- **Marco**: Fase 3.2 marcada como concluída no roadmap — os 7 modelos de valuation de ação (Bazin, Graham, Gordon/DDM, DCF/FCFF, Bancos, RNAV, Preço Teto Projetivo) estão fechados ponta a ponta (cálculo → persistência → UI), cada um com guarda de erro, testes unitários e confirmação visual do usuário

### 2026-07-09 — Sessão 4

- **Score cripto (Fase 3.3) fechado ponta a ponta — planejada com `/plan` antes de implementar**, mesmo cuidado usado no DCF por ser estruturalmente diferente dos 7 modelos anteriores (não é "formulário → preço-justo", é "log de leituras ao longo do tempo → placar"). Duas decisões de desenho foram levadas ao usuário antes do schema (respostas já refletidas no código):
  - **Faixa entre os dois limiares de cada indicador → três estados** (verde/neutro/vermelho), em vez de forçar binário — guarda os dois números da spec (ex.: MVRV Z-Score: verde <0, vermelho >7) sem descartar nenhum. O placar continua "quantos estão verdes de 9" (neutro não conta como verde)
  - **Navegação → seção separada**, alternador simples no topo do `App.tsx` ("Valuation" / "Crypto Score"), em vez de somar como 8ª opção no dropdown dos 7 modelos (que é conceitualmente outra coisa — modelo de valuation vs. placar de indicadores)
- **Motor genérico, não 9 módulos separados**: diferente dos 7 modelos de ação (uma tabela de inputs por modelo), aqui os 9 indicadores compartilham a mesma forma (valor vs. dois limiares configuráveis) — então uma tabela de leituras (`crypto_indicators`) + uma tabela de limiares configuráveis (`indicator_thresholds`, semeada pela própria migration com os 9 valores de partida) + uma função de classificação única bastam. **Achado de modelagem**: a direção de cada indicador ("menor é melhor" vs. "maior é melhor") não precisou de uma coluna própria — é inferida comparando os dois limiares (`green_boundary > red_boundary` ⇒ maior é melhor), verificado contra os 9 indicadores da spec
- **Indicadores de tendência (NVT vs. média móvel de 90d, TVL/endereços ativos MoM)**: pra manter o domínio Rust uma função pura sem I/O (mesmo princípio dos 7 modelos), esses indicadores entram como um valor **já normalizado** (ex.: razão NVT/MM90d, variação % mês a mês) em vez do número bruto — quem calcula essa normalização é quem registra a leitura (manual por enquanto, já que a Fase 2/coleta automatizada ainda não existe)
- **Valores-semente dos 9 limiares**: só 3 indicadores (MVRV Z-Score, Puell Multiple, Staking Yield) tinham os dois números exatos na spec original — os outros 6 foram documentados como chute inicial (`nvt_ratio`, `net_issuance`, `tvl_trend`, `active_addresses_trend`, `exchange_netflow`, `fees_vs_emission`), ajustáveis depois direto na tabela `indicator_thresholds` sem precisar de migration nova (a própria spec já avisa que os thresholds são "ponto de partida ajustável, não regra imutável")
- Implementação: migration `create_crypto_score_tables` (duas tabelas, sem FK entre elas — `indicator` é só uma chave de texto compartilhada, não uma referência formal) + seed via SQL bruto (`execute_unprepared`); `src/domain/crypto_score.rs` (`classify()`, guarda de limiares iguais, 7 testes unitários); `src/commands/crypto_indicator.rs` (`record_crypto_indicator`, `list_crypto_indicators`); `src/crypto/indicators.ts` (mapa dos 9 rótulos) + `src/crypto/CryptoScorePanel.tsx` (formulário de registro + tabela "leitura mais recente por indicador" + resumo "Verdes: X/9 (Y logados)", reduzido no client a partir da lista de leituras — sem query de agregação dedicada); `App.tsx` ganhou o alternador de seção
- Novas variantes em `AppError`: `EqualThresholds` e `UnknownIndicator` (mensagem própria, não reaproveitam a de `InvalidGuard`)
- **Achado (não corrigido, fora do escopo desta sessão)**: `AppError::InvalidGuard` serializa sempre a mesma mensagem fixa ("desired yield must be greater than zero"), reaproveitada por engano pelos 7 modelos mesmo quando a guarda é outra (ex.: `WACC − g` no DCF, `LPA/VPA` no Graham) — mensagem enganosa na UI, registrado aqui pra corrigir numa sessão futura, não misturado com o trabalho de hoje
- `cargo check`, `cargo test --lib` (39 testes, todos passando — os 32 anteriores + 7 novos do score cripto) e `npx tsc --noEmit` limpos. Smoke test real rodado (`docker compose up`) — **usuário confirmou visualmente que o alternador de seção e o registro/classificação de leituras funcionam**
- Usuário perguntou se a interface atual era o design final — confirmado que não, é rascunho cru de propósito (mesma nota da Fase 4 desde a Sessão 3); com o score cripto fechado, a Fase 3 está **completa por inteiro** (7 modelos + cripto), então a Fase 4 (interface de verdade — shadcn/ui, navegação real, lista de ativos, telas de análise) é a próxima frente natural
- **Marco**: Fase 3 marcada como completa no "Status Geral" — não sobra nenhum modelo/indicador de valuation pendente de implementar antes da Fase 4

- **Continuação da Sessão 4 (início da Fase 4 — pergunta do usuário puxou o assunto)**: usuário perguntou se dava pra ver os valuations já salvos e confirmou (consultando o `.db` direto, `sqlite3 data-collector/practice_valuation.db "SELECT ... FROM valuation"`) que sim — cada cálculo já é uma linha nova, nada sobrescreve, exatamente como desenhado desde a Fase 1. Isso puxou a decisão de já começar a Fase 4 (interface de verdade) nesta mesma sessão, em vez de esperar a próxima — **planejada com `/plan` antes de implementar** pelo tamanho da mudança (primeira tela "de verdade" do projeto). Duas decisões levadas ao usuário antes de codar:
  - **Navegação lista → detalhe: estado na mão** (`useState`), sem `react-router` — mesmo padrão do alternador de seções, sem necessidade real de URL num app desktop pessoal
  - **Escopo: só as telas novas** — os 7 formulários de cálculo e o painel cripto continuam crus por enquanto; "vestir os formulários de uma vez" (nota da Sessão 3) fica pra uma sessão futura
- **Decisão de engenharia (sem perguntar — não é gosto, é YAGNI)**: não criei a tabela `asset` que a Fase 1 original previa (ticker/tipo/nome cadastrados à parte). A lista de "ativos acompanhados" é derivada dos tickers distintos que já aparecem em `valuation` — resolve o que foi pedido (ver/comparar cálculos já salvos) sem precisar de uma tela de cadastro nova. Uma tabela `asset` registrável faria sentido se um dia o usuário quiser um ticker na lista **antes** de calcular algo pra ele (watchlist) — registrado no Roadmap, não implementado agora
- **shadcn/ui e TanStack Table instalados de verdade pela primeira vez** (decididos na Sessão 1, Fase 0.4, nunca usados até aqui): `npx shadcn@latest init -t vite --base radix -p nova` (precisou primeiro configurar o alias de import `@/*` em `tsconfig.json` + `vite.config.ts`, que o init exige e o projeto ainda não tinha) + componentes `table`, `button`, `badge`, `card`; `@tanstack/react-table` via `npm install`
- **`src/commands/valuation.rs`** (novo, `list_valuations` — só leitura, sem lógica de domínio, não entra em `domain/`) + **`src/valuations/SavedValuationsPanel.tsx`**: tela de lista (tickers agrupados no client — `count`, modelo/veredito/data do cálculo mais recente, aproveitando que `list_valuations` já ordena por `updated_at` desc no backend) e tela de detalhe (todas as linhas daquele ticker, comparação lado a lado — a Fase 3.4 pedida desde a Sessão 3). Tabelas renderizadas com `@tanstack/react-table` (`useReactTable`/`getCoreRowModel`, sem sort/filtro nesta primeira leva) + `Table`/`Badge`/`Card`/`Button` do shadcn. `App.tsx` ganhou a 3ª seção ("Saved Valuations"), com container mais largo (`max-w-4xl`) só quando essa seção está ativa
- **Simplificação assumida**: a tela de detalhe mostra só os campos comuns da `valuation` (preço justo, margem, veredito, data) — não busca as premissas específicas de cada modelo (ex.: qual dividendo médio foi usado naquele Bazin), que exigiria juntar com as 7 tabelas de input conforme o `model` da linha. Registrado como 4.2 parcial, não 4.2 completa
- `cargo check`, `cargo test --lib` (32 testes, nada quebrou) e `npx tsc --noEmit` limpos. Smoke test real rodado (`docker compose up`) — **usuário confirmou visualmente que a lista mostra o BBAS3/Bazin salvo e que o drill-down pro detalhe funciona** ("mt massa deu boa")
- **Marco**: Fase 4.1 concluída, Fase 4.2 parcialmente concluída (falta só o detalhe fino das premissas por modelo) — primeira tela do projeto com visual de verdade (shadcn/ui), não mais rascunho cru

- **Continuação da Sessão 4 (vestir os 7 formulários + painel cripto — nota pendente desde a Sessão 3, feita ainda hoje a pedido do usuário)**: instalados os componentes shadcn que faltavam (`label`, `input`, `select`, `tabs`, além de `table`/`button`/`badge`/`card` já usados na tela de valuations salvos). Dois componentes compartilhados extraídos pra não repetir em 8 arquivos: `src/components/Field.tsx` (par Label+campo, mesmo formato do antigo `<label className="flex flex-col gap-1">`) e `src/components/VerdictBadge.tsx` (extraído de dentro do `SavedValuationsPanel`, agora reusado também pelo `ValuationResult`). Os 7 `models/*Form.tsx` + `ValuationResult.tsx` + `CryptoScorePanel.tsx` reescritos com `Card`/`Field`+`Input`/`Button`/`Select` no lugar do HTML cru; `App.tsx` trocou os botões do alternador de seção por `Tabs` de verdade e o dropdown de modelo por `Select`. Nenhuma mudança de lógica/Rust — só camada visual
- **Identidade visual definida (dark navy + verde)**: usuário pediu uma "personalidade" pro app inspirada no TruthID (que é dark quase preto com accent ciano, ver `truthid/desktop/src/App.css`) — só que verde. Como o shadcn/Tailwind v4 já centraliza toda cor em variáveis CSS (`:root`/`.dark` no `index.css`, geradas no `init`), a troca foi só nesse bloco, sem tocar em nenhum componente: `index.html` ganhou `class="dark"` fixo (sem alternância clara/escura — é a única aparência do app, não um tema opcional) e o bloco `.dark` do `index.css` foi reescrito com a paleta (`--background: #0b0f14`, `--card: #111820`, `--foreground: #e6edf3`, `--primary: #4ade80` — verde na mesma "temperatura" do ciano `#4dd0e1` do TruthID, só com o matiz trocado). Os badges de veredito/sinal (`VerdictBadge`, `SIGNAL_STYLE` do cripto) não foram tocados — já usam classes `dark:` do Tailwind diretamente, que passam a valer sempre agora que `.dark` está sempre presente
- `npx tsc --noEmit` limpo, `cargo check` sem novidade (nenhum arquivo Rust mudou nesta parte). Dois smoke tests reais rodados (`docker compose up`) — **usuário confirmou visualmente o reskin dos formulários/painel cripto** e, em seguida, **confirmou visualmente o tema dark+verde** ("top deu boa")
- **Marco**: Fase 4.3 concluída (painel cripto vestido) e Fase 4.5 (direção visual) ganhou a identidade de cor definida, não só a decisão de biblioteca

- **Continuação da Sessão 4 (4.2 fino — mostrar as premissas por modelo, última pendência de Fase 4 registrada)**: `src/commands/valuation.rs` ganhou `get_valuation_inputs(valuation_id, model)` — casa o `model` da linha com a tabela de input certa (`bazin_inputs`, `graham_inputs`, etc.) e devolve a linha encontrada como `serde_json::Value` (sem precisar de um enum Rust com 7 variantes só pra carregar isso — o front já sabe o formato de cada modelo). Novo `AppError::NotFound(String)`. `src/valuations/inputFields.ts` (novo) mapeia cada modelo pra sua lista de campos (rótulo + formatação `currency`/`percentage`/`number`/`integer`), espelhando os rótulos já usados nos 7 formulários de cálculo. `SavedValuationsPanel.tsx`: a tabela de detalhe ganhou uma coluna "Assumptions" com botão View/Hide por linha, que expande uma linha extra (`AssumptionsRow`, busca sob demanda via `get_valuation_inputs` — só quando expandida, não teria sentido buscar as 7 tabelas de input adiantado pra linhas que talvez nunca sejam abertas)
- **Achado da própria sessão, verificado no código-fonte do macro `#[tauri::command]` do Tauri (`tauri-macros-2.6.3/src/command/wrapper.rs`)**: por padrão os argumentos de comando são convertidos pra `camelCase` do lado Rust→JS (`ArgumentCase::Camel`), então `valuation_id` (Rust) precisa ser chamado como `valuationId` no `invoke()` — confirmado antes de escrever o código, não descoberto por tentativa e erro
- **Achado + correção, pedido do usuário**: a tela "Saved Valuations" ganhou barra de rolagem horizontal — causa raiz era o campo `updated_at` mostrando o timestamp ISO bruto com precisão de nanossegundo (`2026-07-09T14:55:25.487759978+00:00`). Adicionado `formatDateTime()` (formato curto tipo "Jul 9, 2026, 02:55 PM") nas duas tabelas (lista e detalhe) + container da seção alargado de `max-w-4xl` pra `max-w-6xl`
- `cargo check`, `cargo test --lib` (32 testes) e `npx tsc --noEmit` limpos. Múltiplos smoke tests reais rodados — **usuário confirmou visualmente que o botão "View" expande as premissas corretamente** ("deu boa maninho") **e que a barra horizontal sumiu**
- **Marco**: Fase 4.2 concluída por completo — não sobra nenhuma pendência conhecida de Fase 4 além de 4.4 (alertas, que depende da Fase 5) e do reskin visual já feito

- **Continuação da Sessão 4 (ajustes de layout pedidos pelo usuário, depois de ver tudo rodando)**:
  - **"Muito vertical, não preenche a tela"**: os 7 formulários + painel cripto usavam `flex flex-col` (uma coluna só, empilhada) dentro de um container `max-w-md` — em telas com muitos campos (DCF, 13 campos) isso virava uma rolagem longa sem aproveitar a largura da janela. Trocado por `grid grid-cols-1 sm:grid-cols-2` em todos (botão de submit com `sm:col-span-2` pra continuar ocupando a linha toda) + container do `App.tsx` unificado em `max-w-6xl` pra todas as seções (antes só "Saved Valuations" era largo). `Field` ganhou uma prop `className` opcional pra campos que precisam ocupar as duas colunas (ex.: o "Raw value" de descrição longa no painel cripto)
  - **Painel cripto redesenhado como dashboard, não formulário**: usuário apontou que escolher 1 indicador por vez num dropdown e enviar não tinha cara de "tela de análise". Perguntei se "atualizar tudo com 1 botão" significava automatizar a coleta (Fase 2, que não existe) ou um formulário em lote — confirmado que é o formulário em lote. Reescrito: a tabela de 9 indicadores agora tem um campo de valor + fonte cada (`Drafts`, estado por indicador), um único campo de data compartilhado, e um botão **"Update all"** que dispara `record_crypto_indicator` uma vez por indicador preenchido (via `Promise.all` — sem comando novo no Rust, o comando existente já faz tudo que cada linha precisa). Indicadores com o campo vazio são ignorados (atualização parcial permitida)
  - **Placar viesse tabela → grid de KPI tiles**: usuário pediu "dashboard horizontal". Consultado o skill `dataviz` antes de desenhar — a seção "choosing-a-form" confirma que "uma leva de números-manchete" é exatamente o caso de **KPI row de stat tiles**, não uma tabela. Substituído o placar (antes uma `Table` de 9 linhas) por um grid `sm:grid-cols-3` de `IndicatorTile` (label + valor + badge de sinal + data), seguindo o contrato de stat tile do skill (label, value, status) — paleta de cor mantida a que já existia no app (verde/amarelo/vermelho dos badges), não trocada pela paleta padrão do skill (ver `references/palette.md` do skill — é só um ponto de partida pra quem ainda não tem marca definida, e o app já tem a sua)
  - `npx tsc --noEmit` limpo em cada etapa (nenhuma mudança em Rust nesta parte). Três smoke tests reais rodados — **usuário confirmou o grid mais largo, confirmou o "Update all" em lote, e confirmou o grid de KPI tiles** ("melhorou agora top")
- **Marco**: layout geral do app deixou de ser uma coluna estreita só; painel cripto passou de "formulário de log" pra "tela de análise" de verdade, alinhado com o pedido original de "tipo uma planilha" (ver histórico da conversa) — dashboard, não formulário sequencial

- **Continuação da Sessão 4 (corrigido o achado do próprio dia — mensagem genérica de `AppError::InvalidGuard`)**: o enum virou `InvalidGuard(String)` (era `InvalidGuard` sem dado, com a mensagem fixa no `#[error("...")]` do `thiserror`) — cada um dos 7 `domain/*.rs` agora passa sua própria mensagem no `return Err(AppError::InvalidGuard("...".to_string()))` (ex.: Gordon → "Ke must be greater than the expected growth rate", DCF → "WACC must be greater than the perpetuity growth rate", RNAV → "shares outstanding must be greater than zero"). Os testes unitários que verificavam `Err(AppError::InvalidGuard)` viraram `Err(AppError::InvalidGuard(_))` (ignora o conteúdo da mensagem, só confirma a variante). Nenhuma mudança de frontend necessária — `ValuationResult`/os formulários já exibem `error.message` genericamente
- `cargo check`, `cargo test --lib` (32 testes, todos passando) e `npx tsc --noEmit` limpos. Smoke test real rodado — **usuário confirmou visualmente que a mensagem de erro agora é específica por modelo** ("deu boa")
- **Marco**: pendência #2 da lista anterior resolvida — não sobra nenhum item pequeno registrado, só a decisão de direção (Fase 2 vs Fase 5) e README/LICENSE

- **Continuação da Sessão 4 (início da Fase 2 — usuário escolheu começar por aqui "aos poucos", reconhecendo que é mais complexa)**: primeira fatia vertical real de coleta de dados, **planejada com `/plan`** dado que atravessa 3 linguagens (Python, Rust, React) pela primeira vez. Escolhida cotação de ações via brapi como ponto de partida — API mais simples que a CVM (que exige baixar zip anual e fazer parsing de CSV), e resolve uma dor imediata (o "Current price" dos 7 formulários é digitado à mão)
  - **API verificada de verdade antes de codar** (`WebFetch` em brapi.dev/docs, não por memória — o endpoint mudou de shape desde a spec original da Fase 2): `GET /api/v2/stocks/quote?symbols=...`, autenticação via header `Authorization: Bearer` (opcional pra 4 tickers de teste sem conta — PETR4, MGLU3, VALE3, ITUB4 — usados como exemplo fictício no `config.yaml`, não é portfólio real do usuário)
  - **Decisão de schema nova**: cotação não é a mesma coisa que uma linha de `valuation` (que só existe quando um cálculo roda) — criada `stock_quotes` (`ticker`, `price`, `source`, `fetched_at`), série temporal, mesmo padrão de `crypto_indicators` (cada fetch é uma linha nova)
  - **Python** (`data-collector/`): `sources/acoes_brapi.py` (`fetch_quotes`, token opcional via `.env`/`BRAPI_TOKEN`), `main.py` (orquestrador: lê `config.yaml`, chama a fonte, grava no SQLite compartilhado), `requirements.txt` (`requests`, `PyYAML`, `python-dotenv`), `.env.example`. Venv criado e testado manualmente contra a API real (`docker compose run ... .venv/bin/python3 main.py`) **antes** de plugar no botão — confirmou 4 cotações reais gravadas no banco
  - **Rust**: `tokio` virou dependência direta com a feature `process` (antes só existia transitivo via sea-orm/sqlx — declarar explícito é mais seguro). `src/commands/collector.rs`: `run_stock_collector` (trava via `AtomicBool` gerenciado como state — recusa chamada concorrente, mesma trava contra clique duplo decidida desde a Fase 2.1) dispara `tokio::process::Command` no Python do venv e devolve stdout/stderr; `list_stock_quotes` no mesmo molde dos outros `list_*`. Duas variantes novas em `AppError`: `CollectorBusy`, `CollectorFailed`
  - **Frontend**: `src/collector/StockCollectorPanel.tsx` — botão "Run stock collector" + resumo do resultado + tabela da cotação mais recente por ticker (mesmo padrão "reduz no client" já usado 3x nesta sessão). Usuário pediu pra tirar da aba própria e colocar dentro da aba **Valuation** (no topo, antes do seletor de modelo) — mais coerente já que é o "Current price" dali que ele resolve
  - `cargo check`, `cargo test --lib` (32 testes, nada quebrou — collector não tem lógica de domínio, só orquestração) e `npx tsc --noEmit` limpos. Múltiplos smoke tests reais — **usuário confirmou visualmente que o botão busca cotação real da brapi e grava no banco** ("funcinou sim")
  - Usuário perguntou duas vezes se os tickers de exemplo "ficam lá" — reforçado que é só config editável (`data-collector/config.yaml`), não hardcoded, sem risco de virar dado permanente ou vazar portfólio real
- **Marco**: Fase 2 deixou de ser "não iniciada" — primeira fonte de dado real funcionando ponta a ponta (Python → SQLite → botão → UI). Próximas fontes (CVM, bolsai, cripto) seguem o mesmo molde já provado

- **Continuação da Sessão 4 (editar/excluir valuations salvos — pedido do usuário depois de ver a tela de análise funcionando)**: até aqui o schema seguia a regra fixa "nunca sobrescreve, cada cálculo é uma linha nova" (Fase 1). Usuário pediu uma exceção proposital: poder **corrigir** um cálculo salvo (typo num número) ou **remover** um de vez, sem virar uma linha nova a cada correção — **planejado com `/plan`**, verificando antes duas coisas técnicas em vez de assumir:
  - Conferido no código-fonte do `sqlx-sqlite` (driver por baixo do SeaORM) que `foreign_keys = ON` já vem ligado por padrão nas conexões do app — então o `ON DELETE CASCADE` das 7 tabelas de input (configurado desde a primeira migration, Sessão 1/2) já funciona de verdade; excluir uma `valuation` já limpa a tabela de input específica sozinho, sem precisar de fix extra
  - **Desenho**: em vez de 7 comandos de update (um por modelo, espelhando os 7 de criação), **um comando genérico só** — reaproveita a mesma função pura de cálculo de cada modelo (`domain::X::calculate`, sem duplicar lógica) e o mesmo mapa `INPUT_FIELDS` já criado pra tela de "Assumptions" (Sessão 4, mais cedo) pra desenhar **um formulário de edição genérico** (não 7), que se adapta ao modelo da linha sendo editada
  - Os 7 structs de input em `domain/*.rs` ganharam `#[derive(serde::Deserialize)]` (não derivavam nada antes) — único jeito de desserializar o JSON genérico vindo do front de volta pro tipo certo de cada modelo. Novo `AppError::InvalidInput` pra erro de desserialização (distinto de `InvalidGuard`, que é regra de negócio)
  - `update_valuation` e `delete_valuation` em `commands/valuation.rs` (mesmo arquivo de `list_valuations`/`get_valuation_inputs`) — update dá `UPDATE` na linha de `valuation` e na tabela de input certa (filtrando por `valuation_id`, sem precisar buscar o `id` próprio da linha de input antes); delete só remove a `valuation`, cascade cuida do resto
  - `src/valuations/EditValuationForm.tsx` (novo): busca os inputs atuais (reaproveita `get_valuation_inputs`), preenche o formulário, converte de volta no submit. `inputFields.ts` ganhou `toEditableString`/`fromEditableString` (inverso de `formatInputValue` — ex.: percentual mostra "6" pra editar, não "0.06", igual os 7 formulários de cálculo já fazem)
  - `SavedValuationsPanel.tsx`: coluna "Assumptions" virou **"Actions"** com View/Edit/Delete. Delete exige clique duplo (primeiro clique vira "Confirm?", só o segundo executa) — trava manual contra exclusão sem querer, já que é ação sem volta
  - `cargo check`, `cargo test --lib` (32 testes, nada quebrou — a lógica de cálculo em si não mudou) e `npx tsc --noEmit` limpos. Smoke test real — **usuário confirmou visualmente que editar atualiza a mesma linha (não cria nova) e que excluir remove de vez** ("sim deu certo")
- **Marco**: Saved Valuations completo — lista, detalhe/comparação, premissas, edição e exclusão. Fase 4 não tem mais pendência conhecida além de 4.4 (alertas, depende da Fase 5)

### 2026-07-10 — Sessão 5

- **Continuação da Fase 2 (fundamentos via bolsai, usuário escolheu "bolsai, é mais simples que a CVM")**: documentação real verificada via `WebFetch`/`curl` em usebolsai.com/docs (não por memória) antes de codar, mesmo padrão da brapi. Achado logo de cara: bolsai **exige cadastro** (login Google no dashboard pra gerar `X-API-Key`) — diferente da brapi, não tem ticker de teste sem chave. Usuário gerou a chave na hora
- Consultado o usuário sobre escopo antes de decidir sozinho (regra do arquivo): **narrow scope** escolhido — só os campos que os formulários já pedem à mão hoje (LPA/VPA pro Graham, ROE pros Bancos, via `GET /fundamentals/{ticker}`) e o dividendo médio 5 anos do Bazin (via `GET /dividends/{ticker}`) — não a tabela completa de ~27 indicadores
- **Schema**: duas tabelas novas, mesmo molde de `stock_quotes` (série temporal, nunca sobrescrita) — `stock_fundamentals` (`ticker`, `lpa`, `vpa`, `roe`, `source`, `fetched_at`) e `stock_dividends_avg` (`ticker`, `avg_dividend_5y`, `source`, `fetched_at`). Migration gerada via `sea-orm-cli migrate generate`, aplicada (`migrate up`) e entities geradas (`generate entity`), mesmo fluxo já documentado (`chown -R 1000:1000` depois de cada comando gerado dentro do container)
- **Python**: `data-collector/sources/acoes_bolsai.py` — `fetch_fundamentals` (LPA/VPA/ROE) e `fetch_dividends_avg` (média dos últimos 5 anos **completos**, descartando o ano corrente parcial do `annual_summary` pra não puxar a média pra baixo sem motivo). `main.py` passou a chamar as duas depois da cotação, sempre gravando linha nova
- **Achado testando contra a API real** (não coberto pela primeira leitura da doc): `GET /dividends/{ticker}` devolve **403 Forbidden** — é endpoint exclusivo do plano Pro (R$49/mês; a doc marca isso com uma badge "PRO" no menu lateral, só percebida na segunda passada). Apresentadas 3 opções ao usuário (entregar só o que funciona / investigar brapi com token / assinar o Pro) — **escolhida "entregar só o que já funciona"**. `fetch_dividends_avg` trata o 403 como `RuntimeError` (mesmo tratamento já usado pra chave ausente), `main.py` pula essa coleta sem derrubar cotação/fundamentos. Dividendo médio do Bazin **continua manual** até o usuário decidir diferente
- **Rust**: `list_stock_fundamentals`/`list_stock_dividends_avg` em `commands/collector.rs`, mesmo molde de `list_stock_quotes` — sem `AppError` novo, sem mudança em `domain/`
- **Frontend**: `StockCollectorPanel.tsx` ganhou duas tabelas novas (Fundamentals, Average dividend) com o mesmo padrão "última leitura por ticker" já usado pra cotação; texto do card atualizado explicando que bolsai exige `BOLSAI_API_KEY` e que a tabela de dividendo fica vazia até isso mudar
- `cargo check`, `cargo test --lib` (32 testes, nada quebrou) e `npx tsc --noEmit` limpos. Coletor rodado de verdade (`docker compose run ... .venv/bin/python3 main.py`) contra a API real **antes** de plugar no botão — confirmou LPA/VPA/ROE reais gravados pros 4 tickers de config, saída limpa (`exit code 0`) mesmo com o bloqueio de dividendos. App subido via `dev.sh`, smoke test real na tela — **usuário confirmou visualmente que o botão busca os dados e populam as duas tabelas novas** ("deu boa")
- **Marco**: Fase 2 ganha sua segunda fonte ponta a ponta (bolsai, fundamentos). Padrão Python → Rust subprocess → tela seguiu firme pela segunda vez; a lição nova desta sessão foi "ler a doc de novo depois do primeiro 403" — a badge PRO estava lá, só não foi vista na primeira passada

- **Continuação da Sessão 5 (cripto, usuário escolheu "Cripto (CoinGecko, DefiLlama, etc.)")**: fontes verificadas contra a API real antes de propor escopo — CoinGecko e DefiLlama funcionam **sem nenhum cadastro** (testado com `curl`); ultrasound.money não tem API pública documentada (repo antigo no GitHub sugere descontinuada, precisaria investigar scraping); Etherscan e stakingrewards.com exigem chave, como a bolsai. Dos 9 indicadores do score cripto, só **TVL Trend** tinha fonte 100% livre de fricção verificada — escopo consultado com o usuário, **escolhido "só TVL Trend via DefiLlama"** em vez de incluir CoinGecko como dado de referência extra
- **Achado de arquitetura levantado (não é decisão de gosto, é constraint física já decidida — não parou pra perguntar)**: diferente de `stock_quotes`/`stock_fundamentals` (só dado bruto), `crypto_indicators` já guarda o **sinal classificado** junto (`GREEN`/`NEUTRAL`/`RED`), calculado hoje só em Rust (`domain::crypto_score::classify`, via comando `record_crypto_indicator`). Como Python escreve direto no SQLite compartilhado sem IPC com o Rust (arquitetura decidida desde a Fase 0), a única forma de o coletor gravar uma leitura completa é duplicar essa classificação em Python — feito em `_classify_signal` no `main.py`, com comentário apontando de volta pro `crypto_score.rs` pra manter os dois em sync se a regra mudar
- **Python**: `data-collector/sources/cripto_defillama.py` — `fetch_tvl_trend_mom(chain="Ethereum")`, usa `GET /v2/historicalChainTvl/{chain}` (confirmado sem chave), calcula variação % comparando o TVL mais recente com o de ~30 dias atrás (série é diária, confirmado — dá pra contar posições em vez de comparar datas). `main.py` ganhou `collect_crypto_tvl_trend()` (lê o threshold de `indicator_thresholds`, classifica, grava em `crypto_indicators` com `coin="ETH"`, `source="defillama"`) e um modo de execução separado (`python3 main.py crypto`, mantendo o modo padrão só pra ações) — primeira vez que o orquestrador ganha essa ramificação, prevista desde a Fase 2.1 ("roda tudo ou um módulo específico") mas nunca usada até agora
- **Rust**: `run_crypto_collector` novo em `commands/collector.rs`, reaproveitando a mesma trava (`AtomicBool`) e a mesma lógica de subprocess do `run_stock_collector` (extraída pra um helper `run_collector` com args variáveis) — sem tabela nova, sem `AppError` novo
- **Frontend**: botão "Run TVL Trend collector (DefiLlama)" adicionado no topo do `CryptoScorePanel.tsx`, acima do grid de KPI tiles — invalida a query de `crypto-indicators` no sucesso, mesmo padrão dos outros botões de coletor. Ajuste pedido pelo usuário depois de ver rodando: `IndicatorTile` mostrava o valor bruto sem arredondar (ex.: várias casas decimais) — trocado pra `.toFixed(2)`, aplicado a todos os 9 indicadores (não só TVL Trend, já que todos passam pelo mesmo componente)
- `cargo check`, `cargo test --lib` (32 testes, nada quebrou) e `npx tsc --noEmit` limpos em cada etapa. Coletor rodado de verdade (`python3 main.py crypto`) antes de plugar no botão — confirmou TVL Trend real (~9%, GREEN) gravado no banco. App já estava rodando via `dev.sh`; Rust recompilou e Vite deu HMR sem precisar reiniciar o container. Dois smoke tests reais — **usuário confirmou o botão populando o tile** ("deu certo") **e depois confirmou o arredondamento pra 2 casas** ("deu boa")
- **Marco**: Fase 2 dá seu primeiro passo em cripto — 1 dos 9 indicadores automatizado ponta a ponta (TVL Trend via DefiLlama), sem exigir nenhum cadastro. Os outros 8 seguem manuais (3 sem fonte gratuita conhecida — MVRV, Puell, Exchange Netflow — e os demais exigindo cadastro ou investigação, ver etapa 2.3 acima)

- **Continuação da Sessão 5 (tentativa de automatizar `active_addresses_trend` via Etherscan — usuário gerou chave grátis, escolheu investigar)**: `ETHERSCAN_API_KEY` criada no `.env`. Testado contra a API v2 real: a chave funciona (`gastracker` respondeu ok), mas **todo o módulo `stats` de séries históricas diárias é exclusivo do plano Pro** (`dailytx`, `dailynewaddress` — qualquer `daily*` — devolvem "trying to access an API Pro endpoint"), mesmo bloqueio já visto na bolsai. Investigada uma rota alternativa (usuário pediu, em vez de desistir na hora): módulos não-`stats` (`account`, `block`, `proxy`) só dão dado por endereço ou por bloco individual — pra montar "endereços ativos da rede por dia" seria preciso varrer ~7.200 blocos/dia contando endereços únicos, inviável no rate limit do free tier; a alternativa de usar um endereço específico como proxy foi descartada por medir a atividade daquele endereço, não da rede — alimentaria o score com um dado errado, pior que deixar manual. **Nenhum código escrito** (só investigação) — `active_addresses_trend` continua manual, registrado como pendência junto de `net_issuance` e `staking_yield`
- **Marco**: cripto fechada por hoje com 1/9 indicadores automatizados (TVL Trend). Os outros 8 têm um motivo específico documentado pra continuarem manuais (fonte paga, API Pro-only, ou API não documentada) — não é preguiça, é o estado real das fontes gratuitas hoje

- **Continuação da Sessão 5 (`net_issuance`, usuário escolheu investigar em vez de gerar mais uma chave sem garantia)**: pesquisada de novo a API do ultrasound.money — **a suposição da entrada anterior deste log estava errada**. Achado o repositório real (`github.com/ultrasoundmoney/eth-analysis-rs`, Rust/axum) direto no GitHub, lendo `src/serve/mod.rs`, revelando rotas REST públicas de verdade em `/api/v2/fees/*` (não é só WebSocket). Testado `/api/v2/fees/supply-over-time` contra a API real: devolve várias janelas de supply total de ETH (`m5`, `h1`, `d1`, `d7`, `d30`, `since_merge`), sem chave — `d30` já vem cortada nos últimos 30 dias pelo próprio servidor, então dá pra calcular a variação % anualizada direto (primeiro vs. último ponto da janela), sem precisar contar posições como no DefiLlama
- Verificado `staking_yield` (stakingrewards.com) em paralelo: usa GraphQL com chave via cadastro (`api.stakingrewards.com/public/query`), mas não deu pra confirmar se o "real staking yield" específico está no free tier sem criar a conta — **usuário decidiu não arriscar mais um cadastro hoje** (2 de 2 cadastros anteriores na sessão bateram em endpoint pago no meio do caminho); fica pendente pra quando alguém quiser testar
- **Python**: `data-collector/sources/cripto_ultrasound.py` — `fetch_net_issuance_annualized_pct()`. `main.py` ganhou `_record_crypto_indicator(indicator, source, raw_value)` (extraído de `collect_crypto_tvl_trend`, que tinha essa lógica de threshold+classificação+insert hardcoded só pra `tvl_trend` — virou reutilizável agora que existe um segundo indicador igual) e `collect_crypto_net_issuance()`. **Nenhuma mudança em Rust/frontend precisou** — `run_crypto_collector` já roda `main.py crypto` inteiro, e o botão existente já invalida a query que os dois indicadores compartilham; só o rótulo do botão em `CryptoScorePanel.tsx` foi atualizado ("Run crypto collector (TVL Trend, Net Issuance)") pra não ficar desatualizado
- `npx tsc --noEmit` limpo (sem mudança em Rust desta vez). Coletor rodado de verdade (`python3 main.py crypto`) antes de reusar o botão — confirmou Net Issuance real (~0,84%/ano, NEUTRAL, dentro da faixa esperada do threshold placeholder) gravado no banco junto do TVL Trend. Smoke test real — **usuário confirmou o botão existente já buscando os dois indicadores** ("deu boa")
- **Marco**: Fase 2 fecha o dia com 2/9 indicadores cripto automatizados (TVL Trend, Net Issuance), ambos sem cadastro nenhum. Lição da sessão: **não parar de investigar no primeiro "parece que não tem API"** — a leitura inicial do ultrasound.money (baseada só no site/HTML) tinha concluído errado que não existia API pública; o código-fonte real no GitHub mostrou o contrário

- **Continuação da Sessão 5 (CVM pro DCF — usuário pediu primeiro "como vai funcionar?", depois "cria o planejamento de implementação")**: explicado o mecanismo (zip anual, não API REST) antes de planejar, seguindo a diretriz de ensino. Planejado com `/plan`: um agente `Explore` leu o schema exato do `DcfInputs` em Rust e testou ao vivo os CSVs reais da CVM (zip `dfp_cia_aberta_2025.zip`, baixado e inspecionado de verdade) pra achar os códigos de conta certos; um agente `Plan` estruturou a implementação a partir desses achados. Achado central: **nem toda conta é padronizada igual** — EBIT (`3.05`), dívida (`2.01.04`+`2.02.01`) e caixa (`1.01.01`) usam o mesmo código pra ~850 das ~870 empresas, mas D&A e Capex têm código que varia empresa a empresa (a WEG rotula Capex só como "Imobilizado"/"Intangível", sem a palavra "Aquisição" que a maioria usa) — pra essas duas a extração busca por palavra-chave no texto da conta e **devolve `None` em vez de chutar** quando fica ambíguo. Consultado o usuário sobre escopo antes de decidir: os 7 campos de uma vez (5 seguros + D&A/Capex melhor-esforço) vs. só os 5 seguros — **escolhidos os 7**, já que D&A/Capex nascem com a trava de nunca retornar valor errado
- **ΔNWC não vem do fluxo de caixa** (mesmo problema de instabilidade de código do Capex) — é calculado a partir do balanço patrimonial, que traz dois exercícios lado a lado (`ÚLTIMO`/`PENÚLTIMO`): `(Contas a Receber + Estoques − Fornecedores)` este ano menos o mesmo cálculo ano passado, usando só códigos estáveis
- **Achado corrigido durante o teste real, não pelo planejamento** (o motivo de sempre testar contra dado de verdade antes de fechar): o número de ações do arquivo `composicao_capital` da CVM veio 1000x menor que o real pra VALE3 especificamente (4,5 milhões em vez de ~4,5 bilhões — um erro nos próprios dados da CVM pra aquela companhia, não um bug de leitura) — decidido na hora **abandonar essa fonte pro nº de ações** e reaproveitar o `shares_outstanding` que a bolsai já devolve no `/fundamentals` (mesma chamada que já busca LPA/VPA/ROE/cvm_code), conferido como correto pros 4 tickers
- **Segundo bug pego só no teste real, não no plano**: a coleta de dividendos (bloqueio conhecido, 403 Pro-only da bolsai) estava dentro do mesmo `try/except` que a coleta nova do DCF — como o 403 sempre acontece (usuário não assinou o Pro), o `except` interrompia o bloco inteiro antes de chegar no DCF, que nunca rodava. Corrigido isolando cada coleta bolsai-dependente no seu próprio `try/except` — a falha conhecida dos dividendos não bloqueia mais nada depois dela
- **Python**: `data-collector/sources/cvm_dfp.py` (novo) — baixa/cacheia o zip anual (`data-collector/.cache/cvm_dfp/`, novo no `.gitignore`), lê os CSVs com `csv`/`zipfile` da stdlib (decidido não usar pandas — o volume de trabalho real é filtrar poucos tickers de um zip, sem justificar uma dependência nova e pesada na imagem Docker, diferente do que o plano original da Fase 2 supunha), indexa por `CD_CVM` (convertido pra `int` — a bolsai devolve sem zero à esquerda, a CVM devolve com, comparar como texto não bate nunca), descarta linhas de versões antigas (`VERSAO`) de uma retificação. `acoes_bolsai.py::fetch_fundamentals` ganhou `shares_outstanding` no retorno (dado já vinha na resposta, só não estava sendo usado)
- **Schema**: `stock_dcf_fundamentals` (mesmo molde de `stock_fundamentals` — série temporal, nunca sobrescrita), com `depreciation_amortization`/`capex` nullable (`double_null`). Migration gerada, aplicada e entity gerada no mesmo fluxo já repetido a cada modelo desde a Sessão 1
- **Rust**: `list_stock_dcf_fundamentals` em `commands/collector.rs`, mesmo molde dos outros `list_*` — sem `AppError` novo, sem mudança em `domain/dcf.rs` (é dado de referência, não um cálculo novo)
- **Frontend**: quarta tabela em `StockCollectorPanel.tsx` ("DCF fundamentals (CVM)"), mesmo padrão "última leitura por ticker"; D&A/Capex nulos mostram "—"
- `cargo check`, `cargo test --lib` (32 testes, nada quebrou) e `npx tsc --noEmit` limpos. Testado de verdade contra o zip real e os 4 tickers do `config.yaml` **antes** de mexer no Rust — confirmou EBIT/dívida/caixa/ΔNWC reais pra PETR4/MGLU3/VALE3 (batendo com a ordem de grandeza esperada), ITUB4 (banco) corretamente sem retorno (taxonomia de DRE diferente pra financeiras, DCF não é pra banco mesmo). Smoke test real na tela, app já rodando via `dev.sh` — **usuário confirmou a tabela nova populada** ("deu boa")
- **Marco**: Fase 2.2 completa pro que dá pra automatizar de fontes gratuitas — DCF ganha 6 dos 13 campos automatizados nesta etapa (só `tax_rate` entre os contábeis ficou de fora, não pesquisado; os 5 restantes são premissas de mercado que nunca vêm de balanço). Terceira fatia de coleta desta sessão (depois de bolsai e cripto) — o padrão Python→Rust subprocess→tela se provou de novo, e o hábito de "testar contra dado real antes de fechar" pegou dois problemas reais (VALE3, bug de isolamento) que o planejamento sozinho não teria pego

- **Continuação da Sessão 5 (`tax_rate` do DCF, usuário escolheu essa opção pra "fechar o DCF inteiro")**: achado direto na mesma DRE já usada pro EBIT — `3.07` (Resultado Antes dos Tributos sobre o Lucro) e `3.08` (Imposto de Renda e Contribuição Social), ambos ~97% estáveis entre empresas (mesma taxa de acerto do próprio EBIT). `tax_rate = -VL(3.08) / VL(3.07) × 100`
- **Achado testando contra dado real, não previsto no plano**: a MGLU3 real deu uma alíquota de **263%** — resultado antes dos tributos quase zero e negativo (despesas financeiras pesadas apagaram o EBIT positivo), dividir por um número perto de zero explode a razão. Adicionado guard: `pretax_income <= 0` → `None`. Testando a correção, a **VALE3** (mineradora, tributação de subsidiárias no exterior + CFEM) deu **55,75%** — uma alíquota alta mas **real**, não artefato — um teto de sanidade inicial de 50% teria descartado esse valor legítimo por engano. Corrigido pra um teto mais generoso (100%), guiado pelo dado real de duas empresas diferentes, não por uma regra arbitrária definida de antemão
- **Python**: `_effective_tax_rate` em `cvm_dfp.py`, tratado como campo "melhor esforço" (nullable) igual D&A/Capex, mas por um motivo diferente — lá o problema é código de conta instável entre empresas, aqui o código é estável e o problema é matemático (denominador perto de zero)
- **Schema**: `stock_dcf_fundamentals` ganhou coluna `tax_rate` (nullable) via migration de `ALTER TABLE ADD COLUMN` — primeira vez que uma migration desta sessão altera uma tabela existente em vez de criar uma nova
- `cargo check`, `cargo test --lib` (32 testes) e `npx tsc --noEmit` limpos. Testado contra dado real (PETR4 26,6%, VALE3 55,8%, MGLU3 `None`) antes do Rust. Rust recompilou e Vite deu HMR no app já rodando — **usuário confirmou a coluna nova na tela** ("deu boa")
- **Marco**: DCF completo — todas as 8 entradas contábeis automatizadas (só as 5 premissas de mercado seguem manuais, por natureza). Segunda vez nesta fatia que testar com dado real (não só planejar) pegou um problema que a lógica sozinha não pegaria — desta vez em dobro, um falso alarme (VALE3) e um problema de verdade (MGLU3), corrigidos com dado de duas empresas reais em vez de uma regra chutada

### 2026-07-10 — Sessão 6

- **Continuação da Fase 2 (`staking_yield`, usuário escolheu investigar stakingrewards.com)**: docs reais consultados via `WebFetch`/`WebSearch` (`api-docs.stakingrewards.com`) — endpoint `POST https://api.stakingrewards.com/public/query`, GraphQL, header `X-API-KEY`. A doc menciona "free tier for hobbyists" mas não confirma limites nem se o metric certo (`real_reward_rate`, o "yield real" que a spec original pede) está incluso — só o `reward_rate` nominal aparece nos exemplos oficiais. Consultado o usuário: escolhida a alternativa de usar o `reward_rate` nominal como proxy (evita depender de um metric potencialmente pago), escrito `cripto_stakingrewards.py` + integração em `main.py`/`.env.example`/botão do `CryptoScorePanel.tsx`, seguindo o mesmo molde de TVL Trend/Net Issuance
- **Achado que invalidou o plano antes de testar contra a API real**: ao tentar gerar a chave, usuário achou a página de preços atual (`stakingrewards.com/data-api`) — **não existe free tier nenhum**, só 4 planos pagos (Standard €166/mês, Advanced €333/mês, Professional €666/mês, Enterprise sob consulta), todos cobrados anualmente, sem trial. A menção a "free tier" nos docs técnicos parece resquício de um produto/plano legado, não reflete o pricing atual. Confirmado via `WebFetch` na própria página de preços
- **Revertido tudo** (`git checkout`) — `cripto_stakingrewards.py`, `main.py`, `.env.example`, botão do `CryptoScorePanel.tsx` — não faz sentido manter cliente/integração de uma fonte que não tem como ser testada nem usada sem assinar um plano pago; ficaria código morto no repositório público. `staking_yield` passa a fazer parte do mesmo grupo de `mvrv_z_score`/`puell_multiple`/`exchange_netflow` (sem fonte gratuita conhecida) — permanece manual
- **Marco**: nenhuma mudança de código líquida nesta sessão (feature implementada e revertida na mesma sessão, antes de qualquer commit) — só a atualização deste log. Lição: mesmo com a doc técnica mencionando "free tier", vale confirmar contra a página de pricing atual antes de pedir pro usuário gerar a chave, não só contra a doc de API (que pode estar desatualizada em relação ao produto)

- **Continuação da Sessão 6 (dividendo médio do Bazin)**: investigada a brapi como alternativa à bolsai (bloqueada, 403 Pro-only). Achado: a brapi tem **dois formatos de API concorrentes** — o endpoint dedicado `GET /api/v2/stocks/dividends` devolve array vazio sem token (mesmo pros 4 tickers de teste), mas o endpoint mais antigo `GET /api/quote/{ticker}?dividends=true` devolve o histórico completo de graça pros 4 tickers de teste — só que qualquer ticker real fora desses 4 exige plano pago (`brapi.dev/pricing`: Startup R$99,99/mês = 1 ano de histórico, Pro R$116,66/mês = 10+ anos), então essa rota resolve só a demo, não o caso de uso real (portfólio próprio). Consultado o usuário, que sugeriu investigar o Yahoo Finance
- **Yahoo Finance (API não-oficial, mesma usada por baixo dos panos pela lib `yfinance`)**: `GET query1.finance.yahoo.com/v8/finance/chart/{ticker}.SA?events=div` testado contra a API real (via container Docker de rede bridge — o `curl`/`requests` direto do ambiente de desenvolvimento deste agente estava recebendo timeout/resposta vazia contra `brapi.dev`, aparentemente proteção de bot da Cloudflare specífica desta rede; o ambiente real do usuário e o `docker compose run` do próprio projeto não têm esse problema) — funciona de graça, sem cadastro, **pra qualquer ticker real da B3** (testado com WEGE3, BBAS3, MGLU3, nenhum deles um dos 4 tickers especiais da brapi). Sem contrato formal de estabilidade (API não documentada oficialmente), mas resolve o caso real, diferente da brapi/bolsai
- **Python**: `data-collector/sources/acoes_yahoo.py` (novo) — `fetch_dividends_avg`, mesmo cálculo já usado com a bolsai (soma por ano, descarta o ano corrente parcial, média dos últimos 5 anos completos), mas resiliente por ticker (`try/except` por request, tickers sem dividendo ou com erro são só ignorados — sem derrubar o resto, já que é API não-oficial e pode falhar de formas imprevisíveis). `main.py` trocou a chamada de `acoes_bolsai.fetch_dividends_avg` pra `acoes_yahoo.fetch_dividends_avg`; removido o `try/except` de isolamento do 403 da bolsai (não se aplica mais) e a função morta `acoes_bolsai.fetch_dividends_avg` (sempre falhava no Free, nunca mais chamada)
- **Bug real pego só no teste contra dado de verdade (não no plano)**: `INSERT INTO stock_dividends_avg` falhou com `no such column: avg_dividend_5y` — a coluna real da tabela (e o campo do entity Rust gerado na Sessão 5) é `avg_dividend5y` **sem underscore antes do "5y"** (o `#[derive(DeriveIden)]` do sea-orm-migration converte o nome do enum `AvgDividend5y` pra snake_case sem separar o dígito). Isso nunca tinha sido pego porque a bolsai sempre falhava (403) antes de qualquer `INSERT` acontecer — a tabela nunca tinha recebido uma linha de verdade até agora. Pior ainda: o frontend (`StockCollectorPanel.tsx`) já esperava `avg_dividend_5y` (com underscore) desde que a tela foi escrita — ou seja, a tabela "Average dividend" estava programada pra sempre mostrar `undefined`/quebrar assim que uma linha real chegasse, só nunca tinha sido exercitada
- **Corrigido com migration** (não editando a coluna à mão): `m20260710_220000_rename_avg_dividend5y_column` — `ALTER TABLE ... RENAME COLUMN avg_dividend5y TO avg_dividend_5y`, usando `Alias::new(...)` nos dois nomes (contorna o mesmo problema de conversão automática que causou o bug). `migrate up` + `generate entity` rodados (mesmo fluxo de sempre, `chown -R 1000:1000` depois) — o entity gerado já veio com `avg_dividend_5y: f64`, batendo com o frontend sem precisar tocar em `commands/collector.rs` (serialização automática via serde)
- `cargo check`, `cargo test --lib` (32 testes) e `npx tsc --noEmit` limpos. Coletor rodado de verdade via `docker compose run` **três vezes seguidas** antes de mexer no Rust (confirmando reprodutibilidade, não só uma tentativa) — dividendo médio real gravado pros 4 tickers (PETR4 R$7,88, MGLU3 R$0,29, VALE3 R$8,26, ITUB4 R$1,23). Depois da migration, `main.py` completo rodado de novo confirmando o `INSERT` funcionando. Smoke test real (`dev.sh`) — **usuário confirmou visualmente a tabela "Average dividend" populada** ("deu boa")
- **Marco**: dividendo médio do Bazin automatizado via Yahoo Finance, resolvendo pra qualquer ticker real (não só demo) — pendência aberta desde a Sessão 5 fechada. De quebra, um bug latente de nomenclatura (coluna/campo/frontend desalinhados) foi descoberto e corrigido só porque essa foi a primeira vez que a tabela recebeu dado de verdade — reforça o hábito de sempre testar contra o fluxo real antes de fechar, mesmo em tabelas "antigas" que nunca tinham sido exercitadas de fato

### 2026-07-11 — Sessão 7

- Usuário decidiu seguir pra **Fase 5 — Monitoramento & Alertas**, começando pela etapa 5.1 (cadastro de regra de alerta). Planejado com `/plan` por atravessar as 3 camadas do projeto pela primeira vez desde a fatia inicial da Fase 2 (migration/entity/comando Rust + painel React), com duas explorações em paralelo (backend SeaORM/Tauri, frontend React) seguidas de um agente de Plan validando o desenho técnico antes de codar
- **Duas decisões de design consultadas com o usuário antes de codar** (ambas aceitas na opção recomendada): (1) alerta de ação reusa o `fair_price` já calculado numa valuation salva (`valuation_id`), em vez de pedir um preço-alvo novo digitado à mão; (2) alerta de indicador cripto reusa o signal GREEN/NEUTRAL/RED já calculado via `indicator_thresholds`/`domain/crypto_score.rs::classify`, em vez de uma faixa numérica própria por regra. As duas evitam duplicar conceito que já existe no app
- **Schema**: tabela única polimórfica `alert_rule` (`target_type: "stock_price"|"crypto_indicator"`, `valuation_id` nullable FK→`valuation` com `ON DELETE CASCADE`, `condition`, `coin`/`indicator` nullable, `is_active`, `created_at`) — segue o precedente do discriminador `valuation.model`. Migration `m20260711_093000_create_alert_rule_table` (primeira coluna `boolean` do projeto — `is_active` — confirmada gerando certo via `sea-orm-cli generate entity`, `bool` no Rust)
- **Achado ao validar o rascunho do agente de Plan contra o código real**: a alegação inicial de que "nenhuma migration usa `.foreign_key(...)` explícito, só a relação SeaORM" estava errada — a migration original (`m20260709_010051_create_valuation_and_bazin_inputs.rs`, `bazin_inputs`) usa sim `.foreign_key(ForeignKey::create()...)`. Conferido lendo o arquivo fonte antes de escrever a migration nova, que replica o mesmo padrão (FK declarado tanto na migration quanto na relação `belongs_to` da entity)
- **Backend**: `commands/alert_rule.rs` — `create_alert_rule` (valida combinação de campos por `target_type`, confere que a valuation referenciada existe e tem `fair_price`, ou que o `indicator` existe em `indicator_thresholds` reusando `AppError::UnknownIndicator` do mesmo jeito que `record_crypto_indicator` já faz), `list_alert_rules` (DTO `AlertRuleView` com `ticker`/`fair_price` resolvidos via batch-fetch, sem N+1), `set_alert_rule_active` (pausar/retomar sem apagar), `delete_alert_rule`. Nenhuma variante nova de `AppError` — reusa `InvalidGuard`/`UnknownIndicator`/`NotFound`. Editar `target_type`/`condition` de uma regra existente ficou fora de escopo (apagar e recriar)
- **Frontend**: `src/alerts/AlertsPanel.tsx` (novo) — formulário com toggle de tipo de alerta que troca os campos exibidos (Select de valuation salva pra ação, texto livre de `coin` + Select de indicador pra cripto, igual ao padrão já usado no `CryptoScorePanel`), lista com badge ativo/pausado + botão de toggle, delete com o mesmo "clique de novo pra confirmar" do `SavedValuationsPanel`. Nova aba "Alerts" em `App.tsx`
- `cargo check`, `cargo test --lib` (32 testes, sem regressão) e `npx tsc --noEmit` limpos. Sem ferramenta de automação de UI disponível pro app desktop (não é possível dirigir o console de devtools do WebView via shell), então a etapa de "testar os comandos antes da UI existir" do plano original foi fundida num único smoke test real — **usuário rodou o app e testou os fluxos de criar/listar/pausar/retomar/apagar pros dois tipos de alerta, confirmou que funcionou** ("deu boa")
- **Marco**: Fase 5.1 completa — cadastro de regra de alerta funcionando ponta a ponta pros dois tipos (ação e cripto). Falta 5.2 (verificação periódica, provavelmente `tokio::time::interval` + `tauri::async_runtime::spawn`, hoje inexistente no projeto) e 5.3 (notificação, também inexistente — nem `@tauri-apps/plugin-notification` nem toast lib instalados ainda)

### 2026-07-11 — Sessão 8

- Usuário escolheu seguir com **Fase 5.2 — verificação periódica**, entre as duas opções deixadas em aberto no fim da Sessão 7. Planejado com `/plan` (agente de Plan validando o desenho contra o código real antes de codar), com três decisões de design consultadas antes — todas aceitas na opção recomendada: (1) o checker só reavalia dado já existente no banco, não dispara o coletor Python sozinho; (2) estado de disparo rastreado via tabela nova **append-only** `alert_event` (mesma filosofia "nunca sobrescreve" de `valuation`), não campos mutáveis em `alert_rule`; (3) a aba Alerts já mostra o estado (badge "Triggered") via polling da UI, sem sistema de eventos push do Tauri
- **Achado do agente de Plan ao validar contra o código real**: `evaluate_stock_price` precisa comparar contra o `stock_quotes.price` mais recente, não `valuation.current_price` (que fica congelado no momento do cálculo) — se não, a checagem vira um no-op permanente. Também: transições são bidirecionais (a condição pode "entrar" e depois "sair"), cada uma com sua própria linha em `alert_event`, não só o sentido de disparo
- **Schema**: tabela `alert_event` (`alert_rule_id` FK→`alert_rule` cascata, `is_triggered`, `message`, `created_at`) — migration `m20260711_171445_create_alert_event_table`. `sea-orm-cli generate entity` regenerado automaticamente adicionou `Relation::AlertEvent` (has-many) em `entity/alert_rule.rs`, efeito colateral esperado (mesmo padrão de `valuation.rs`/`Relation::AlertRule` hoje)
- **Domain**: `domain/alert_check.rs` — duas funções puras (`evaluate_stock_price`, `evaluate_crypto_indicator`), sem `Result`/`AppError` (diferente de `crypto_score::classify`: aqui condição desconhecida não é um erro que valha propagar, já é validada na criação da regra — fallback seguro é só "nunca dispara", já que o loop de background não pode crashar). 10 testes novos, mesmo estilo de `crypto_score.rs`
- **Orquestração**: `alert_checker.rs` (módulo novo, irmão de `commands`/`domain`/`db`/`entity`/`error` — não é um `#[tauri::command]`, nunca é chamado do frontend). `check_active_rules` reusa o padrão de batch-fetch-e-montar-`HashMap`-em-Rust já usado em `list_alert_rules` pra resolver "cotação/leitura mais recente por chave" (não existe "latest per group" nativo no SeaORM usado aqui, e não vale a pena SQL cru pra isso). `spawn_periodic_check` chamado em `lib.rs::run()` logo após conectar o banco (clone barato de `DatabaseConnection`, que é `Arc`-backed)
- **Frontend**: `AlertsPanel.tsx` ganhou `refetchInterval: 30_000` no `useQuery` existente e um segundo `Badge` vermelho "Triggered" (com `last_message` como tooltip) ao lado do badge Active/Paused já existente — sem componente novo
- `cargo check`, `cargo test --lib` (42 testes, 32 antigos + 10 novos) e `npx tsc --noEmit` limpos. Smoke test real: usuário rodou o app via `docker compose up`, e `docker compose restart` foi usado repetidamente pra forçar o tick imediato do checker na inicialização (em vez de esperar os 5 min do intervalo) — confirmados **os 4 cenários**: (1) regra sem cotação/leitura ainda fica silenciosamente sem badge; (2) regra cripto (ETH/tvl_trend/SIGNAL_GREEN) dispara usando dado real já existente no banco; (3) regra de ação (BBAS3) dispara com cotação simulada abaixo do fair price e o badge some quando a cotação simulada volta pra acima — prova a lógica bidirecional e o "só grava na mudança" (evento do ETH não duplicou nos restarts); (4) pausar uma regra disparada mantém o badge/mensagem como último estado conhecido. Cotações fake de teste (`source='manual-test'`) removidas do banco depois, mantendo só o evento real do ETH
- **Marco**: Fase 5.2 completa — checagem periódica funcionando ponta a ponta pros dois tipos de alerta, validada com dado real e simulado. Falta só 5.3 (notificação nativa do SO e/ou destaque na UI) pra fechar a Fase 5 inteira

### 2026-07-11 — Sessão 9

- Usuário trouxe uma ideia pra debater (não pra implementar): reusar a tecnologia do TruthID (identidade descentralizada por wallet + dispositivo confiável) e IPFS pra sincronizar o Practice Valuation entre máquinas. Análise: o fluxo de "Login" do TruthID (QR + callback HTTP pro backend do integrador) não encaixa aqui, já que o app não tem backend — o que encaixaria é o fluxo de **"Add Device"/`DeviceRegistry`** (confiança entre dispositivos), mais parecido com Syncthing do que com OAuth. IPFS puro também não resolve sync de um SQLite mutável sem uma camada CRDT/log por cima (o que aliás já combina com a filosofia append-only que o projeto já usa em `valuation`/`alert_event`). Trade-off levantado: dependência de blockchain + nó IPFS só pra sincronizar duas máquinas pessoais é bem mais peso que `git`/Syncthing — só compensaria como dogfooding proposital do TruthID. **Usuário decidiu deixar quieto** e seguir com a Fase 5.3
- **Fase 5.3 — notificação nativa do SO**, planejada com `/plan` (duas explorações em paralelo, backend Rust e frontend React, antes de desenhar). Decisão consultada e aceita: notificar só ao **entrar** em triggered, não ao sair ("alerta resolvido") — menos ruído
- **Achado da exploração**: `check_active_rules` já fazia detecção de borda (só grava `alert_event` na mudança de estado) — a notificação só precisava entrar dentro do `if is_triggered { ... }` já existente, ganhando o dedup de graça. O bloqueio técnico real: `spawn_periodic_check` era chamado em `lib.rs::run()` **antes** do `Builder` existir, sem `AppHandle` disponível (só `DatabaseConnection`) — precisou migrar pra dentro de um `.setup()` novo (não existia nenhum ainda)
- Verificado contra a doc real do `tauri-plugin-notification` (`v2.tauri.app/plugin/notification`, via `WebFetch`) antes de codar, mesma regra de "não confiar em memória pra API de terceiro" já aplicada a fontes de dados — `.plugin(tauri_plugin_notification::init())` encaixa direto na chain do `Builder`, disparo do lado Rust é `app.notification().builder().title(..).body(..).show()`
- **Código**: `Cargo.toml` (+`tauri-plugin-notification`), `capabilities/default.json` (+`notification:default`), `lib.rs` (spawn do checker migrado pro `.setup()`, plugin registrado), `alert_checker.rs` (`spawn_periodic_check`/`check_active_rules` passam a receber `AppHandle`; notificação disparada só quando `is_triggered` vira `true`, erro só logado via `eprintln!`, nunca derruba o loop). Nenhuma mudança de frontend — `AlertsPanel.tsx` continua como estava, a notificação é só mais um canal
- **Achado de infra durante o smoke test**: o `docker-compose.yml` só montava X11, não o socket D-Bus — notificação nativa no Linux passa por `org.freedesktop.Notifications` via D-Bus, não X11. Primeira tentativa (só montar `/run/user/1000/bus` + `DBUS_SESSION_BUS_ADDRESS`) não bastou: o container roda como `root`, e o D-Bus autentica pela credencial real do processo (`SO_PEERCRED`), não por cookie como o X11 — root não bate com o dono da sessão (uid 1000). Confirmado com `dbus-send` direto de dentro do container ("Did not receive a reply")
- **Fix consultado e aceito**: rodar o processo do container como uid 1000 (o usuário `node` que a imagem `node:20-bookworm-slim` já traz), em vez de root. `Dockerfile` ganhou `ENV RUSTUP_HOME/CARGO_HOME/HOME=/root` explícitos (pra não depender do `$HOME` de quem estiver rodando) + `chmod 755 /root` e `chmod -R a+rX /root/.cargo /root/.rustup` (toolchain instalado como root vira legível/executável pelo uid 1000); `docker-compose.yml` ganhou `user: "1000:1000"`. Os 3 volumes nomeados (`cargo-registry`, `cargo-git`, `cargo-target`), até então root-owned, precisaram de um chown único (`docker compose run --rm -u root ... chown -R 1000:1000 ...`) — mesmo achado, encontrado também num diretório órfão (`migration/target`, escrito por root direto no bind mount por um `cargo check`/`test` anterior desta sessão) e um diretório vazio solto (`desktop/desktop`, resíduo de sessão anterior, removido)
- `cargo check` e `cargo test --lib` (42 testes, sem regressão) e `npx tsc --noEmit` limpos, rodados antes do fix de D-Bus. O primeiro smoke test real (cotação simulada de BBAS3 abaixo do fair price, mesmo truque da Sessão 8) confirmou que a detecção de borda e o caminho até a chamada de notificação executam certo (evento gravado no banco na hora certa, sem erro logado) — mas a notificação em si **não apareceu**, o que levou à investigação e ao fix de D-Bus/uid acima
- **Bloqueio final, fora do escopo do projeto**: depois do fix de D-Bus/uid, o rebuild da imagem parou de funcionar por um motivo totalmente não relacionado — a rede do Docker no host quebrou (erro `failed to add the host <=> sandbox pair interfaces: operation not supported` até num `docker run alpine` puro). Kernel log mostra a máquina passou por suspend/resume (`s2idle`) pouco antes — padrão conhecido de o Docker perder a rede de bridge depois de suspender. Usuário reiniciou o daemon (`sudo systemctl restart docker`, rodado por ele mesmo via `!`, já que o host não tem sudo sem senha) mas não resolveu dessa vez; provavelmente precisa de reboot da máquina. Não insisti em mexer mais fundo na rede do host (uma tentativa de recriar a rede `bridge` do Docker foi corretamente barrada pelo classificador de segurança por ser destrutivo em infra não solicitada)
- Dados de teste limpos do banco ao final (mesma convenção da Sessão 8): cotação `manual-test` de BBAS3 e o `alert_event` gerado a partir dela removidos; evento real do ETH mantido
- **Marco**: código da Fase 5.3 completo e compilando/testando limpo; fix de D-Bus/uid no Docker também codado. **Falta só validar de verdade que a notificação aparece**, bloqueado no momento por um problema de rede do Docker no host (não relacionado ao projeto) que só deve se resolver com um reboot da máquina

### 2026-07-12 — Sessão 10

- Rede do Docker do host voltou a funcionar sozinha (sem precisar do reboot cogitado no fim da Sessão 9 — um `docker run alpine` simples já confirmou). `docker compose build` (Dockerfile mudou na Sessão 9) + `docker compose up` rodados de novo
- **Bug novo encontrado no boot** (efeito colateral do fix de D-Bus/uid da Sessão 9, não pego no smoke test daquela sessão porque ela nunca chegou a rebuildar com sucesso): `Failed to setup app: Permission denied (os error 13)` logo depois do MESA/libGL warning. Causa: `Dockerfile` define `ENV HOME=/root` (necessário só em build-time, pra `RUSTUP_HOME`/`CARGO_HOME` acharem o toolchain), mas isso persiste em runtime — como uid 1000 (não mais root) só tem leitura/execução em `/root` (`chmod 755` da Sessão 9), o WebKitGTK não conseguia criar `~/.config`/`~/.cache`/`~/.local/share`. Corrigido com `HOME=/home/node` no `environment:` do `docker-compose.yml` (só runtime, `/home/node` já existe e já é do uid 1000 por vir da própria imagem `node:20-bookworm-slim`) — Dockerfile/build não mudou. App voltou a abrir normalmente
- Usuário pediu uma feature nova: no topo da aba Valuation tinha um painel de tabelas cruas (`StockCollectorPanel.tsx`) com um botão que rodava o coletor Python pra **todos** os tickers do `config.yaml` — construído nas primeiras sessões só pra validar o pipeline, nunca foi pensado pra uso real. Pedido: remover esse painel e, no lugar, cada um dos 7 formulários ganhar um botão (diferente do "Calculate") que busca o dado daquele ticker específico e preenche os campos automaticamente na medida do possível
- Planejado com `/plan` (atravessa Python + Rust + 7 componentes React) — duas decisões consultadas e aceitas: (1) o botão de fetch roda o pipeline Python **inteiro** pro ticker (quotes+fundamentals+dividends+DCF), não só o subconjunto que aquele formulário usa — mais simples, aceito o custo de velocidade; (2) "Current dividend (D0)" do Gordon/Projected Ceiling é preenchido com a média de dividendos de 5 anos (única fonte disponível) + aviso de que é aproximação
- **Backend**: `data-collector/main.py` ganhou `--ticker <TICKER>` (parsing manual de `sys.argv`, mesmo estilo do dispatch `crypto` já existente) — quando passado, ignora `config.yaml` e roda só aquele ticker; `collect_stock_*` já eram genéricas sobre `list[str]`, sem mudança nelas. `commands/collector.rs::run_stock_collector` ganhou parâmetro `ticker: String`, repassado como `&["--ticker", &ticker]` pro `run_collector` (mesmo molde de `run_crypto_collector`'s `&["crypto"]`) — `AtomicBool` do lock preservado sem mudança
- **Frontend**: `StockCollectorPanel.tsx` deletado. Novo módulo `src/collector/` com `types.ts` (os 4 tipos de linha, movidos de dentro do painel antigo), `latestForTicker.ts` (versão pra 1 ticker do antigo `latestPerTicker`) e `useTickerCollector.ts` (hook compartilhado pelos 7 formulários — roda o coletor escopado, depois lê de volta as 4 queries `list_stock_*` e resolve a linha mais recente por ticker). Cada formulário ganhou um botão `type="button"` (ícone `RefreshCw` da `lucide-react`, `variant="outline" size="icon"`) do lado do campo Ticker — nunca dispara o submit do "Calculate", valida ticker vazio antes de chamar, mostra "No data found" quando nada volta. Mapeamento por formulário: Bazin (preço+dividendo médio), Graham (preço+LPA+VPA), Gordon/Projected Ceiling (preço+dividendo médio como proxy de D0, com aviso), DCF (preço+todos os 8 campos contábeis do DCF, pulando os nullable quando `null`), Banks (preço+VPA+ROE), RNAV (preço+shares outstanding, reaproveitado da tabela do DCF)
- **Achado real testando contra dado de verdade** (não coberto pelo plano): ao testar com BBAS3 (fora dos 4 tickers do `config.yaml`), o fetch deu "No data found" — a brapi (fonte da cotação) devolve `401 Unauthorized` pra qualquer ticker fora de 4 tickers demo sem token pago, e `collect_stock_quotes` não tinha `try/except`, então a exceção derrubava o `main.py` inteiro antes de chegar em fundamentos/dividendos/DCF (que vêm de fontes sem essa restrição). Usuário decidiu **substituir a brapi inteiramente pela Yahoo Finance** (mesma fonte não-oficial já usada pro dividendo médio desde a Sessão 6, confirmada de graça pra qualquer ticker real) em vez de só tolerar a falha — resolve a causa raiz, não só o sintoma
- **Código do fix**: `acoes_yahoo.py` ganhou `fetch_quotes` (mesmo endpoint `v8/finance/chart/{ticker}.SA`, lendo `meta.regularMarketPrice`, tolerante a falha por ticker como `fetch_dividends_avg` já era). `acoes_brapi.py` deletado. `main.py` trocou a chamada e o `source` gravado (`"brapi"` → `"yahoo_finance"`). Limpeza de referências mortas: `BRAPI_TOKEN` removido de `.env.example`, `config.yaml` e `data-collector/README.md` atualizados (README também ganhou um aviso pra não duplicar status aqui — só `PROJECT_STATE.md` é a fonte viva)
- `cargo check`, `cargo test --lib` (42 testes) e `npx tsc --noEmit` limpos. `python3 main.py --ticker PETR4` e depois `--ticker BBAS3` rodados isolados contra API real antes de qualquer coisa no Rust/UI — confirmado que só a linha do ticker pedido é gravada (outros tickers intocados) e que BBAS3 (banco) some do DCF **corretamente** (mesma taxonomia de DRE diferente já confirmada com ITUB4 em sessões anteriores, não é bug). Smoke test real na tela pelo usuário — **confirmado funcionando em Bazin, Graham, Banks, DCF (PETR4 preenche, BBAS3 não preenche DCF como esperado) e RNAV** (preço+shares outstanding; usuário perguntou se faltava mais campo — não, landbank/inventory/net cash são avaliação patrimonial própria, sem fonte coletável)
- Usuário trouxe outra ideia (não implementada, só planejada): chat de IA integrado ao app, chave própria do usuário (Gemini/Claude/ChatGPT), com acesso só-leitura ao banco (valuations salvas, alertas) + contexto de como o sistema funciona, sessão de conversa que reseta ao fechar o app (não o painel, não a cada 24h). Planejado com `/plan` no fim desta sessão (pesquisa real da doc do Gemini + achado de que o Stronghold do Tauri está sendo descontinuado, ver Fase 7 em "Fases Detalhadas") — virou a **Fase 7**, com 7 etapas registradas pra implementar aos poucos a partir da próxima sessão, começando pelo storage seguro da chave (maior risco técnico) e o Gemini de ponta a ponta
- **Marco**: fetch por ticker funcionando ponta a ponta nos 7 formulários, painel de teste removido, brapi eliminada do projeto (Yahoo Finance cobre cotação + dividendo pra qualquer ticker real de graça). Fase 5.3 (notificação nativa) **ainda não foi re-testada** nesta sessão — o tempo foi todo pra essa feature nova, puxada pelo usuário assim que o app voltou a subir

**Pendências pra próxima sessão** (em ordem):
1. Fase 5.3: o app já está rodando de novo nesta sessão (rede do Docker normalizou sozinha) — falta só refazer o smoke test de notificação nativa que ficou pendente desde a Sessão 9: criar/forçar uma regra disparando (truque de cotação simulada) e confirmar que a notificação do SO aparece de verdade. Se não aparecer, olhar `docker compose exec desktop sh -c 'echo $DBUS_SESSION_BUS_ADDRESS; ls -la /run/user/1000/bus'` e `docker compose exec desktop id`
2. Fase 7 (chat de IA): começar pela etapa 7.1 (storage seguro da chave via `keyring-rs`, testado de verdade contra o container de dev antes de qualquer UI — é o maior risco técnico, ver Fase 7 em "Fases Detalhadas" pro desenho completo em 7 etapas)
3. Fase 2: nenhuma pendência de dado com caminho conhecido sobrando — `staking_yield` (sem free tier), `active_addresses_trend` (Etherscan Pro-only), `mvrv_z_score`, `puell_multiple` e `exchange_netflow` (sem fonte gratuita conhecida) não valem reabrir sem uma ideia nova
4. README.md e LICENSE na raiz do repo ainda não existem (Fase 0.5/6.2/6.3) — usuário disse que prefere esperar ter mais "repertório" antes de escrever o README
5. Quando o usuário voltar a mexer no TruthID mobile, lembrar que o cache Docker foi limpo (Sessão 1 do Practice Valuation) — primeiro `docker compose up` de lá vai ser mais lento
6. Se algum dia migrar a imagem Docker (Node/Debian), lembrar dos 3 fixes de rede/instalação da Sessão 1 (IPv6, npm audit, node_modules corrompido) — não são óbvios

---

### 2026-07-12 — Sessão 11

- Usuário trouxe outra ideia (só brainstorm, não implementada): sincronizar o Practice Valuation entre dispositivos (celular, outro PC) de forma descentralizada, usando IPFS e reaproveitando a identidade do TruthID em vez de um sistema de conta próprio
- Descartado logo de cara: P2P direto entre devices (usuário não gosta). Redesenhado como sync assíncrono via IPFS (publish/pull, sem exigir dois devices online juntos)
- Investigação real no código do TruthID (não só teoria) confirmou duas coisas importantes: (1) `VaultRegistry.sol` já é praticamente o mecanismo necessário (CID versionado por identidade, blob cifrado, múltiplos provedores de pin com health-check, testado em hardware real) — só é 1 vault por identidade hoje, amarrado ao password manager, precisaria generalizar; (2) o login por QR do TruthID exige `callbackUrl` https obrigatório (`approval_screen.dart`), mas a escrita da sessão on-chain já acontece incondicionalmente antes do POST — então um modo de login "sem callback" (fallback on-chain) é barato de expor, só tornando o campo opcional
- Virou a **Fase 8** (ver "Fases Detalhadas") — ideia registrada, nenhuma etapa concreta ainda, depende de decisões do lado do TruthID também (ver `PROJECT_STATE.md` do TruthID, Sessão 94)
- Nenhum código tocado nesta sessão — só a atualização deste arquivo

---

## Como Usar Este Arquivo

1. **Ao começar uma sessão**: diga ao Claude Code "leia o PROJECT_STATE.md e me ajude a continuar"
2. **Ao terminar uma sessão**: o Claude atualiza o Log de Sessões e marca etapas concluídas
3. **Ao tomar uma decisão**: registrar em "Decisões de Arquitetura em Aberto"
4. **Ao mudar de máquina**: sincronizar via git (`git init` já feito neste diretório)
