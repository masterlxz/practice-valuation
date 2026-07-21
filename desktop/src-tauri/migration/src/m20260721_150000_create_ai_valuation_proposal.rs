use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AiValuationProposal::Table)
                    .if_not_exists()
                    .col(pk_auto(AiValuationProposal::Id))
                    .col(integer(AiValuationProposal::MessageId))
                    .col(string(AiValuationProposal::Model))
                    .col(text(AiValuationProposal::Payload))
                    .col(text(AiValuationProposal::Preview))
                    .col(string(AiValuationProposal::Status))
                    .col(integer_null(AiValuationProposal::CreatedValuationId))
                    .col(string(AiValuationProposal::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ai_valuation_proposal_message")
                            .from(AiValuationProposal::Table, AiValuationProposal::MessageId)
                            .to(AiMessage::Table, AiMessage::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ai_valuation_proposal_valuation")
                            .from(AiValuationProposal::Table, AiValuationProposal::CreatedValuationId)
                            .to(Valuation::Table, Valuation::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AiValuationProposal::Table).to_owned())
            .await
    }
}

/// Fase 7.10.4 — proposta de criação de valuation feita pela IA, presa à
/// `ai_message` (role "model") que a exibe. `payload` guarda tudo que
/// `commands::ai_proposal::insert_valuation` precisa pra escrever de verdade
/// (ticker/reference_year/current_price/inputs), serializado como texto JSON
/// — mais simples que uma tabela de payload por modelo, já que o formato de
/// `inputs` varia por modelo e esta linha é só um rascunho até aprovação.
/// `preview` guarda fair_price/safety_margin/verdict já calculados no
/// momento da proposta (mesma chamada domain::<model>::calculate() usada pra
/// validar) — não recalcula na aprovação, só reusa. `status` começa
/// "pending", vira "approved" ou "rejected" uma única vez (nunca volta).
/// `created_valuation_id` é nullable com `ON DELETE SET NULL`: apagar a
/// valuation criada depois não derruba o rastro da proposta que a originou.
#[derive(DeriveIden)]
enum AiValuationProposal {
    Table,
    Id,
    MessageId,
    Model,
    Payload,
    Preview,
    Status,
    CreatedValuationId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AiMessage {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Valuation {
    Table,
    Id,
}
