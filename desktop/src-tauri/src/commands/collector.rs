use std::sync::atomic::{AtomicBool, Ordering};

use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use serde::Serialize;
use tokio::process::Command;

use crate::entity::{stock_dcf_fundamentals, stock_dividends_avg, stock_fundamentals, stock_quotes};
use crate::error::AppError;

// Dev-only paths — the venv and script live in the data-collector/ bind
// mount (see docker-compose.yml), same absolute-path convention as
// db.rs's DATABASE_URL. Production path deferred to Fase 6.
const COLLECTOR_PYTHON: &str = "/data-collector/.venv/bin/python3";
const COLLECTOR_SCRIPT: &str = "/data-collector/main.py";

#[derive(Serialize)]
pub struct CollectorSummary {
    pub success: bool,
    pub output: String,
}

async fn run_collector(
    lock: &AtomicBool,
    extra_args: &[&str],
) -> Result<CollectorSummary, AppError> {
    if lock.swap(true, Ordering::SeqCst) {
        return Err(AppError::CollectorBusy);
    }

    let result = Command::new(COLLECTOR_PYTHON)
        .arg(COLLECTOR_SCRIPT)
        .args(extra_args)
        .output()
        .await;

    lock.store(false, Ordering::SeqCst);

    let output = result.map_err(|err| AppError::CollectorFailed(err.to_string()))?;

    let summary = if output.status.success() {
        CollectorSummary {
            success: true,
            output: String::from_utf8_lossy(&output.stdout).to_string(),
        }
    } else {
        CollectorSummary {
            success: false,
            output: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    };

    Ok(summary)
}

#[tauri::command]
pub async fn run_stock_collector(
    lock: tauri::State<'_, AtomicBool>,
    ticker: String,
) -> Result<CollectorSummary, AppError> {
    run_collector(&lock, &["--ticker", &ticker]).await
}

#[tauri::command]
pub async fn run_crypto_collector(
    lock: tauri::State<'_, AtomicBool>,
) -> Result<CollectorSummary, AppError> {
    run_collector(&lock, &["crypto"]).await
}

#[tauri::command]
pub async fn list_stock_quotes(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<stock_quotes::Model>, AppError> {
    let quotes = stock_quotes::Entity::find()
        .order_by_desc(stock_quotes::Column::FetchedAt)
        .all(db.inner())
        .await?;

    Ok(quotes)
}

#[tauri::command]
pub async fn list_stock_fundamentals(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<stock_fundamentals::Model>, AppError> {
    let fundamentals = stock_fundamentals::Entity::find()
        .order_by_desc(stock_fundamentals::Column::FetchedAt)
        .all(db.inner())
        .await?;

    Ok(fundamentals)
}

#[tauri::command]
pub async fn list_stock_dividends_avg(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<stock_dividends_avg::Model>, AppError> {
    let dividends = stock_dividends_avg::Entity::find()
        .order_by_desc(stock_dividends_avg::Column::FetchedAt)
        .all(db.inner())
        .await?;

    Ok(dividends)
}

#[tauri::command]
pub async fn list_stock_dcf_fundamentals(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<stock_dcf_fundamentals::Model>, AppError> {
    let fundamentals = stock_dcf_fundamentals::Entity::find()
        .order_by_desc(stock_dcf_fundamentals::Column::FetchedAt)
        .all(db.inner())
        .await?;

    Ok(fundamentals)
}
