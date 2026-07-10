use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockDcfFundamentals::Table)
                    .if_not_exists()
                    .col(pk_auto(StockDcfFundamentals::Id))
                    .col(string(StockDcfFundamentals::Ticker))
                    .col(integer(StockDcfFundamentals::ReferenceYear))
                    .col(double(StockDcfFundamentals::Ebit))
                    .col(double_null(StockDcfFundamentals::DepreciationAmortization))
                    .col(double_null(StockDcfFundamentals::Capex))
                    .col(double(StockDcfFundamentals::NwcChange))
                    .col(double(StockDcfFundamentals::TotalDebt))
                    .col(double(StockDcfFundamentals::Cash))
                    .col(double(StockDcfFundamentals::SharesOutstanding))
                    .col(string(StockDcfFundamentals::Source))
                    .col(string(StockDcfFundamentals::FetchedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockDcfFundamentals::Table).to_owned())
            .await
    }
}

/// Time series of DCF-relevant fundamentals collected from CVM's open data
/// (Fase 2.2, Sessão 5) — one row per fetch, never overwritten, same
/// pattern as `stock_fundamentals`. `depreciation_amortization`/`capex` are
/// nullable — extracted via best-effort keyword matching (account codes
/// aren't standardized across companies for these two), left `NULL` rather
/// than guessing when the match is ambiguous. `shares_outstanding` comes
/// from bolsai, not CVM's own `composicao_capital` file (found to have a
/// scale error for at least one real company — see `cvm_dfp.py`).
#[derive(DeriveIden)]
enum StockDcfFundamentals {
    Table,
    Id,
    Ticker,
    ReferenceYear,
    Ebit,
    DepreciationAmortization,
    Capex,
    NwcChange,
    TotalDebt,
    Cash,
    SharesOutstanding,
    Source,
    FetchedAt,
}
