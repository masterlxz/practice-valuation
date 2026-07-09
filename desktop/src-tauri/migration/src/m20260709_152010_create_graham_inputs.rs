use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GrahamInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(GrahamInputs::Id))
                    .col(integer(GrahamInputs::ValuationId))
                    .col(double(GrahamInputs::Eps))
                    .col(double(GrahamInputs::BookValuePerShare))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_graham_inputs_valuation")
                            .from(GrahamInputs::Table, GrahamInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GrahamInputs::Table).to_owned())
            .await
    }
}

/// Reference to the shared `valuation` table (defined in the Bazin migration) for the FK.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}

/// Graham-specific inputs (ver Fase 3 — LPA + VPA).
#[derive(DeriveIden)]
enum GrahamInputs {
    Table,
    Id,
    ValuationId,
    Eps,
    BookValuePerShare,
}
