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
                    .add_column(double_null(StockDcfFundamentals::Revenue))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockDcfFundamentals::Table)
                    .drop_column(StockDcfFundamentals::Revenue)
                    .to_owned(),
            )
            .await
    }
}

/// `revenue` (receita líquida, conta "3.01" da DRE) — nullable como
/// `tax_rate`/`payout`: bancos usam taxonomia de DRE diferente e já são
/// descartados inteiros antes de chegar aqui, mas mantido nullable pelo
/// mesmo motivo de robustez dos outros campos desta tabela. Pra margem
/// líquida do Stock Lookup (Fase 9.2).
#[derive(DeriveIden)]
enum StockDcfFundamentals {
    Table,
    Revenue,
}
