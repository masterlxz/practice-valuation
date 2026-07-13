use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RimInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(RimInputs::Id))
                    .col(integer(RimInputs::ValuationId))
                    .col(double(RimInputs::BookValuePerShare))
                    .col(double(RimInputs::RoeCurrent))
                    .col(double(RimInputs::Payout))
                    .col(double(RimInputs::Ke))
                    .col(integer(RimInputs::FadeYears))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rim_inputs_valuation")
                            .from(RimInputs::Table, RimInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RimInputs::Table).to_owned())
            .await
    }
}

/// Reference to the shared `valuation` table (defined in the Bazin migration) for the FK.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}

/// RIM-specific inputs (ver Fase 3 — modelo 8, Lucro Residual pra bancos).
#[derive(DeriveIden)]
enum RimInputs {
    Table,
    Id,
    ValuationId,
    BookValuePerShare,
    RoeCurrent,
    Payout,
    Ke,
    FadeYears,
}
