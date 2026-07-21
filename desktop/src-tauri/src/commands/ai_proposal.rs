use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, Unchanged};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::commands::conversation::ConversationMessage;
use crate::domain::{banks, bazin, dcf, gordon, graham, projected_ceiling, rim, rnav};
use crate::entity::{
    ai_message, ai_valuation_proposal, banks_inputs, bazin_inputs, dcf_inputs, gordon_inputs,
    graham_inputs, projected_ceiling_inputs, rim_inputs, rnav_inputs, valuation,
};
use crate::error::AppError;

// Serialized into `ai_valuation_proposal.payload` at proposal time (Fase
// 7.10.4) and deserialized back at approval time — internal plumbing, never
// a Tauri command argument, so it stays crate-private.
#[derive(Deserialize, Serialize)]
pub(crate) struct ProposalPayload {
    pub ticker: String,
    pub reference_year: i32,
    pub current_price: f64,
    pub inputs: JsonValue,
}

// Shared by proposal-time validation (`validate_and_preview`, called from
// `send_conversation_message`) and approval-time write (`insert_valuation`).
// Mirrors the per-model match in `commands::valuation::update_valuation`,
// but this one never touches an existing row.
fn calculate_outcome(
    model: &str,
    inputs: &JsonValue,
    current_price: f64,
) -> Result<(f64, f64, String), AppError> {
    let invalid = |err: serde_json::Error| AppError::InvalidInput(err.to_string());

    let (fair_price, safety_margin, verdict) = match model {
        "bazin" => {
            let parsed: bazin::BazinInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = bazin::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        "graham" => {
            let parsed: graham::GrahamInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = graham::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        "gordon" => {
            let parsed: gordon::GordonInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = gordon::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        "dcf" => {
            let parsed: dcf::DcfInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = dcf::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        "banks" => {
            let parsed: banks::BanksInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = banks::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        "rim" => {
            let parsed: rim::RimInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = rim::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        "rnav" => {
            let parsed: rnav::RnavInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = rnav::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        "projected_ceiling" => {
            let parsed: projected_ceiling::ProjectedCeilingInputs =
                serde_json::from_value(inputs.clone()).map_err(invalid)?;
            let outcome = projected_ceiling::calculate(&parsed, current_price)?;
            (outcome.fair_price, outcome.safety_margin, outcome.verdict.as_str().to_string())
        }
        other => return Err(AppError::NotFound(format!("model '{other}'"))),
    };

    Ok((fair_price, safety_margin, verdict))
}

// Called once, at proposal time, from `send_conversation_message` — never a
// `#[tauri::command]` itself. Failure here (bad `inputs` shape, or a
// `domain::*::calculate` guard like Ke <= g) is the "dead end, no retry"
// path: the caller turns the `Err` into a plain error text message and
// inserts no proposal row at all.
pub(crate) fn validate_and_preview(
    model: &str,
    inputs: &JsonValue,
    current_price: f64,
) -> Result<(f64, f64, String), AppError> {
    calculate_outcome(model, inputs, current_price)
}

// The 8-armed write — the only place in this feature that actually calls
// `.insert()` against `valuation`/`<model>_inputs`. Only reached from
// `respond_to_valuation_proposal` after `approved == true`. Each arm mirrors
// the insert block of the corresponding `calculate_<model>` command.
async fn insert_valuation(
    db: &DatabaseConnection,
    model: &str,
    ticker: String,
    reference_year: i32,
    current_price: f64,
    inputs: &JsonValue,
) -> Result<valuation::Model, AppError> {
    let invalid = |err: serde_json::Error| AppError::InvalidInput(err.to_string());
    let (fair_price, safety_margin, verdict) = calculate_outcome(model, inputs, current_price)?;

    let valuation_row = valuation::ActiveModel {
        ticker: Set(ticker),
        reference_year: Set(reference_year),
        current_price: Set(current_price),
        model: Set(model.to_string()),
        fair_price: Set(Some(fair_price)),
        safety_margin: Set(Some(safety_margin)),
        verdict: Set(Some(verdict)),
        updated_at: Set(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    match model {
        "bazin" => {
            let parsed: bazin::BazinInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            bazin_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                average_dividend: Set(parsed.average_dividend),
                desired_yield: Set(parsed.desired_yield),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        "graham" => {
            let parsed: graham::GrahamInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            graham_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                eps: Set(parsed.eps),
                book_value_per_share: Set(parsed.book_value_per_share),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        "gordon" => {
            let parsed: gordon::GordonInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            gordon_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                current_dividend: Set(parsed.current_dividend),
                expected_growth: Set(parsed.expected_growth),
                ke: Set(parsed.ke),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        "dcf" => {
            let parsed: dcf::DcfInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            dcf_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                ebit: Set(parsed.ebit),
                tax_rate: Set(parsed.tax_rate),
                depreciation_amortization: Set(parsed.depreciation_amortization),
                capex: Set(parsed.capex),
                nwc_change: Set(parsed.nwc_change),
                total_debt: Set(parsed.total_debt),
                cash: Set(parsed.cash),
                shares_outstanding: Set(parsed.shares_outstanding),
                beta: Set(parsed.beta),
                risk_free_rate: Set(parsed.risk_free_rate),
                market_risk_premium: Set(parsed.market_risk_premium),
                kd: Set(parsed.kd),
                perpetuity_growth: Set(parsed.perpetuity_growth),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        "banks" => {
            let parsed: banks::BanksInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            banks_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                book_value_per_share: Set(parsed.book_value_per_share),
                roe: Set(parsed.roe),
                payout: Set(parsed.payout),
                ke: Set(parsed.ke),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        "rim" => {
            let parsed: rim::RimInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            rim_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                book_value_per_share: Set(parsed.book_value_per_share),
                roe_current: Set(parsed.roe_current),
                payout: Set(parsed.payout),
                ke: Set(parsed.ke),
                fade_years: Set(parsed.fade_years),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        "rnav" => {
            let parsed: rnav::RnavInputs = serde_json::from_value(inputs.clone()).map_err(invalid)?;
            rnav_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                landbank: Set(parsed.landbank),
                inventory_at_market_value: Set(parsed.inventory_at_market_value),
                net_cash: Set(parsed.net_cash),
                shares_outstanding: Set(parsed.shares_outstanding),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        "projected_ceiling" => {
            let parsed: projected_ceiling::ProjectedCeilingInputs =
                serde_json::from_value(inputs.clone()).map_err(invalid)?;
            projected_ceiling_inputs::ActiveModel {
                valuation_id: Set(valuation_row.id),
                current_dividend: Set(parsed.current_dividend),
                expected_growth: Set(parsed.expected_growth),
                projection_years: Set(parsed.projection_years),
                desired_yield: Set(parsed.desired_yield),
                ke: Set(parsed.ke),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
        other => return Err(AppError::NotFound(format!("model '{other}'"))),
    }

    Ok(valuation_row)
}

#[tauri::command]
pub async fn respond_to_valuation_proposal(
    db: tauri::State<'_, DatabaseConnection>,
    proposal_id: i32,
    approved: bool,
) -> Result<ConversationMessage, AppError> {
    let proposal = ai_valuation_proposal::Entity::find_by_id(proposal_id)
        .one(db.inner())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("proposal {proposal_id}")))?;

    if proposal.status != "pending" {
        return Err(AppError::InvalidInput(format!(
            "proposal {proposal_id} already resolved ({})",
            proposal.status
        )));
    }

    let conversation_id = ai_message::Entity::find_by_id(proposal.message_id)
        .one(db.inner())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("message {}", proposal.message_id)))?
        .conversation_id;

    let (new_status, created_valuation_id, synthetic_text) = if approved {
        let payload: ProposalPayload =
            serde_json::from_str(&proposal.payload).map_err(|e| AppError::InvalidInput(e.to_string()))?;
        let created = insert_valuation(
            db.inner(),
            &proposal.model,
            payload.ticker.clone(),
            payload.reference_year,
            payload.current_price,
            &payload.inputs,
        )
        .await?;
        (
            "approved".to_string(),
            Some(created.id),
            format!(
                "✅ Valuation criada: {} ({}) — preço justo R$ {:.2}, margem {:.1}%, {}.",
                created.ticker,
                proposal.model,
                created.fair_price.unwrap_or_default(),
                created.safety_margin.unwrap_or_default() * 100.0,
                created.verdict.clone().unwrap_or_default(),
            ),
        )
    } else {
        ("rejected".to_string(), None, "❌ Proposta descartada.".to_string())
    };

    ai_valuation_proposal::ActiveModel {
        id: Unchanged(proposal.id),
        status: Set(new_status),
        created_valuation_id: Set(created_valuation_id),
        ..Default::default()
    }
    .update(db.inner())
    .await?;

    let synthetic_row = ai_message::ActiveModel {
        conversation_id: Set(conversation_id),
        role: Set("model".to_string()),
        content: Set(synthetic_text),
        created_at: Set(chrono::Utc::now().to_rfc3339()),
        input_tokens: Set(None),
        output_tokens: Set(None),
        ..Default::default()
    }
    .insert(db.inner())
    .await?;

    Ok(ConversationMessage::from_row(synthetic_row, None))
}
