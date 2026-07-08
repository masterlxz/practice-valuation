# Practice Valuation — Estado do Projeto

> Este arquivo é o centro de controle do projeto. Atualizado a cada sessão de trabalho.
> Pode ser lido por qualquer instância do Claude Code em qualquer máquina para retomar o contexto.
> Última atualização: 2026-07-08 (Sessão 1, continuação — smoke test concluído: `./desktop/dev.sh` builda e abre a janela do Tauri de verdade, confirmado pelo usuário)

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
- `.gitignore` deve cobrir `.env`, banco de dados local (`*.db`, `*.sqlite`), pastas de build/dist, e qualquer arquivo de credencial (`.credentials/`, `service_account.json`, etc. — herdado do spec antigo, ver `docs/spec_automacao_dados.md`).
- **Nunca hardcodar dados pessoais de portfólio** (valores investidos, quantidades, saldo) em código de exemplo, testes ou fixtures — usar dados fictícios.
- Antes de cada `git push` para o remoto público, revisar `git status`/`git diff` procurando por segredos, mesmo em arquivos com nome inofensivo.
- Ao decidir o banco de dados local (Fase 1), garantir que o arquivo do banco fique fora do controle de versão por padrão.

---

## Diretriz de ensino (IMPORTANTE — ler antes de cada sessão)

O usuário não é iniciante em programação (ver `docs/spec_automacao_dados.md` — já desenhou sozinho uma arquitetura de coleta de dados via APIs, com bom domínio de Python). O ponto de partida é **iniciante em construir uma aplicação desktop completa**: escolha de framework de UI, empacotamento/distribuição, banco de dados local, e organização de um projeto maior que um script.

**Regras para o Claude:**
- Ir com calma — construir aos poucos, sessão a sessão, sem pressa pra "terminar tudo de uma vez"
- Antes de decidir framework/lib/arquitetura, explicar as opções e trade-offs e esperar a decisão do usuário — nunca decidir sozinho por ele quando a decisão for de gosto/direção do projeto
- Explicar o conceito novo (de UI, empacotamento, banco de dados, etc.) antes de escrever o código que o usa
- Não assumir conhecimento prévio de frameworks de UI desktop, ORMs/SQL local, ou empacotamento de apps — mas pode assumir Python e lógica de programação em geral
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
Substitui a ideia original de planilha (ver `docs/spec_automacao_dados.md`, histórico) por um app com banco de dados local.

**O que ele precisa fazer (visão do usuário, ainda sendo refinada):**
- Puxar o máximo de dados possível de fontes externas (fundamentos de ações BR, dados on-chain/mercado de cripto), com espaço pra ajuste manual quando necessário
- Guardar **múltiplos preços-teto/cálculos de valuation por ativo**, cada um com seu próprio conjunto de premissas (ex: duas projeções do mesmo ativo com taxas de crescimento diferentes, ambas salvas e comparáveis lado a lado)
- Cadastrar premissas por ativo (incluindo cripto) e monitorar indicadores automaticamente
- Avisar o usuário quando um ativo entrar em "zona de compra" segundo as premissas cadastradas
- Banco de dados **local** por enquanto — sync entre máquinas/nuvem é ideia pra mais adiante (ver Roadmap)

**Decidido até agora** (ver "Decisões de Arquitetura em Aberto"):
- Stack híbrida: app em **Tauri + Rust + React/TypeScript** (reaproveitando o padrão do TruthID), coleta de dados em **Python** (reaproveitando o desenho do `docs/spec_automacao_dados.md`), os dois se comunicando só através de um banco **SQLite** local compartilhado — sem API/IPC entre eles

- UI: **Tailwind CSS + shadcn/ui (Radix) + TanStack Table**, visual **arejado tipo dashboard** (não denso tipo planilha, apesar da ideia original de "funcionar como planilha" — isso ficou pro comportamento/dado, não pra densidade visual)

**Ainda não decidido**:
- Biblioteca de gráfico (pra tela de cripto/indicadores, Fase 4.3) — avaliar quando chegar lá (candidatos: Recharts, ou lightweight-charts da TradingView, mais voltada pra preço/candlestick)
- Onde/como o Python roda (script chamado sob demanda pelo Tauri, ou cron independente, ou serviço que o app dispara)
- Lista exata de metodologias de preço-teto (o usuário vai trazer sua lista numa próxima sessão)

---

## Status Geral

