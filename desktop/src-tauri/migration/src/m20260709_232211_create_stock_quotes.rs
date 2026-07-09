use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockQuotes::Table)
                    .if_not_exists()
                    .col(pk_auto(StockQuotes::Id))
                    .col(string(StockQuotes::Ticker))
                    .col(double(StockQuotes::Price))
                    .col(string(StockQuotes::Source))
                    .col(string(StockQuotes::FetchedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockQuotes::Table).to_owned())
            .await
    }
}

/// Time series of collected stock quotes (Fase 2.2) — one row per fetch,
/// never overwritten, same pattern as `crypto_indicators`.
#[derive(DeriveIden)]
enum StockQuotes {
    Table,
    Id,
    Ticker,
    Price,
    Source,
    FetchedAt,
}
