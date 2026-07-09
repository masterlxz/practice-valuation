use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde_json::Value as JsonValue;

use crate::entity::{
    banks_inputs, bazin_inputs, dcf_inputs, gordon_inputs, graham_inputs,
    projected_ceiling_inputs, rnav_inputs, valuation,
};
use crate::error::AppError;

#[tauri::command]
pub async fn list_valuations(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<valuation::Model>, AppError> {
    let valuations = valuation::Entity::find()
        .order_by_desc(valuation::Column::UpdatedAt)
        .all(db.inner())
        .await?;

    Ok(valuations)
}

// The assumptions behind each saved calculation live in a table specific to
// its model (see Fase 1 — "valuation compartilhada + inputs por modelo").
// This command resolves which one to query from `valuation.model` and hands
// back the row as a plain JSON object — the frontend already knows the field
// shape per model (same labels used in the calculator forms), so there's no
// need for a Rust-side enum with one variant per model just to carry it over.
#[tauri::command]
pub async fn get_valuation_inputs(
    db: tauri::State<'_, DatabaseConnection>,
    valuation_id: i32,
    model: String,
) -> Result<JsonValue, AppError> {
    let not_found = || AppError::NotFound(format!("inputs for valuation {valuation_id}"));

    let value = match model.as_str() {
        "bazin" => serde_json::to_value(
            bazin_inputs::Entity::find()
                .filter(bazin_inputs::Column::ValuationId.eq(valuation_id))
                .one(db.inner())
                .await?
                .ok_or_else(not_found)?,
        ),
        "graham" => serde_json::to_value(
            graham_inputs::Entity::find()
                .filter(graham_inputs::Column::ValuationId.eq(valuation_id))
                .one(db.inner())
                .await?
                .ok_or_else(not_found)?,
        ),
        "gordon" => serde_json::to_value(
            gordon_inputs::Entity::find()
                .filter(gordon_inputs::Column::ValuationId.eq(valuation_id))
                .one(db.inner())
                .await?
                .ok_or_else(not_found)?,
        ),
        "dcf" => serde_json::to_value(
            dcf_inputs::Entity::find()
                .filter(dcf_inputs::Column::ValuationId.eq(valuation_id))
                .one(db.inner())
                .await?
                .ok_or_else(not_found)?,
        ),
        "banks" => serde_json::to_value(
            banks_inputs::Entity::find()
                .filter(banks_inputs::Column::ValuationId.eq(valuation_id))
                .one(db.inner())
                .await?
                .ok_or_else(not_found)?,
        ),
        "rnav" => serde_json::to_value(
            rnav_inputs::Entity::find()
                .filter(rnav_inputs::Column::ValuationId.eq(valuation_id))
                .one(db.inner())
                .await?
                .ok_or_else(not_found)?,
        ),
        "projected_ceiling" => serde_json::to_value(
            projected_ceiling_inputs::Entity::find()
                .filter(projected_ceiling_inputs::Column::ValuationId.eq(valuation_id))
                .one(db.inner())
                .await?
                .ok_or_else(not_found)?,
        ),
        _ => return Err(AppError::NotFound(format!("model '{model}'"))),
    };

    value.map_err(|err| AppError::NotFound(err.to_string()))
}
