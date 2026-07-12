use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockFundamentals::Table)
                    .add_column(double_null(StockFundamentals::Payout))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockFundamentals::Table)
                    .drop_column(StockFundamentals::Payout)
                    .to_owned(),
            )
            .await
    }
}

/// `payout` (% do lucro distribuído como dividendo + JCP) — nullable como
/// `tax_rate` em `stock_dcf_fundamentals`, mas por um motivo diferente: não
/// é instabilidade matemática, é que nem toda empresa usa o mesmo rótulo de
/// conta pro DMPL (`cvm_dfp.py::fetch_payout`), então às vezes a extração
/// por palavra-chave não acha nada e o campo fica sem valor pra aquele
/// fetch em vez de arriscar um número errado.
#[derive(DeriveIden)]
enum StockFundamentals {
    Table,
    Payout,
}
