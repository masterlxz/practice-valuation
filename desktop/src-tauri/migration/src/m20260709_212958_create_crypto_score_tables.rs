use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IndicatorThresholds::Table)
                    .if_not_exists()
                    .col(pk_auto(IndicatorThresholds::Id))
                    .col(string_uniq(IndicatorThresholds::Indicator))
                    .col(double(IndicatorThresholds::GreenBoundary))
                    .col(double(IndicatorThresholds::RedBoundary))
                    .col(string(IndicatorThresholds::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CryptoIndicators::Table)
                    .if_not_exists()
                    .col(pk_auto(CryptoIndicators::Id))
                    .col(string(CryptoIndicators::Coin))
                    .col(string(CryptoIndicators::Indicator))
                    .col(string(CryptoIndicators::ReadingDate))
                    .col(double(CryptoIndicators::RawValue))
                    .col(string(CryptoIndicators::Signal))
                    .col(string(CryptoIndicators::Source))
                    .col(string(CryptoIndicators::CreatedAt))
                    .to_owned(),
            )
            .await?;

        // Starting thresholds for the 9 Ethereum score indicators (Fase 3 spec).
        // Only mvrv_z_score, puell_multiple and staking_yield came with exact
        // numbers in the spec — the rest are documented placeholders (see plan),
        // adjustable later by updating this table directly, no migration needed.
        let now = chrono::Utc::now().to_rfc3339();
        let seeds = [
            ("mvrv_z_score", 0.0, 7.0),
            ("puell_multiple", 0.5, 4.0),
            ("staking_yield", 2.0, 0.0),
            ("nvt_ratio", 0.9, 1.3),
            ("net_issuance", 0.0, 2.0),
            ("tvl_trend", 0.0, -10.0),
            ("active_addresses_trend", 0.0, -10.0),
            ("exchange_netflow", 0.0, 0.5),
            ("fees_vs_emission", 0.5, 0.1),
        ];

        let db = manager.get_connection();
        for (indicator, green_boundary, red_boundary) in seeds {
            db.execute_unprepared(&format!(
                "INSERT INTO indicator_thresholds (indicator, green_boundary, red_boundary, updated_at) VALUES ('{indicator}', {green_boundary}, {red_boundary}, '{now}')"
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CryptoIndicators::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IndicatorThresholds::Table).to_owned())
            .await
    }
}

/// Configurable green/red cutoffs per indicator (Fase 3.3 — "thresholds ajustáveis").
#[derive(DeriveIden)]
enum IndicatorThresholds {
    Table,
    Id,
    Indicator,
    GreenBoundary,
    RedBoundary,
    UpdatedAt,
}

/// Time series of logged readings (the "cripto_indicadores" from PROJECT_STATE.md).
#[derive(DeriveIden)]
enum CryptoIndicators {
    Table,
    Id,
    Coin,
    Indicator,
    ReadingDate,
    RawValue,
    Signal,
    Source,
    CreatedAt,
}
