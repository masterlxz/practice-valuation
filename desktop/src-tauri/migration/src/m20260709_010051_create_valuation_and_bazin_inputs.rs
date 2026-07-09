use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Valuation::Table)
                    .if_not_exists()
                    .col(pk_auto(Valuation::Id))
                    .col(string(Valuation::Ticker))
                    .col(integer(Valuation::ReferenceYear))
                    .col(double(Valuation::CurrentPrice))
                    .col(string(Valuation::Model))
                    .col(double_null(Valuation::FairPrice))
                    .col(double_null(Valuation::SafetyMargin))
                    .col(string_null(Valuation::Verdict))
                    .col(string(Valuation::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BazinInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(BazinInputs::Id))
                    .col(integer(BazinInputs::ValuationId))
                    .col(double(BazinInputs::AverageDividend))
                    .col(double(BazinInputs::DesiredYield))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bazin_inputs_valuation")
                            .from(BazinInputs::Table, BazinInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BazinInputs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Valuation::Table).to_owned())
            .await
    }
}

/// Common fields shared by every valuation model (Fase 3 — "regra geral").
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
    Ticker,
    ReferenceYear,
    CurrentPrice,
    Model,
    FairPrice,
    SafetyMargin,
    Verdict,
    UpdatedAt,
}

/// Bazin-specific inputs (ver Fase 3 — Dividendo Médio + Yield Desejado).
#[derive(DeriveIden)]
enum BazinInputs {
    Table,
    Id,
    ValuationId,
    AverageDividend,
    DesiredYield,
}
