# Practice Valuation — Estado do Projeto

> Este arquivo é o centro de controle do projeto. Atualizado a cada sessão de trabalho.
> Pode ser lido por qualquer instância do Claude Code em qualquer máquina para retomar o contexto.
> Última atualização: 2026-07-09 (Sessão 4, fim — score cripto fechado (Fase 3 completa), tela "Saved Valuations" completa (incl. detalhe fino de premissas), os 7 formulários + painel cripto vestidos com shadcn/ui, identidade visual dark navy + verde definida, layout geral em grid (menos vertical) e o painel cripto redesenhado como dashboard (KPI tiles + update em lote). Retomar por "Pendências pra próxima sessão", item 1)

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
Fase 2 — Coleta de Dados (ações BR + cripto)      [ ] Não iniciada
Fase 3 — Motor de Cálculo (preço-teto/valuation)  [x] Completa — 7 modelos de ação + score cripto (9 indicadores), todos ponta a ponta
Fase 4 — Interface Desktop                        [~] Em andamento (shadcn/ui + TanStack Table instalados, tela de valuations salvos completa incl. detalhe fino de premissas, 7 formulários + painel cripto vestidos, identidade visual dark+verde definida)
Fase 5 — Monitoramento & Alertas                  [ ] Não iniciada
Fase 6 — Publicação (GitHub público)               [ ] Não iniciada
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
| Ações BR | Fundamentos (P/L, P/VP, ROE, ROIC, margens, EV/EBITDA — 27 indicadores TTM) | bolsai (200 req/dia grátis) | brapi (`modules=defaultKeyStatistics`, 15.000 req/dia grátis) |
| Ações BR | Cotação atual | brapi | bolsai |
| Ações BR | Balanço/DRE/DFC histórico (contas CVM brutas) | bolsai / CVM Dados Abertos (DFP/ITR) | brapi (`balanceSheetHistory`) |
| Ações BR | Dividendos históricos | bolsai / brapi | — |
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
    ├── acoes_brapi.py          # cliente da API brapi (cotação + fallback)
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
- [ ] 2.2 — Implementar clientes de fonte de dados de ações (`acoes_bolsai.py`, `acoes_brapi.py`, `cvm_dfp.py`)
- [ ] 2.3 — Implementar clientes de fonte de dados de cripto (`cripto_coingecko.py`, `cripto_defillama.py`, `cripto_ultrasound.py`, `cripto_etherscan.py`, `cripto_stakingrewards.py`)
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
- [x] 4.2 — Tela: detalhe do ativo (histórico de cálculos salvos) — **concluída na Sessão 4**: comparação lado a lado dos campos comuns, mais o detalhe fino (linha expansível "Assumptions" por cálculo, buscando a tabela de inputs certa conforme o `model`)
- [x] 4.3 — Tela: cripto/indicadores — **vestida com shadcn/ui na Sessão 4** (mesmo painel de registro/placar, agora com Card/Select/Table/Badge em vez de HTML cru)
- [ ] 4.4 — Tela: alertas/zona de compra
- [x] 4.5 — Direção visual → **arejado, tipo dashboard** (Tailwind + shadcn/ui + TanStack Table), decidido na Sessão 1; **identidade de cor definida na Sessão 4** — dark navy + verde claro, inspirada no TruthID (ver Log de Sessões)

---

### Fase 5 — Monitoramento & Alertas

**Objetivo**: cadastrar premissas de compra por ativo e avisar o usuário quando o indicador entrar na zona configurada.

**Etapas**:
- [ ] 5.1 — Cadastro de regra de alerta por `tracked_indicator` (ex: preço abaixo do teto, indicador on-chain em faixa X)
- [ ] 5.2 — Verificação periódica (rodando localmente — cron, scheduler embutido, ou o próprio app em background)
- [ ] 5.3 — Notificação (notificação nativa do SO e/ou destaque na UI)

---

### Fase 6 — Publicação (GitHub Público)

