use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AiApiKey::Table)
                    .if_not_exists()
                    .col(pk_auto(AiApiKey::Id))
                    .col(string(AiApiKey::Provider))
                    .col(string(AiApiKey::Name))
                    .col(string(AiApiKey::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AiApiKey::Table).to_owned())
            .await
    }
}

/// Lists the named API keys the user has configured per AI chat provider
/// (Fase 7.9.2) — one row per named key, never the secret itself. The secret
/// stays in the OS keyring, addressed by `"{provider}:{id}"` so each row gets
/// its own keyring entry without needing a secret column here.
#[derive(DeriveIden)]
enum AiApiKey {
    Table,
    Id,
    Provider,
    Name,
    CreatedAt,
}
