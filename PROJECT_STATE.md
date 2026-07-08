# Practice Valuation — Estado do Projeto

> Este arquivo é o centro de controle do projeto. Atualizado a cada sessão de trabalho.
> Pode ser lido por qualquer instância do Claude Code em qualquer máquina para retomar o contexto.
> Última atualização: 2026-07-08 (Sessão 1 — criação do projeto, nome escolhido, escopo inicial definido)

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

## O que é o Practice Valuation

App desktop pessoal para acompanhar teses de investimento em ações (B3) e criptoativos.
Substitui a ideia original de planilha (ver `docs/spec_automacao_dados.md`, histórico) por um app com banco de dados local.

**O que ele precisa fazer (visão do usuário, ainda sendo refinada):**
- Puxar o máximo de dados possível de fontes externas (fundamentos de ações BR, dados on-chain/mercado de cripto), com espaço pra ajuste manual quando necessário
- Guardar **múltiplos preços-teto/cálculos de valuation por ativo**, cada um com seu próprio conjunto de premissas (ex: duas projeções do mesmo ativo com taxas de crescimento diferentes, ambas salvas e comparáveis lado a lado)
- Cadastrar premissas por ativo (incluindo cripto) e monitorar indicadores automaticamente
- Avisar o usuário quando um ativo entrar em "zona de compra" segundo as premissas cadastradas
- Banco de dados **local** por enquanto — sync entre máquinas/nuvem é ideia pra mais adiante (ver Roadmap)

**Ainda não decidido** (a decidir junto, com calma — ver "Decisões de Arquitetura em Aberto"):
- Framework do app desktop
- Banco de dados local
- Visual/UI
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

## Fases Detalhadas

### Fase 0 — Fundamentos & Decisões de Arquitetura

**Objetivo**: decidir, com calma e com explicação de trade-offs, a stack do projeto antes de escrever código de verdade.

**Etapas**:
- [x] 0.1 — Nome do projeto → **Practice Valuation** (repo: `practice-valuation`), decidido na Sessão 1
- [ ] 0.2 — Escolher framework do app desktop (ver "Decisões de Arquitetura em Aberto")
- [ ] 0.3 — Escolher banco de dados local
- [ ] 0.4 — Escolher stack/lib de UI e uma direção visual inicial (bem simples, "planilha-like")
- [ ] 0.5 — Estrutura inicial do repositório (pastas, README, LICENSE, `.gitignore`)
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
- [ ] 2.1 — Decidir onde a coleta roda: dentro do próprio app vs. processo/serviço separado que escreve no banco local
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
- [ ] 4.5 — Direção visual (a decidir com o usuário — provavelmente algo denso em dados, tipo planilha, não um app "bonito" com muito espaço em branco)

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
| Framework do app desktop | Python (PySide6/Qt, Flet, etc.) vs Tauri (Rust+React/TS) vs Electron vs Flutter Desktop | Pendente — a discutir com trade-offs na próxima sessão |
| Banco de dados local | SQLite vs DuckDB | Pendente |
| Onde roda a coleta de dados | Dentro do próprio app vs. processo separado (herdado do desenho em `docs/spec_automacao_dados.md`) que escreve no banco local | Pendente |
| Sync entre dispositivos/nuvem | Adiado — ver Roadmap | Não é MVP |

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
- Próximo passo sugerido: decidir framework do app desktop (Fase 0.2) — Claude deve apresentar opções com trade-offs, não decidir sozinho

---

## Como Usar Este Arquivo

1. **Ao começar uma sessão**: diga ao Claude Code "leia o PROJECT_STATE.md e me ajude a continuar"
2. **Ao terminar uma sessão**: o Claude atualiza o Log de Sessões e marca etapas concluídas
3. **Ao tomar uma decisão**: registrar em "Decisões de Arquitetura em Aberto"
4. **Ao mudar de máquina**: sincronizar via git (`git init` já feito neste diretório)