```
Fase 0 — Fundamentos & Decisões de Arquitetura   [ ] Não iniciada
Fase 1 — Modelo de Dados (schema do banco local)  [ ] Não iniciada
Fase 2 — Coleta de Dados (ações BR + cripto)      [ ] Não iniciada
Fase 3 — Motor de Cálculo (preço-teto/valuation)  [ ] Não iniciada
Fase 4 — Interface Desktop                        [ ] Não iniciada
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
  - `docs/spec_automacao_dados.md` — mantido como referência de fontes de dados
  - Ainda falta: README.md na raiz do repo, LICENSE
- [ ] 0.6 — Checklist de segurança aplicado desde o primeiro commit (ver "Diretriz de segurança" acima)

---

### Fase 1 — Modelo de Dados

**Objetivo**: desenhar o schema do banco local que sustenta tudo — ativos, premissas, cálculos salvos, indicadores e alertas.

**Rascunho inicial de entidades (a refinar quando a lista de preços-teto do usuário chegar)**:
- `asset` — ativo acompanhado (ação BR ou cripto), com tipo, ticker/símbolo, nome
- `assumption_set` — um conjunto de premissas nomeado, vinculado a um `asset` (ex: "cenário conservador", "cenário otimista")
- `valuation_calc` — um cálculo de preço-teto/valuation, vinculado a um `assumption_set`, com o resultado e a metodologia usada — histórico completo, nada é sobrescrito
- `tracked_indicator` — indicador monitorado por ativo (ex: preço, P/L, TVL, emissão líquida) com a regra de "zona de compra"
- `alert` — disparo gerado quando um `tracked_indicator` entra na zona configurada

**Etapas**:
- [ ] 1.1 — Validar o rascunho de entidades acima com o usuário
- [ ] 1.2 — Escolher engine de banco local e ORM/driver (depende da Fase 0)
- [ ] 1.3 — Migrations iniciais

---

### Fase 2 — Coleta de Dados

**Objetivo**: puxar o máximo de dado possível de fontes externas, com fallback manual quando a fonte automática não cobre.

O levantamento de fontes já foi feito antes deste projeto virar app desktop — ver `docs/spec_automacao_dados.md` (mantido como referência histórica: o desenho de fontes/APIs continua válido, só o destino dos dados mudou de "Google Sheets" para "banco de dados local do app").

**Fontes já mapeadas**:
| Categoria | Dado | Fonte primária | Fallback |
|---|---|---|---|
| Ações BR | Fundamentos (P/L, P/VP, ROE, ROIC, margens, EV/EBITDA) | bolsai | brapi |
| Ações BR | Cotação atual | brapi | bolsai |
| Ações BR | Balanço/DRE/DFC histórico | bolsai / CVM Dados Abertos (DFP/ITR) | brapi |
| Ações BR | Dividendos históricos | bolsai / brapi | — |
| Cripto | Preço, market cap, volume | CoinGecko | — |
| Cripto | TVL (DeFi) | DefiLlama | — |
| Cripto | Emissão líquida (ETH) | ultrasound.money | — |
| Cripto | Endereços ativos/transações | Etherscan | — |
| Cripto | Exchange netflow, MVRV, Puell | CryptoQuant/Glassnode (pago) | manual, link pro dashboard |
| PDF/release não estruturado | Campos qualitativos (landbank, comentários) | pdfplumber + API Claude (schema fixo) | preenchimento manual |

**Etapas**:
- [x] 2.1 — Decidir onde/como a coleta roda → **processo Python separado**, disparado **manualmente por um botão na UI** ("Run"/"Atualizar dados"), sem cron/scheduler. Decidido na Sessão 1. Mecanismo:
  - Frontend: botão chama `invoke()` de um comando Tauri
  - Backend (Rust): comando assíncrono dispara o script Python como subprocesso (não trava a UI), espera terminar
  - Python: puxa os dados das fontes e grava direto no SQLite compartilhado
  - Frontend: enquanto roda, mostra spinner; ao terminar, mostra um resumo (quantos ativos, sucesso/erro) — sem log ao vivo linha a linha por enquanto (pode vir depois se sentir falta)
  - **Guarda contra clique duplo/spam**: desabilitar o botão no frontend enquanto roda **e** ter uma trava no lado Rust (ex: `Mutex`/flag no estado do app) que recusa uma segunda chamada concorrente mesmo se disparada rápido demais — evita dois processos Python escrevendo no mesmo SQLite ao mesmo tempo e evita estourar rate limit das APIs gratuitas
  - A Fase 5 (alertas) pode um dia precisar de checagem periódica dos indicadores **já salvos** — isso é diferente de "puxar dado novo" e fica pra quando chegarmos lá
- [ ] 2.2 — Implementar clientes de fonte de dados de ações (bolsai, brapi, CVM)
- [ ] 2.3 — Implementar clientes de fonte de dados de cripto (CoinGecko, DefiLlama, ultrasound.money, Etherscan)
- [ ] 2.4 — Fallback de extração via PDF (pdfplumber + Claude), quando necessário

---

### Fase 3 — Motor de Cálculo (Preço-Teto/Valuation)

**Objetivo**: calcular e salvar preços-teto/valuation com premissas customizáveis, permitindo múltiplos cálculos por ativo.

**Etapas**:
- [ ] 3.1 — Usuário traz a lista de metodologias/fórmulas de preço-teto desejadas (ações e cripto)
- [ ] 3.2 — Modelar cada metodologia como função pura: premissas (`assumption_set`) → resultado (`valuation_calc`)
- [ ] 3.3 — Permitir salvar quantos cálculos o usuário quiser por ativo, todos comparáveis lado a lado

---

### Fase 4 — Interface Desktop

**Objetivo**: telas simples, "planilha-like", que dão espaço pra edição manual quando preciso.

**Etapas**:
- [ ] 4.1 — Tela: lista de ativos acompanhados
- [ ] 4.2 — Tela: detalhe do ativo (premissas + histórico de cálculos salvos)
- [ ] 4.3 — Tela: cripto/indicadores
- [ ] 4.4 — Tela: alertas/zona de compra
- [x] 4.5 — Direção visual → **arejado, tipo dashboard** (Tailwind + shadcn/ui + TanStack Table), decidido na Sessão 1

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
| Banco de dados local | SQLite vs DuckDB | **SQLite** ✓ — decidido na Sessão 1 junto com a decisão de stack híbrida (precisa ser lido/escrito tanto pelo Rust quanto pelo Python, e SQLite tem driver maduro nos dois: `sqlx`/`rusqlite` e `sqlite3`) |
| Onde roda a coleta de dados | Dentro do app (Rust) vs. processo separado em Python (herdado do desenho em `docs/spec_automacao_dados.md`) | **Processo separado em Python**, escrevendo no mesmo SQLite ✓ — decidido na Sessão 1. Motivo: evita reescrever em Rust o parsing de CVM/pandas e a extração de PDF, que já foram desenhados em Python e têm bibliotecas maduras lá (pandas, pdfplumber) — Rust ainda não tem equivalente tão bom. Falta decidir *como* o Python roda (script sob demanda disparado pelo Tauri, cron independente, etc. — ver Fase 2.1) |
| Sync entre dispositivos/nuvem | Adiado — ver Roadmap | Não é MVP |
| Densidade visual | Denso (planilha) vs meio-termo vs arejado (dashboard) | **Arejado, tipo dashboard** ✓ — decidido na Sessão 1 |
| Biblioteca de tabela/grid | AG Grid Community vs Glide Data Grid vs TanStack Table + shadcn/ui | **TanStack Table + shadcn/ui** ✓ — decidido na Sessão 1. Motivo: headless, visual 100% customizável e consistente com o resto do app (mesma base do shadcn/ui), em troca de implementar edição/filtro na mão em vez de ganhar pronto |
| Sistema de componentes | shadcn/ui vs Mantine vs Ant Design | **shadcn/ui** (Radix + Tailwind) ✓ — decidido na Sessão 1. Componentes copiados pro repo, visual moderno/neutro, fácil de customizar |
| Biblioteca de gráfico | Recharts vs lightweight-charts (TradingView) vs outra | Pendente — avaliar na Fase 4.3 |
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

**Pendências pra próxima sessão** (em ordem):
1. Testar o botão "Greet" na janela aberta (smoke test da comunicação React↔Rust via `invoke()`) — só visual, não testado a fundo ainda
2. Trazer a lista de metodologias/fórmulas de preço-teto (ações e cripto) — Fase 3.1
3. Decidir onde o arquivo SQLite físico vai morar (Fase 1) e começar o schema (`asset`, `assumption_set`, `valuation_calc`, `tracked_indicator`, `alert`)
4. README.md e LICENSE na raiz do repo ainda não existem (Fase 0.5/6.2/6.3)
5. Quando o usuário voltar a mexer no TruthID mobile, lembrar que o cache Docker foi limpo (Sessão 1 do Practice Valuation) — primeiro `docker compose up` de lá vai ser mais lento
6. Se algum dia migrar a imagem Docker (Node/Debian), lembrar dos 3 fixes de rede/instalação acima — não são óbvios e custaram bastante tempo de debug nesta sessão

---

## Como Usar Este Arquivo

1. **Ao começar uma sessão**: diga ao Claude Code "leia o PROJECT_STATE.md e me ajude a continuar"
2. **Ao terminar uma sessão**: o Claude atualiza o Log de Sessões e marca etapas concluídas
3. **Ao tomar uma decisão**: registrar em "Decisões de Arquitetura em Aberto"
4. **Ao mudar de máquina**: sincronizar via git (`git init` já feito neste diretório)
