use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlertEvent::Table)
                    .if_not_exists()
                    .col(pk_auto(AlertEvent::Id))
                    .col(integer(AlertEvent::AlertRuleId))
                    .col(boolean(AlertEvent::IsTriggered))
                    .col(string(AlertEvent::Message))
                    .col(string(AlertEvent::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_alert_event_alert_rule")
                            .from(AlertEvent::Table, AlertEvent::AlertRuleId)
                            .to(AlertRule::Table, AlertRule::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AlertEvent::Table).to_owned())
            .await
    }
}

/// Append-only log of `alert_rule` state transitions (Fase 5.2 — "verificação
/// periódica"). A row is only ever inserted, never updated: the background
/// checker writes a new row exactly when a rule's triggered/not-triggered
/// state flips, in either direction (condition starts matching, or stops
/// matching again) — same "never overwrite" philosophy as `valuation`. The
/// most recent row per `alert_rule_id` is the rule's current known state;
/// no row yet means "never evaluated as triggered".
#[derive(DeriveIden)]
enum AlertEvent {
    Table,
    Id,
    AlertRuleId,
    IsTriggered,
    Message,
    CreatedAt,
}

/// Only `Table`/`Id` needed — this migration doesn't own `alert_rule`
/// (created in `m20260711_093000_...`), it just points a foreign key at it.
#[derive(DeriveIden)]
enum AlertRule {
    Table,
    Id,
}
