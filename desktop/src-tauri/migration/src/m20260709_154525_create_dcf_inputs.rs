use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DcfInputs::Table)
                    .if_not_exists()
                    .col(pk_auto(DcfInputs::Id))
                    .col(integer(DcfInputs::ValuationId))
                    .col(double(DcfInputs::Ebit))
                    .col(double(DcfInputs::TaxRate))
                    .col(double(DcfInputs::DepreciationAmortization))
                    .col(double(DcfInputs::Capex))
                    .col(double(DcfInputs::NwcChange))
                    .col(double(DcfInputs::TotalDebt))
                    .col(double(DcfInputs::Cash))
                    .col(double(DcfInputs::SharesOutstanding))
                    .col(double(DcfInputs::Beta))
                    .col(double(DcfInputs::RiskFreeRate))
                    .col(double(DcfInputs::MarketRiskPremium))
                    .col(double(DcfInputs::Kd))
                    .col(double(DcfInputs::PerpetuityGrowth))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dcf_inputs_valuation")
                            .from(DcfInputs::Table, DcfInputs::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DcfInputs::Table).to_owned())
            .await
    }
}

/// Reference to the shared `valuation` table (defined in the Bazin migration) for the FK.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}

/// DCF/FCFF-specific inputs (ver Fase 3 — modelo 1).
#[derive(DeriveIden)]
enum DcfInputs {
    Table,
    Id,
    ValuationId,
    Ebit,
    TaxRate,
    DepreciationAmortization,
    Capex,
    NwcChange,
    TotalDebt,
    Cash,
    SharesOutstanding,
    Beta,
    RiskFreeRate,
    MarketRiskPremium,
    Kd,
    PerpetuityGrowth,
}
