use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

use crate::entity::valuation;
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
