use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::domain::graham::{self, GrahamInputs};
use crate::entity::{graham_inputs, valuation};
use crate::error::AppError;

#[derive(Deserialize)]
pub struct CalculateGrahamRequest {
    pub ticker: String,
    pub reference_year: i32,
    pub current_price: f64,
    pub eps: f64,
    pub book_value_per_share: f64,
}

#[derive(Serialize)]
pub struct GrahamValuationResponse {
    pub valuation: valuation::Model,
    pub inputs: graham_inputs::Model,
}

#[tauri::command]
pub async fn calculate_graham(
    db: tauri::State<'_, DatabaseConnection>,
    request: CalculateGrahamRequest,
) -> Result<GrahamValuationResponse, AppError> {
    let outcome = graham::calculate(
        &GrahamInputs {
            eps: request.eps,
            book_value_per_share: request.book_value_per_share,
        },
        request.current_price,
    )?;

    let valuation = valuation::ActiveModel {
        ticker: Set(request.ticker),
        reference_year: Set(request.reference_year),
        current_price: Set(request.current_price),
        model: Set("graham".to_string()),
        fair_price: Set(Some(outcome.fair_price)),
        safety_margin: Set(Some(outcome.safety_margin)),
        verdict: Set(Some(outcome.verdict.as_str().to_string())),
        updated_at: Set(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
    .insert(db.inner())
    .await?;

    let inputs = graham_inputs::ActiveModel {
        valuation_id: Set(valuation.id),
        eps: Set(request.eps),
        book_value_per_share: Set(request.book_value_per_share),
        ..Default::default()
    }
    .insert(db.inner())
    .await?;

    Ok(GrahamValuationResponse { valuation, inputs })
}
