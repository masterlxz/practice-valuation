use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BanksInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(BanksInputs::Id))
                    .col(integer(BanksInputs::ValuationId))
                    .col(double(BanksInputs::BookValuePerShare))
                    .col(double(BanksInputs::Roe))
                    .col(double(BanksInputs::Payout))
                    .col(double(BanksInputs::Ke))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_banks_inputs_valuation")
                            .from(BanksInputs::Table, BanksInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BanksInputs::Table).to_owned())
            .await
    }
}

/// Reference to the shared `valuation` table (defined in the Bazin migration) for the FK.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}

/// Banks-specific inputs (ver Fase 3 — modelo 5, P/B via ROE-Gordon).
#[derive(DeriveIden)]
enum BanksInputs {
    Table,
    Id,
    ValuationId,
    BookValuePerShare,
    Roe,
    Payout,
    Ke,
}
