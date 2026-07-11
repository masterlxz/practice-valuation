use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlertRule::Table)
                    .if_not_exists()
                    .col(pk_auto(AlertRule::Id))
                    .col(string(AlertRule::TargetType))
                    .col(integer_null(AlertRule::ValuationId))
                    .col(string(AlertRule::Condition))
                    .col(string_null(AlertRule::Coin))
                    .col(string_null(AlertRule::Indicator))
                    .col(boolean(AlertRule::IsActive))
                    .col(string(AlertRule::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_alert_rule_valuation")
                            .from(AlertRule::Table, AlertRule::ValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AlertRule::Table).to_owned())
            .await
    }
}

/// Single polymorphic alert rule table (Fase 5.1 — "cadastro de regra de
/// alerta"). `target_type` discriminates the two supported kinds, same
/// pattern as `valuation.model`: "stock_price" rules point at an existing
/// saved `valuation` row via `valuation_id` (nullable — only stock_price
/// rules use it) and compare its already-computed `fair_price` against a
/// live quote later, in Fase 5.2; "crypto_indicator" rules use
/// `coin`+`indicator` (nullable — only crypto_indicator rules use them) and
/// compare against the already-computed GREEN/RED signal via
/// `indicator_thresholds`. `condition` holds BELOW_FAIR_PRICE/
/// ABOVE_FAIR_PRICE for stock_price or SIGNAL_GREEN/SIGNAL_RED for
/// crypto_indicator — validated in the command layer, not at the DB level.
/// `is_active` lets a rule be paused/resumed without deleting.
#[derive(DeriveIden)]
enum AlertRule {
    Table,
    Id,
    TargetType,
    ValuationId,
    Condition,
    Coin,
    Indicator,
    IsActive,
    CreatedAt,
}

/// Only `Table`/`Id` needed here — this migration doesn't own the
/// `valuation` table (created in `m20260709_010051_...`), it just points a
/// foreign key at it.
#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}
