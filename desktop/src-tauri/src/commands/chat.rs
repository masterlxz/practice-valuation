use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::commands::api_key::read_api_key_secret;
use crate::domain::chat_provider::Provider;
use crate::entity::{alert_event, valuation};
use crate::error::AppError;

// Hand-written, not generated from the domain modules — this is a compact
// briefing for the model, not a spec. Full formulas/guards live in
// PROJECT_STATE.md (Fase 3) if this ever needs updating alongside the models.
const SYSTEM_REPERTOIRE: &str = "\
Você é o assistente de IA embutido no Practice Valuation, um app de desktop pessoal pra acompanhar teses de investimento em ações da B3 e em cripto (Ethereum). Você tem acesso só de leitura ao banco do usuário (nunca cria, edita ou apaga nada) — os dados relevantes (valuations salvas e eventos de alerta recentes) vêm anexados depois deste texto.

Regra geral de todo modelo de ação: margem_seguranca = (preco_justo - preco_atual) / preco_justo; veredito é BARATO se margem_seguranca > 0, senão CARO.

Os 8 modelos de valuation de ação disponíveis (campo `model` na valuation salva):
1. dcf (DCF/FCFF) — empresas 'normais' (varejo, indústria, tech, utilities), não serve pra banco/incorporadora. FCFF = EBIT×(1-IR) + D&A - Capex - ΔNWC; desconta a WACC, cresce a taxa g na perpetuidade.
2. gordon (Gordon/DDM) — boa pagadora de dividendo com crescimento previsível. Preco_Justo = D0×(1+g) / (Ke-g).
3. bazin — 'vaca leiteira' (bancão, elétrica, saneamento), foco em yield. Preco_Teto = Dividendo_Médio_5a / Yield_Desejado.
4. graham (Graham Number) — filtro rápido de margem de segurança, qualquer empresa com lucro e PL positivos. Graham_Number = RAIZ(22.5 × LPA × VPA).
5. banks (P/B via ROE-Gordon) — instituições financeiras, onde dívida é matéria-prima do negócio, não alavancagem a evitar. P/B_Justo = (ROE - g_sustentável) / (Ke - g_sustentável), g_sustentável = ROE×(1-Payout).
6. rnav — construtoras/incorporadoras, o 'estoque' é imóvel. RNAV/Ação = (Landbank + Estoque + Caixa_Líquido) / Nº_Ações.
7. projected_ceiling (Preço Teto Projetivo) — como o Bazin, mas projeta N anos de crescimento do dividendo e traz a valor presente.
8. rim (RIM — Lucro Residual) — generaliza o banks pra bancos/financeiras: projeta o ROE convergindo (fade linear) do valor atual até o próprio Ke ao longo de N anos, desconta o lucro residual [(ROE_t - Ke) × VPA_(t-1)] ano a ano a Ke. Preco_Justo = VPA0 + Σ VP(LucroResidual_t). Sem valor terminal a somar (em t=N o ROE já é Ke, lucro residual dali em diante é zero por construção). Quando ROE_atual = Ke, o preço justo bate exatamente com o valor patrimonial (mesmo caso particular do banks).

Score de cripto (Ethereum, score contínuo, atualizado ao longo do tempo, não anual como as ações): 9 indicadores on-chain/mercado (MVRV Z-Score, NVT Ratio, Puell Multiple, Emissão Líquida, Staking Yield Líquido, TVL DeFi, Endereços Ativos/Transações, Exchange Netflow, Fees de Rede vs Emissão), cada um classificado GREEN/NEUTRAL/RED contra dois limiares configuráveis. Score final = quantos indicadores estão GREEN de 9. Leitura sugerida: 7-9 verdes = tese intacta (manter/aportar); 4-6 = neutro (observar de perto); 0-3 = considerar reduzir risco/posição.

Responda de forma direta e concisa, sempre baseado nos dados reais fornecidos abaixo. Se a pergunta não puder ser respondida com os dados disponíveis, diga isso claramente em vez de inventar números.\
";

const RECENT_ALERT_EVENTS_LIMIT: u64 = 20;

