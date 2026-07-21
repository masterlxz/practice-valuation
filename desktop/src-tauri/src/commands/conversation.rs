use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
    Unchanged,
};
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::commands::ai_proposal::{self, ProposalPayload};
use crate::commands::chat::{generate_reply, AiOutcome, GeminiContent, GeminiPart};
use crate::entity::{ai_conversation, ai_message, ai_valuation_proposal};
use crate::error::AppError;

const DEFAULT_TITLE: &str = "New conversation";

#[derive(Serialize)]
pub struct ConversationSummary {
    pub id: i32,
    pub title: String,
    pub key_id: Option<i32>,
    pub model: String,
    pub updated_at: String,
}

impl From<ai_conversation::Model> for ConversationSummary {
    fn from(row: ai_conversation::Model) -> Self {
        ConversationSummary {
            id: row.id,
            title: row.title,
            key_id: row.key_id,
            model: row.model,
            updated_at: row.updated_at,
        }
    }
}

// Fase 7.10.4 — the frontend's tool-call preview card and approve/deny
// state both derive from this, not from parsing `ConversationMessage.content`
// as text. `payload`/`preview` are parsed here (not left as strings) so the
// frontend never needs to `JSON.parse` anything itself.
#[derive(Serialize)]
pub struct ValuationProposalSummary {
    pub id: i32,
    pub model: String,
    pub payload: JsonValue,
    pub preview: JsonValue,
    pub status: String,
    pub created_valuation_id: Option<i32>,
}

impl From<ai_valuation_proposal::Model> for ValuationProposalSummary {
    fn from(row: ai_valuation_proposal::Model) -> Self {
        ValuationProposalSummary {
            id: row.id,
            model: row.model,
            // A malformed stored payload/preview shouldn't 500 the whole
            // conversation view — fall back to `Null`, the frontend just
            // won't be able to render that one card's details.
            payload: serde_json::from_str(&row.payload).unwrap_or(JsonValue::Null),
            preview: serde_json::from_str(&row.preview).unwrap_or(JsonValue::Null),
            status: row.status,
            created_valuation_id: row.created_valuation_id,
        }
    }
}

#[derive(Serialize)]
pub struct ConversationMessage {
    pub id: i32,
    pub role: String,
    pub content: String,
    pub created_at: String,
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    pub proposal: Option<ValuationProposalSummary>,
}

impl ConversationMessage {
    pub(crate) fn from_row(row: ai_message::Model, proposal: Option<ai_valuation_proposal::Model>) -> Self {
        ConversationMessage {
            id: row.id,
            role: row.role,
            content: row.content,
            created_at: row.created_at,
            input_tokens: row.input_tokens,
            output_tokens: row.output_tokens,
            proposal: proposal.map(ValuationProposalSummary::from),
        }
    }
}

#[tauri::command]
pub async fn list_conversations(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<ConversationSummary>, AppError> {
    let rows = ai_conversation::Entity::find()
        .order_by_desc(ai_conversation::Column::UpdatedAt)
        .all(db.inner())
        .await?;

    Ok(rows.into_iter().map(ConversationSummary::from).collect())
}

// `key_id`/`model` start empty — the user picks a key the first time they
// send a message (same selector as the floating widget), `send_conversation_message`
// remembers it from then on.
#[tauri::command]
pub async fn create_conversation(
    db: tauri::State<'_, DatabaseConnection>,
    title: Option<String>,
) -> Result<ConversationSummary, AppError> {
    let now = chrono::Utc::now().to_rfc3339();
    let title = title
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| DEFAULT_TITLE.to_string());

    let row = ai_conversation::ActiveModel {
        title: Set(title),
        key_id: Set(None),
        model: Set(String::new()),
        created_at: Set(now.clone()),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db.inner())
    .await?;

    Ok(row.into())
}

#[tauri::command]
pub async fn rename_conversation(
    db: tauri::State<'_, DatabaseConnection>,
    id: i32,
    title: String,
) -> Result<(), AppError> {
    ai_conversation::Entity::find_by_id(id)
        .one(db.inner())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("conversation {id}")))?;

    ai_conversation::ActiveModel {
        id: Unchanged(id),
        title: Set(title),
        ..Default::default()
    }
    .update(db.inner())
    .await?;

    Ok(())
}

// Cascade (`ON DELETE CASCADE` on `ai_message.conversation_id`) cleans up the
// conversation's messages on its own — same idiom as `commands::valuation::delete_valuation`.
#[tauri::command]
pub async fn delete_conversation(
    db: tauri::State<'_, DatabaseConnection>,
    id: i32,
) -> Result<(), AppError> {
    ai_conversation::Entity::delete_by_id(id).exec(db.inner()).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_conversation_messages(
    db: tauri::State<'_, DatabaseConnection>,
    id: i32,
) -> Result<Vec<ConversationMessage>, AppError> {
    let rows = ai_message::Entity::find()
        .filter(ai_message::Column::ConversationId.eq(id))
        .order_by_asc(ai_message::Column::Id)
        .all(db.inner())
        .await?;

    let message_ids: Vec<i32> = rows.iter().map(|r| r.id).collect();
    let proposals = ai_valuation_proposal::Entity::find()
        .filter(ai_valuation_proposal::Column::MessageId.is_in(message_ids))
        .all(db.inner())
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let proposal = proposals.iter().find(|p| p.message_id == row.id).cloned();
            ConversationMessage::from_row(row, proposal)
        })
        .collect())
}

