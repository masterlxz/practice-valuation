use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
    Unchanged,
};
use serde::Serialize;

use crate::commands::chat::{generate_reply, GeminiContent, GeminiPart};
use crate::entity::{ai_conversation, ai_message};
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

#[derive(Serialize)]
pub struct ConversationMessage {
    pub id: i32,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

impl From<ai_message::Model> for ConversationMessage {
    fn from(row: ai_message::Model) -> Self {
        ConversationMessage {
            id: row.id,
            role: row.role,
            content: row.content,
            created_at: row.created_at,
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

    Ok(rows.into_iter().map(ConversationMessage::from).collect())
}

// Inserts the user's message, calls the same `generate_reply` the floating
// widget uses (Fase 7.10.2 is the first DB write the chat does), inserts the
// reply, and remembers the key/model used for next time this conversation is
// reopened.
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
            parts: vec![GeminiPart { text: row.content }],
        })
        .collect();

    let reply_text = generate_reply(db.inner(), key_id, &model, history).await?;

    let reply_row = ai_message::ActiveModel {
        conversation_id: Set(conversation_id),
        role: Set("model".to_string()),
        content: Set(reply_text),
        created_at: Set(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
    .insert(db.inner())
    .await?;

    ai_conversation::ActiveModel {
        id: Unchanged(conversation_id),
        key_id: Set(Some(key_id)),
        model: Set(model),
        updated_at: Set(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
    .update(db.inner())
    .await?;

    Ok(reply_row.into())
}
