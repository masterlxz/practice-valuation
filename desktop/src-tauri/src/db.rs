use sea_orm::{Database, DatabaseConnection, DbErr};

// Dev-only path: Rust and the Python data-collector share this file via the
// bind mount declared in docker-compose.yml. Production path (outside
// Docker) is deferred to Fase 6.
const DATABASE_URL: &str = "sqlite:///data-collector/practice_valuation.db?mode=rwc";

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    Database::connect(DATABASE_URL).await
}
