use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AiConversation::Table)
                    .if_not_exists()
                    .col(pk_auto(AiConversation::Id))
                    .col(string(AiConversation::Title))
                    .col(integer_null(AiConversation::KeyId))
                    .col(string(AiConversation::Model))
                    .col(string(AiConversation::CreatedAt))
                    .col(string(AiConversation::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ai_conversation_key")
                            .from(AiConversation::Table, AiConversation::KeyId)
                            .to(AiApiKey::Table, AiApiKey::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AiMessage::Table)
                    .if_not_exists()
                    .col(pk_auto(AiMessage::Id))
                    .col(integer(AiMessage::ConversationId))
                    .col(string(AiMessage::Role))
                    .col(string(AiMessage::Content))
                    .col(string(AiMessage::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ai_message_conversation")
                            .from(AiMessage::Table, AiMessage::ConversationId)
                            .to(AiConversation::Table, AiConversation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AiMessage::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AiConversation::Table).to_owned())
            .await
    }
}

/// Conversas salvas do chat em tela cheia (Fase 7.10.2) — cada uma guarda a
/// última chave/modelo usados (`key_id`/`model`), atualizados a cada mensagem
/// enviada, pra reabrir a conversa já com o mesmo seletor. `key_id` é nullable
/// com `ON DELETE SET NULL`: apagar uma chave de API não quebra conversas
/// antigas que a usaram, só exige escolher outra na próxima mensagem.
#[derive(DeriveIden)]
enum AiConversation {
    Table,
    Id,
    Title,
    KeyId,
    Model,
    CreatedAt,
    UpdatedAt,
}

/// Histórico de mensagens de uma `ai_conversation` — `role` segue a mesma
/// convenção que `GeminiContent.role` já usa em `chat.rs`/`ChatPanel.tsx`
/// ("user"/"model"), sem precisar de tradução entre os dois.
#[derive(DeriveIden)]
enum AiMessage {
    Table,
    Id,
    ConversationId,
    Role,
    Content,
    CreatedAt,
}

/// Só `Table`/`Id` — esta migration não é dona de `ai_api_key` (criada em
/// `m20260716_005530_...`), só aponta uma FK pra ela.
#[derive(DeriveIden)]
enum AiApiKey {
    Table,
    Id,
}