**Etapas**:
- [ ] 6.1 — Checklist de segurança final (ver "Diretriz de segurança") antes do primeiro push público
- [ ] 6.2 — README explicando o projeto (em inglês, já que o repo é público)
- [ ] 6.3 — LICENSE
- [ ] 6.4 — `git init` + primeiro commit

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
| Sync entre dispositivos/nuvem | Adiado — ver Roadmap | Não é MVP |
| Densidade visual | Denso (planilha) vs meio-termo vs arejado (dashboard) | **Arejado, tipo dashboard** ✓ — decidido na Sessão 1 |
| Biblioteca de tabela/grid | AG Grid Community vs Glide Data Grid vs TanStack Table + shadcn/ui | **TanStack Table + shadcn/ui** ✓ — decidido na Sessão 1. Motivo: headless, visual 100% customizável e consistente com o resto do app (mesma base do shadcn/ui), em troca de implementar edição/filtro na mão em vez de ganhar pronto |
| Sistema de componentes | shadcn/ui vs Mantine vs Ant Design | **shadcn/ui** (Radix + Tailwind) ✓ — decidido na Sessão 1. Componentes copiados pro repo, visual moderno/neutro, fácil de customizar |
| Biblioteca de gráfico | Recharts vs lightweight-charts (TradingView) vs outra | Pendente — avaliar na Fase 4.3 |
| Navegação entre os 7 modelos de valuation | Seletor de modelo numa tela só vs rota própria por modelo (react-router) | **Seletor numa tela só** ✓ — decidido na Sessão 3. Dropdown troca o formulário exibido, sem roteador; mais rápido de replicar a cada novo modelo, revisitar se a navegação ficar densa demais |
| Gatilho da coleta de dados | Botão manual vs cron/scheduler periódico | **Botão manual** ✓ — decidido na Sessão 1. Rust dispara o Python como subprocesso, sem periodicidade automática por enquanto |
| Ambiente de desenvolvimento | Instalar tudo no host vs Docker | **Docker** ✓ — decidido na Sessão 1, mesmo padrão do TruthID (container único com Node+Rust+WebKitGTK+Python), sem precisar instalar nada na máquina |

---

## Roadmap de Evoluções Planejadas

- **Sync entre máquinas/nuvem**: hoje o banco é 100% local; no futuro avaliar sync (self-hosted vs serviço gerenciado)
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

**Pendências pra próxima sessão** (em ordem):
1. Fase 4.4 (tela de alertas/zona de compra) só faz sentido depois da Fase 5 (motor de monitoramento) — considerar se a próxima frente é Fase 2 (coleta de dados, ainda não iniciada) ou Fase 5 diretamente
2. Corrigir a mensagem genérica de `AppError::InvalidGuard` (achado da Sessão 4 — hoje toda guarda dos 7 modelos mostra a mesma frase do Bazin na UI, independente de qual guarda disparou)
3. README.md e LICENSE na raiz do repo ainda não existem (Fase 0.5/6.2/6.3) — se o README ganhar screenshot, já vai refletir o tema dark+verde novo e o layout em grid
4. Quando o usuário voltar a mexer no TruthID mobile, lembrar que o cache Docker foi limpo (Sessão 1 do Practice Valuation) — primeiro `docker compose up` de lá vai ser mais lento
5. Se algum dia migrar a imagem Docker (Node/Debian), lembrar dos 3 fixes de rede/instalação da Sessão 1 (IPv6, npm audit, node_modules corrompido) — não são óbvios

---

## Como Usar Este Arquivo

1. **Ao começar uma sessão**: diga ao Claude Code "leia o PROJECT_STATE.md e me ajude a continuar"
2. **Ao terminar uma sessão**: o Claude atualiza o Log de Sessões e marca etapas concluídas
3. **Ao tomar uma decisão**: registrar em "Decisões de Arquitetura em Aberto"
4. **Ao mudar de máquina**: sincronizar via git (`git init` já feito neste diretório)