fn format_context(valuations: &[valuation::Model], events: &[alert_event::Model]) -> String {
    let valuations_block = if valuations.is_empty() {
        "Nenhuma valuation salva ainda.".to_string()
    } else {
        valuations
            .iter()
            .map(|v| {
                format!(
                    "- {} ({}, ano-ref {}, atualizado em {}): preço atual R$ {:.2}, preço justo {}, margem de segurança {}, veredito {}",
                    v.ticker,
                    v.model,
                    v.reference_year,
                    v.updated_at,
                    v.current_price,
                    v.fair_price
                        .map(|p| format!("R$ {p:.2}"))
                        .unwrap_or_else(|| "N/D".to_string()),
                    v.safety_margin
                        .map(|m| format!("{:.1}%", m * 100.0))
                        .unwrap_or_else(|| "N/D".to_string()),
                    v.verdict.as_deref().unwrap_or("N/D"),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let events_block = if events.is_empty() {
        "Nenhum evento de alerta registrado ainda.".to_string()
    } else {
        events
            .iter()
            .map(|e| {
                format!(
                    "- [{}] {} (disparado: {})",
                    e.created_at,
                    e.message,
                    if e.is_triggered { "sim" } else { "não" },
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "{SYSTEM_REPERTOIRE}\n\n## Valuations salvas\n{valuations_block}\n\n## Eventos de alerta recentes (até {RECENT_ALERT_EVENTS_LIMIT})\n{events_block}"
    )
}

async fn build_system_instruction(db: &DatabaseConnection) -> Result<String, AppError> {
    let valuations = valuation::Entity::find()
        .order_by_desc(valuation::Column::UpdatedAt)
        .all(db)
        .await?;

    let events = alert_event::Entity::find()
        .order_by_desc(alert_event::Column::CreatedAt)
        .limit(RECENT_ALERT_EVENTS_LIMIT)
        .all(db)
        .await?;

    Ok(format_context(&valuations, &events))
}

// Mirrors the Gemini REST API shape 1:1 (contents/parts/role) so the same
// struct can grow into full chat history in Fase 7.4 without changing shape.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Serialize)]
struct GeminiRequestBody<'a> {
    system_instruction: &'a GeminiSystemInstruction,
    contents: &'a [GeminiContent],
}

#[derive(Serialize)]
struct GeminiSystemInstruction {
    parts: Vec<GeminiPart>,
}

#[derive(Deserialize)]
struct GeminiResponseBody {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

async fn ask_gemini_api(
    api_key: &str,
    model: &str,
    system_instruction_text: String,
    history: Vec<GeminiContent>,
) -> Result<String, AppError> {
    let system_instruction = GeminiSystemInstruction {
        parts: vec![GeminiPart {
            text: system_instruction_text,
        }],
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}"
    );
    let body = GeminiRequestBody {
        system_instruction: &system_instruction,
        contents: &history,
    };

    let client = reqwest::Client::new();
    let response = client.post(&url).json(&body).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(AppError::GeminiApi(format!("{status}: {error_body}")));
    }

    let parsed: GeminiResponseBody = response.json().await?;
    parsed
        .candidates
        .into_iter()
        .next()
        .and_then(|candidate| candidate.content.parts.into_iter().next())
        .map(|part| part.text)
        .ok_or_else(|| AppError::GeminiApi("empty response from Gemini".to_string()))
}

const CLAUDE_API_VERSION: &str = "2023-06-01";
// Chat replies here are short answers about already-saved data, not long-form
// generation — same reasoning as picking a "flash-lite"-tier Gemini model.
const CLAUDE_MAX_TOKENS: u32 = 4096;

// Reused by both Claude and OpenAI — both map Gemini's "model" role to
// "assistant" and treat anything else (only ever "user" here) as "user".
fn gemini_role_to_assistant_style(role: &str) -> &'static str {
    if role == "model" {
        "assistant"
    } else {
        "user"
    }
}

fn join_parts(parts: &[GeminiPart]) -> String {
    parts.iter().map(|part| part.text.as_str()).collect::<Vec<_>>().join("")
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct ClaudeRequestBody<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<ClaudeMessage>,
}

#[derive(Deserialize)]
struct ClaudeResponseBody {
    content: Vec<ClaudeContentBlock>,
}

#[derive(Deserialize)]
struct ClaudeContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}

async fn ask_claude_api(
    api_key: &str,
    model: &str,
    system_instruction: String,
    history: Vec<GeminiContent>,
) -> Result<String, AppError> {
    let messages = history
        .iter()
        .map(|content| ClaudeMessage {
            role: gemini_role_to_assistant_style(&content.role),
            content: join_parts(&content.parts),
        })
        .collect();

    let body = ClaudeRequestBody {
        model,
        max_tokens: CLAUDE_MAX_TOKENS,
        system: &system_instruction,
        messages,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", CLAUDE_API_VERSION)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(AppError::ClaudeApi(format!("{status}: {error_body}")));
    }

    let parsed: ClaudeResponseBody = response.json().await?;
    parsed
        .content
        .into_iter()
        .find(|block| block.block_type == "text")
        .and_then(|block| block.text)
        .ok_or_else(|| AppError::ClaudeApi("empty response from Claude".to_string()))
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct OpenAiRequestBody<'a> {
    model: &'a str,
    messages: Vec<OpenAiMessage>,
}

#[derive(Deserialize)]
struct OpenAiResponseBody {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiResponseMessage,
}

#[derive(Deserialize)]
struct OpenAiResponseMessage {
    content: String,
}

async fn ask_openai_api(
    api_key: &str,
    model: &str,
    system_instruction: String,
    history: Vec<GeminiContent>,
) -> Result<String, AppError> {
    // OpenAI has no separate "system" field — it's just the first message.
    let mut messages = vec![OpenAiMessage {
        role: "system",
        content: system_instruction,
    }];
    messages.extend(history.iter().map(|content| OpenAiMessage {
        role: gemini_role_to_assistant_style(&content.role),
        content: join_parts(&content.parts),
    }));

    let body = OpenAiRequestBody { model, messages };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(AppError::OpenAiApi(format!("{status}: {error_body}")));
    }

    let parsed: OpenAiResponseBody = response.json().await?;
    parsed
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .ok_or_else(|| AppError::OpenAiApi("empty response from OpenAI".to_string()))
}

// Shared by `ask_ai` (floating widget) and `commands::conversation`'s saved
// conversations (Fase 7.10.2) — resolves the key, builds the read-only DB
// context, and dispatches to the right provider. Kept as a plain function
// (not a command) so both entry points stay in sync with `SYSTEM_REPERTOIRE`
// instead of duplicating this logic.
pub async fn generate_reply(
    db: &DatabaseConnection,
    key_id: i32,
    model: &str,
    history: Vec<GeminiContent>,
) -> Result<String, AppError> {
    let (provider, api_key) = read_api_key_secret(db, key_id).await?;
    let system_instruction = build_system_instruction(db).await?;
    match provider {
        Provider::Gemini => ask_gemini_api(&api_key, model, system_instruction, history).await,
        Provider::Claude => ask_claude_api(&api_key, model, system_instruction, history).await,
        Provider::OpenAi => ask_openai_api(&api_key, model, system_instruction, history).await,
    }
}

// Provider-generic entry point for the floating chat widget. `key_id` (Fase
// 7.9.2/7.9.3 — one of possibly several named keys per provider) replaces the
// old plain `provider` string; the provider itself is now derived from the
// chosen key's row instead of being picked by the frontend directly.
#[tauri::command]
pub async fn ask_ai(
    key_id: i32,
    model: String,
    db: tauri::State<'_, DatabaseConnection>,
    history: Vec<GeminiContent>,
) -> Result<String, AppError> {
    generate_reply(db.inner(), key_id, &model, history).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_valuation() -> valuation::Model {
        valuation::Model {
            id: 1,
            ticker: "BBAS3".to_string(),
            reference_year: 2025,
            current_price: 25.0,
            model: "bazin".to_string(),
            fair_price: Some(30.0),
            safety_margin: Some(0.1667),
            verdict: Some("BARATO".to_string()),
            updated_at: "2026-07-12T10:00:00-03:00".to_string(),
        }
    }

    fn sample_event() -> alert_event::Model {
        alert_event::Model {
            id: 1,
            alert_rule_id: 1,
            is_triggered: true,
            message: "BBAS3 price 25.00 now crosses fair price 30.00".to_string(),
            created_at: "2026-07-12T10:05:00-03:00".to_string(),
        }
    }

    #[test]
    fn format_context_reports_empty_sections_explicitly() {
        let text = format_context(&[], &[]);
        assert!(text.contains("Nenhuma valuation salva ainda."));
        assert!(text.contains("Nenhum evento de alerta registrado ainda."));
        assert!(text.starts_with(SYSTEM_REPERTOIRE));
    }

    #[test]
    fn format_context_includes_valuation_and_event_data() {
        let text = format_context(&[sample_valuation()], &[sample_event()]);
        assert!(text.contains("BBAS3 (bazin, ano-ref 2025"));
        assert!(text.contains("preço justo R$ 30.00"));
        assert!(text.contains("margem de segurança 16.7%"));
        assert!(text.contains("veredito BARATO"));
        assert!(text.contains("now crosses fair price 30.00"));
        assert!(text.contains("(disparado: sim)"));
    }

    #[test]
    fn format_context_falls_back_to_nd_for_missing_fields() {
        let mut incomplete = sample_valuation();
        incomplete.fair_price = None;
        incomplete.safety_margin = None;
        incomplete.verdict = None;

        let text = format_context(&[incomplete], &[]);
        assert!(text.contains("preço justo N/D"));
        assert!(text.contains("margem de segurança N/D"));
        assert!(text.contains("veredito N/D"));
    }
}
