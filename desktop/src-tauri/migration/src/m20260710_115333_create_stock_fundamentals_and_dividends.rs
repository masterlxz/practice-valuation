use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockFundamentals::Table)
                    .if_not_exists()
                    .col(pk_auto(StockFundamentals::Id))
                    .col(string(StockFundamentals::Ticker))
                    .col(double(StockFundamentals::Lpa))
                    .col(double(StockFundamentals::Vpa))
                    .col(double(StockFundamentals::Roe))
                    .col(string(StockFundamentals::Source))
                    .col(string(StockFundamentals::FetchedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(StockDividendsAvg::Table)
                    .if_not_exists()
                    .col(pk_auto(StockDividendsAvg::Id))
                    .col(string(StockDividendsAvg::Ticker))
                    .col(double(StockDividendsAvg::AvgDividend5y))
                    .col(string(StockDividendsAvg::Source))
                    .col(string(StockDividendsAvg::FetchedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockDividendsAvg::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(StockFundamentals::Table).to_owned())
            .await
    }
}

/// Time series of fundamentals fetched from bolsai's `/fundamentals/{ticker}`
/// (Fase 2.2) — one row per fetch, never overwritten, same pattern as
/// `stock_quotes`/`crypto_indicators`. Narrow scope: only the three fields
/// today's forms ask for by hand (Graham's LPA/VPA, Banks' ROE) — not the
/// full ~27-field response.
#[derive(DeriveIden)]
enum StockFundamentals {
    Table,
    Id,
    Ticker,
    Lpa,
    Vpa,
    Roe,
    Source,
    FetchedAt,
}

/// Time series of the Bazin-style "average dividend per share, last 5
/// years", derived from bolsai's `/dividends/{ticker}` annual summary
/// (Fase 2.2) — one row per fetch, same pattern as `stock_fundamentals`.
#[derive(DeriveIden)]
enum StockDividendsAvg {
    Table,
    Id,
    Ticker,
    AvgDividend5y,
    Source,
    FetchedAt,
}
