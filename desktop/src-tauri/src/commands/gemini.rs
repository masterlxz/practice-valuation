use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::commands::api_key::KEYRING_SERVICE;
use crate::entity::{alert_event, valuation};
use crate::error::AppError;

const GEMINI_PROVIDER: &str = "gemini";
const GEMINI_MODEL: &str = "gemini-3.1-flash-lite";

// Hand-written, not generated from the domain modules — this is a compact
// briefing for the model, not a spec. Full formulas/guards live in
// PROJECT_STATE.md (Fase 3) if this ever needs updating alongside the models.
const SYSTEM_REPERTOIRE: &str = "\
Você é o assistente de IA embutido no Practice Valuation, um app de desktop pessoal pra acompanhar teses de investimento em ações da B3 e em cripto (Ethereum). Você tem acesso só de leitura ao banco do usuário (nunca cria, edita ou apaga nada) — os dados relevantes (valuations salvas e eventos de alerta recentes) vêm anexados depois deste texto.

Regra geral de todo modelo de ação: margem_seguranca = (preco_justo - preco_atual) / preco_justo; veredito é BARATO se margem_seguranca > 0, senão CARO.

Os 7 modelos de valuation de ação disponíveis (campo `model` na valuation salva):
1. dcf (DCF/FCFF) — empresas 'normais' (varejo, indústria, tech, utilities), não serve pra banco/incorporadora. FCFF = EBIT×(1-IR) + D&A - Capex - ΔNWC; desconta a WACC, cresce a taxa g na perpetuidade.
2. gordon (Gordon/DDM) — boa pagadora de dividendo com crescimento previsível. Preco_Justo = D0×(1+g) / (Ke-g).
3. bazin — 'vaca leiteira' (bancão, elétrica, saneamento), foco em yield. Preco_Teto = Dividendo_Médio_5a / Yield_Desejado.
4. graham (Graham Number) — filtro rápido de margem de segurança, qualquer empresa com lucro e PL positivos. Graham_Number = RAIZ(22.5 × LPA × VPA).
5. banks (P/B via ROE-Gordon) — instituições financeiras, onde dívida é matéria-prima do negócio, não alavancagem a evitar. P/B_Justo = (ROE - g_sustentável) / (Ke - g_sustentável), g_sustentável = ROE×(1-Payout).
6. rnav — construtoras/incorporadoras, o 'estoque' é imóvel. RNAV/Ação = (Landbank + Estoque + Caixa_Líquido) / Nº_Ações.
7. projected_ceiling (Preço Teto Projetivo) — como o Bazin, mas projeta N anos de crescimento do dividendo e traz a valor presente.

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

fn read_gemini_api_key() -> Result<String, AppError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, GEMINI_PROVIDER)?;
    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => Err(AppError::MissingApiKey(GEMINI_PROVIDER.to_string())),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
pub async fn ask_gemini(
    db: tauri::State<'_, DatabaseConnection>,
    history: Vec<GeminiContent>,
) -> Result<String, AppError> {
    let api_key = read_gemini_api_key()?;
    let system_instruction = GeminiSystemInstruction {
        parts: vec![GeminiPart {
            text: build_system_instruction(db.inner()).await?,
        }],
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{GEMINI_MODEL}:generateContent?key={api_key}"
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
