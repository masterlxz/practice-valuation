use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockTechnicals::Table)
                    .if_not_exists()
                    .col(pk_auto(StockTechnicals::Id))
                    .col(string(StockTechnicals::Ticker))
                    // `Alias::new` em vez das variantes do enum abaixo: o
                    // `DeriveIden` converte `Sma50`/`Cagr5y` pra `sma50`/
                    // `cagr5y` (sem underscore antes do dígito) — mesma
                    // armadilha que já causou `m20260710_220000_rename_avg_dividend5y_column`.
                    .col(double_null(Alias::new("sma_50")))
                    .col(double_null(Alias::new("sma_100")))
                    .col(double_null(Alias::new("sma_200")))
                    .col(double_null(Alias::new("cagr_5y")))
                    .col(double_null(Alias::new("cagr_10y")))
                    .col(string(StockTechnicals::Source))
                    .col(string(StockTechnicals::FetchedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockTechnicals::Table).to_owned())
            .await
    }
}

/// Time series of computed technicals (SMA 50/100/200 dias, CAGR 5/10 anos
/// de preço) pra tela "Stock Lookup" — one row per fetch, mesmo padrão de
/// `stock_dividends_avg`. Campos nullable porque tickers com pouco
/// histórico (IPO recente) não têm SMA/CAGR calculável pra toda janela —
/// mesmo padrão de `payout` em `stock_fundamentals`.
#[derive(DeriveIden)]
enum StockTechnicals {
    Table,
    Id,
    Ticker,
    Source,
    FetchedAt,
}
