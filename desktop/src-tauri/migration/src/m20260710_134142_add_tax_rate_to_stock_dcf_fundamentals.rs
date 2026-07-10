use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockDcfFundamentals::Table)
                    .add_column(double_null(StockDcfFundamentals::TaxRate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockDcfFundamentals::Table)
                    .drop_column(StockDcfFundamentals::TaxRate)
                    .to_owned(),
            )
            .await
    }
}

/// `tax_rate` (effective tax rate, %) — nullable like `depreciation_amortization`/
/// `capex`, but for a different reason: the account codes (DRE `3.07`/`3.08`)
/// are stable, the instability is mathematical (near-zero pre-tax income
/// makes the ratio explode — see `cvm_dfp.py::_effective_tax_rate`).
#[derive(DeriveIden)]
enum StockDcfFundamentals {
    Table,
    TaxRate,
}
