# Spec Funcional — Modelos de Valuation e Score Cripto (pro app desktop)

Objetivo deste documento: descrever, modelo por modelo, quais campos o app
precisa salvar (inputs), o que ele calcula sozinho, e como chegar no
resultado final (Preço Justo/Teto, Margem de Segurança, Veredito). É a
"tradução" do que as abas da planilha já fazem, pro Claude Code implementar
no app local.

## Regra geral (vale pra TODOS os modelos de ação)

Todo modelo de ação tem essa mesma "cauda final", independente da fórmula:

```
Margem de Segurança = (Preço Justo/Teto − Preço Atual) / Preço Justo/Teto
Veredito = "BARATO" se Margem > 0, senão "CARO"
```

E todo modelo carrega 3 campos fixos, além dos específicos:
- **Ticker** (texto, ex: FIQE3)
- **Ano Ref.** (inteiro, ano-base dos dados usados) → o app deve calcular
  `anos_desatualizado = ano_atual − ano_ref` e sinalizar: `<=0` → em dia,
  `==1` → atenção, `>=2` → desatualizado. É o campo que te avisa quando
  revisar aquela empresa.
- **Preço Atual** (decimal, R$) → pode ser puxado de API (brapi/bolsai) ou
  digitado manual como fallback.

---

## 1. DCF / FCFF (empresas "normais")

**Quando usar:** empresa com capital de giro e capex previsíveis (varejo,
indústria, tech, utilities). Não usar em banco ou incorporadora.

**Inputs que o app precisa salvar:**
| Campo | Tipo/Unidade |
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

**Cálculo:**
```
FCFF        = EBIT × (1 − IR) + D&A − Capex − ΔNWC
Ke (CAPM)   = Rf + Beta × Prêmio_Risco_Mercado
E (equity)  = Preço_Atual × Nº_Ações
WACC        = [E / (E + Dívida)] × Ke + [Dívida / (E + Dívida)] × Kd × (1 − IR)
Valor_Firma = FCFF × (1 + g) / (WACC − g)
Valor_Equity= Valor_Firma − Dívida_Total + Caixa
Preço_Justo = Valor_Equity / Nº_Ações
```
Depois aplica a regra geral de Margem/Veredito.

⚠️ Guarda de erro: se `(WACC − g) <= 0`, não calcular (modelo quebra
matematicamente) — mostrar aviso em vez de número.

---

## 2. Gordon / DDM (Dividend Discount Model)

**Quando usar:** boa pagadora de dividendo, crescimento previsível.

**Inputs:**
| Campo | Tipo/Unidade |
|---|---|
| Dividendo Atual (D0) | R$/ação |
| Crescimento Esperado dos Dividendos (g) | % |
| Ke (retorno exigido) | % |

**Cálculo:**
```
D1          = D0 × (1 + g)
Preço_Justo = D1 / (Ke − g)
```
Guarda: `Ke > g`, senão inválido.

---

## 3. Bazin

**Quando usar:** "vaca leiteira" (bancão, elétrica, saneamento), foco em
yield de dividendo.

**Inputs:**
| Campo | Tipo/Unidade |
|---|---|
| Dividendo Médio por Ação (últimos 5 anos) | R$/ação |
| Yield Desejado | % (default sugerido: 6%) |

**Cálculo:**
```
Preço_Teto = Dividendo_Médio / Yield_Desejado
```

---

## 4. Graham (Graham Number)

**Quando usar:** filtro rápido de margem de segurança, qualquer empresa com
lucro e patrimônio positivos.

**Inputs:**
| Campo | Tipo/Unidade |
|---|---|
| LPA (Lucro por Ação) | R$/ação |
| VPA (Valor Patrimonial por Ação) | R$/ação |

**Cálculo:**
```
Graham_Number = RAIZ(22.5 × LPA × VPA)
```
Guarda: se LPA <= 0 ou VPA <= 0, não calcular (empresa com prejuízo ou PL
negativo não se encaixa nesse método).

---

## 5. Bancos (P/B via ROE-Gordon)

**Quando usar:** bancos e instituições financeiras — FCFF não serve porque
dívida é matéria-prima do negócio, não uma alavancagem a evitar.

