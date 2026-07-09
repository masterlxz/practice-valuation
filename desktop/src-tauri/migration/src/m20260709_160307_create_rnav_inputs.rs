use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RnavInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(RnavInputs::Id))
                    .col(integer(RnavInputs::ValuationId))
                    .col(double(RnavInputs::Landbank))
                    .col(double(RnavInputs::InventoryAtMarketValue))
                    .col(double(RnavInputs::NetCash))
                    .col(double(RnavInputs::SharesOutstanding))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rnav_inputs_valuation")
                            .from(RnavInputs::Table, RnavInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RnavInputs::Table).to_owned())
            .await
    }
}

/// Reference to the shared `valuation` table (defined in the Bazin migration) for the FK.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}

/// RNAV-specific inputs (ver Fase 3 — modelo 6, incorporadoras).
#[derive(DeriveIden)]
enum RnavInputs {
    Table,
    Id,
    ValuationId,
    Landbank,
    InventoryAtMarketValue,
    NetCash,
    SharesOutstanding,
}
