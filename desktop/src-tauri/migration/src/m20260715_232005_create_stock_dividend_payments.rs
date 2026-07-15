use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockDividendPayments::Table)
                    .if_not_exists()
                    .col(pk_auto(StockDividendPayments::Id))
                    .col(string(StockDividendPayments::Ticker))
                    .col(string(StockDividendPayments::PaymentDate))
                    .col(double(StockDividendPayments::Amount))
                    .col(double_null(StockDividendPayments::PriceAtPayment))
                    .col(double_null(StockDividendPayments::YieldPct))
                    .col(string(StockDividendPayments::Source))
                    .col(string(StockDividendPayments::FetchedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_stock_dividend_payments_ticker_date")
                    .table(StockDividendPayments::Table)
                    .col(StockDividendPayments::Ticker)
                    .col(StockDividendPayments::PaymentDate)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockDividendPayments::Table).to_owned())
            .await
    }
}

/// Histórico de pagamentos individuais de dividendo (Fase 9.3, gráfico da
/// tela Stock Lookup) — diferente de `stock_dividends_avg` (só a média dos
/// últimos 5 anos, usada pelo Bazin), aqui é uma linha por pagamento real,
/// com o preço do dia e o yield daquele pagamento específico. Índice único
/// `(ticker, payment_date)` permite `INSERT OR IGNORE` no coletor Python —
/// rodar de novo não duplica histórico já salvo, só acrescenta pagamento
/// novo.
#[derive(DeriveIden)]
enum StockDividendPayments {
    Table,
    Id,
    Ticker,
    PaymentDate,
    Amount,
    PriceAtPayment,
    YieldPct,
    Source,
    FetchedAt,
}