**Inputs:**
| Campo | Tipo/Unidade |
|---|---|
| VPA (Valor Patrimonial por Ação) | R$/ação |
| ROE | % |
| Payout | % |
| Ke (retorno exigido) | % |

**Cálculo:**
```
g_sustentável = ROE × (1 − Payout)
P/B_Justo     = (ROE − g_sustentável) / (Ke − g_sustentável)
Preço_Justo   = P/B_Justo × VPA
```
Guarda: `Ke > g_sustentável`.

---

## 6. Incorporadoras (RNAV)

**Quando usar:** construtoras/incorporadoras — o "estoque" é imóvel, não dá
pra projetar FCFF de forma suave trimestre a trimestre.

**Inputs:**
| Campo | Tipo/Unidade |
|---|---|
| Landbank a Valor de Mercado | R$ milhões |
| Estoque a Valor de Mercado | R$ milhões |
| Caixa Líquido (caixa − dívida, pode ser negativo) | R$ milhões |
| Nº de Ações | milhões |

**Cálculo:**
```
RNAV_Total = Landbank + Estoque + Caixa_Líquido
RNAV/Ação  = RNAV_Total / Nº_Ações
```
(RNAV/Ação entra no lugar de "Preço Justo" na regra geral.)

---

## 7. Preço Teto Projetivo

**Quando usar:** mesma lógica do Bazin, mas trazendo N anos de crescimento
esperado pra frente e descontando a valor presente — útil quando você quer
o teto "olhando pra frente", não só o dividendo de hoje.

**Inputs:**
| Campo | Tipo/Unidade |
|---|---|
| Dividendo Atual (D0) | R$/ação |
| Crescimento Esperado (g) | % |
| Anos de Projeção (N) | inteiro (default sugerido: 5) |
| Yield Desejado (alvo, estilo Bazin) | % (default sugerido: 6%) |
| Ke (taxa de desconto) | % |

**Cálculo:**
```
Dividendo_Projetado_N   = D0 × (1 + g)^N
Preço_Teto_Futuro_N     = Dividendo_Projetado_N / Yield_Desejado
Preço_Teto_Projetivo    = Preço_Teto_Futuro_N / (1 + Ke)^N
```
(`Preço_Teto_Projetivo` entra como "Preço Justo/Teto" na regra geral.)

---

## Persistência sugerida (banco local do app)

Uma tabela por modelo é a forma mais simples (ex: SQLite), todas com o
padrão: `ticker`, `ano_ref`, `preco_atual`, campos específicos do modelo,
`preco_justo` (calculado, mas vale salvar em cache), `margem_seguranca`,
`veredito`, `data_ultima_atualizacao`. Isso permite o app mostrar histórico
("como essa margem evoluiu ano a ano") sem recalcular tudo toda vez.

---

# Parte 2 — Score de Cripto (Ethereum)

Diferente de ação (1x/ano), aqui é um **score contínuo**: cada indicador
vira verde (bom pra compra/manter) ou vermelho (sinal de reduzir risco), e
o app soma quantos estão verdes de um total de 9. A ideia (igual você
descreveu) é: quando indicadores começam a virar vermelho, é hora de
considerar reduzir posição — não depende de "vibe", é contagem objetiva.

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

**Score final:** `verdes / 9`. Sugestão de leitura (ajuste como quiser
depois de ver rodando um tempo):
- **7-9 verdes** → tese intacta, manter/aportar
- **4-6 verdes** → neutro, observar de perto
- **0-3 verdes** → considerar reduzir risco/posição

**Persistência sugerida:** uma tabela `cripto_indicadores` com
`moeda`, `data`, `indicador`, `valor_bruto`, `sinal` (verde/vermelho),
`fonte`. Isso dá histórico de série temporal — dá pra plotar a evolução do
score ao longo do tempo, não só o snapshot do dia.

**Nota importante pro Claude Code implementar:** os thresholds acima (tipo
"< 0", "> 7", "> 2%") são ponto de partida razoável baseado em uso histórico
de mercado, não são uma regra imutável — o app deveria deixar esses números
configuráveis (não hardcoded), porque o Fabio provavelmente vai querer
calibrar depois de ver como cada indicador se comporta na prática.
