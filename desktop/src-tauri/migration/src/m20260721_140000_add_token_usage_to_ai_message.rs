use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite only allows one column operation per `ALTER TABLE`
        // statement (sea-query panics on "multiple alter options") — unlike
        // every other migration in this project so far, this needs two
        // separate `alter_table` calls instead of one chained builder.
        manager
            .alter_table(
                Table::alter()
                    .table(AiMessage::Table)
                    .add_column(integer_null(AiMessage::InputTokens))
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(AiMessage::Table)
                    .add_column(integer_null(AiMessage::OutputTokens))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AiMessage::Table)
                    .drop_column(AiMessage::InputTokens)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(AiMessage::Table)
                    .drop_column(AiMessage::OutputTokens)
                    .to_owned(),
            )
            .await
    }
}

/// `input_tokens`/`output_tokens` (Fase 7.10.3) — só a mensagem de resposta
/// (`role = "model"`) grava valor, extraído do `usage`/`usageMetadata` que
/// os 3 provedores já devolvem em `commands::chat::generate_reply`. A
/// mensagem do usuário fica com as duas colunas `NULL`, mesmo espírito
/// nullable de `payout` em `stock_fundamentals` — aqui o motivo é que não
/// existe "custo de geração" pra uma mensagem que só foi enviada.
#[derive(DeriveIden)]
enum AiMessage {
    Table,
    InputTokens,
    OutputTokens,
}
