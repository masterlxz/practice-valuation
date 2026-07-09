use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectedCeilingInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(ProjectedCeilingInputs::Id))
                    .col(integer(ProjectedCeilingInputs::ValuationId))
                    .col(double(ProjectedCeilingInputs::CurrentDividend))
                    .col(double(ProjectedCeilingInputs::ExpectedGrowth))
                    .col(integer(ProjectedCeilingInputs::ProjectionYears))
                    .col(double(ProjectedCeilingInputs::DesiredYield))
                    .col(double(ProjectedCeilingInputs::Ke))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_projected_ceiling_inputs_valuation")
                            .from(ProjectedCeilingInputs::Table, ProjectedCeilingInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectedCeilingInputs::Table).to_owned())
            .await
    }
}

/// Reference to the shared `valuation` table (defined in the Bazin migration) for the FK.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}

/// Projected ceiling price inputs (ver Fase 3 — modelo 7, Bazin com N anos de projeção).
#[derive(DeriveIden)]
enum ProjectedCeilingInputs {
    Table,
    Id,
    ValuationId,
    CurrentDividend,
    ExpectedGrowth,
    ProjectionYears,
    DesiredYield,
    Ke,
}
