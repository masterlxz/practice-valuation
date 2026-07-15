use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, Unchanged};
use serde::Deserialize;

use crate::entity::stock_notes;
use crate::error::AppError;

#[tauri::command]
pub async fn list_stock_notes(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<stock_notes::Model>, AppError> {
    let notes = stock_notes::Entity::find().all(db.inner()).await?;

    Ok(notes)
}

#[derive(Deserialize)]
pub struct SaveStockNoteRequest {
    pub ticker: String,
    pub note: String,
}

// One row per ticker (upsert) — the "Stock Lookup" screen's free-text
// annotation, not a time series like the collector tables in
// commands/collector.rs.
#[tauri::command]
pub async fn save_stock_note(
    db: tauri::State<'_, DatabaseConnection>,
    request: SaveStockNoteRequest,
) -> Result<stock_notes::Model, AppError> {
    let ticker = request.ticker.trim().to_uppercase();
    if ticker.is_empty() {
        return Err(AppError::InvalidInput("ticker must not be empty".to_string()));
    }

    let existing = stock_notes::Entity::find()
        .filter(stock_notes::Column::Ticker.eq(ticker.clone()))
        .one(db.inner())
        .await?;

    let now = chrono::Utc::now().to_rfc3339();

    let saved = if let Some(existing) = existing {
        stock_notes::ActiveModel {
            id: Unchanged(existing.id),
            ticker: Set(ticker),
            note: Set(request.note),
            updated_at: Set(now),
        }
        .update(db.inner())
        .await?
    } else {
        stock_notes::ActiveModel {
            ticker: Set(ticker),
            note: Set(request.note),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(db.inner())
        .await?
    };

    Ok(saved)
}