// Inserts the user's message, calls the same `generate_reply` the floating
// widget uses (Fase 7.10.2 is the first DB write the chat does), inserts the
// reply, and remembers the key/model used for next time this conversation is
// reopened. Fase 7.10.4: `generate_reply` is called with tool-calling turned
// on, so the reply can be either plain text or a `propose_valuation` call —
// see the `AiOutcome` match below.
#[tauri::command]
pub async fn send_conversation_message(
    db: tauri::State<'_, DatabaseConnection>,
    conversation_id: i32,
    key_id: i32,
    model: String,
    content: String,
) -> Result<ConversationMessage, AppError> {
    ai_message::ActiveModel {
        conversation_id: Set(conversation_id),
        role: Set("user".to_string()),
        content: Set(content),
        created_at: Set(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
    .insert(db.inner())
    .await?;

    let history_rows = ai_message::Entity::find()
        .filter(ai_message::Column::ConversationId.eq(conversation_id))
        .order_by_asc(ai_message::Column::Id)
        .all(db.inner())
        .await?;
    let history: Vec<GeminiContent> = history_rows
        .into_iter()
        .map(|row| GeminiContent {
            role: row.role,
            parts: vec![GeminiPart { text: Some(row.content), function_call: None }],
        })
        .collect();

    let (outcome, usage) = generate_reply(db.inner(), key_id, &model, history, true).await?;

    let (reply_row, proposal_row) = match outcome {
        AiOutcome::Text(text) => {
            let row = ai_message::ActiveModel {
                conversation_id: Set(conversation_id),
                role: Set("model".to_string()),
                content: Set(text),
                created_at: Set(chrono::Utc::now().to_rfc3339()),
                input_tokens: Set(Some(usage.input_tokens)),
                output_tokens: Set(Some(usage.output_tokens)),
                ..Default::default()
            }
            .insert(db.inner())
            .await?;
            (row, None)
        }
        AiOutcome::ToolCall { model: proposed_model, ticker, reference_year, current_price, inputs } => {
            match ai_proposal::validate_and_preview(&proposed_model, &inputs, current_price) {
                Ok((fair_price, safety_margin, verdict)) => {
                    // No JSON-cru in `content` — the frontend derives the
                    // real preview card from the joined `ai_valuation_proposal`
                    // row (see `ValuationProposalSummary`), not from parsing
                    // this text. It's only a fallback label.
                    let placeholder_row = ai_message::ActiveModel {
                        conversation_id: Set(conversation_id),
                        role: Set("model".to_string()),
                        content: Set(format!(
                            "Proposta de valuation: {ticker} ({proposed_model}) — preço justo R$ {fair_price:.2}, margem {:.1}%, {verdict}.",
                            safety_margin * 100.0
                        )),
                        created_at: Set(chrono::Utc::now().to_rfc3339()),
                        input_tokens: Set(Some(usage.input_tokens)),
                        output_tokens: Set(Some(usage.output_tokens)),
                        ..Default::default()
                    }
                    .insert(db.inner())
                    .await?;

                    let payload = serde_json::to_string(&ProposalPayload {
                        ticker,
                        reference_year,
                        current_price,
                        inputs,
                    })
                    .map_err(|e| AppError::InvalidInput(e.to_string()))?;
                    let preview = serde_json::json!({
                        "fair_price": fair_price,
                        "safety_margin": safety_margin,
                        "verdict": verdict,
                    })
                    .to_string();

                    let proposal_row = ai_valuation_proposal::ActiveModel {
                        message_id: Set(placeholder_row.id),
                        model: Set(proposed_model),
                        payload: Set(payload),
                        preview: Set(preview),
                        status: Set("pending".to_string()),
                        created_valuation_id: Set(None),
                        created_at: Set(chrono::Utc::now().to_rfc3339()),
                        ..Default::default()
                    }
                    .insert(db.inner())
                    .await?;

                    (placeholder_row, Some(proposal_row))
                }
                Err(err) => {
                    // Dead end, no retry (deliberate Fase 7.10.4 simplicity
                    // choice): just surface the error, no proposal row.
                    let row = ai_message::ActiveModel {
                        conversation_id: Set(conversation_id),
                        role: Set("model".to_string()),
                        content: Set(format!(
                            "A IA tentou propor uma valuation mas os dados são inválidos: {err}"
                        )),
                        created_at: Set(chrono::Utc::now().to_rfc3339()),
                        input_tokens: Set(Some(usage.input_tokens)),
                        output_tokens: Set(Some(usage.output_tokens)),
                        ..Default::default()
                    }
                    .insert(db.inner())
                    .await?;
                    (row, None)
                }
            }
        }
    };

    ai_conversation::ActiveModel {
        id: Unchanged(conversation_id),
        key_id: Set(Some(key_id)),
        model: Set(model),
        updated_at: Set(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
    .update(db.inner())
    .await?;

    Ok(ConversationMessage::from_row(reply_row, proposal_row))
}
