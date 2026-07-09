use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GordonInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(GordonInputs::Id))
                    .col(integer(GordonInputs::ValuationId))
                    .col(double(GordonInputs::CurrentDividend))
                    .col(double(GordonInputs::ExpectedGrowth))
                    .col(double(GordonInputs::Ke))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gordon_inputs_valuation")
                            .from(GordonInputs::Table, GordonInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GordonInputs::Table).to_owned())
            .await
    }
}

/// Reference to the shared `valuation` table (defined in the Bazin migration) for the FK.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}

/// Gordon/DDM-specific inputs (ver Fase 3 — D0, g, Ke).
#[derive(DeriveIden)]
enum GordonInputs {
    Table,
    Id,
    ValuationId,
    CurrentDividend,
    ExpectedGrowth,
    Ke,
}
