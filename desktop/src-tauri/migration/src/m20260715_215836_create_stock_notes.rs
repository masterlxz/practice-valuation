use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockNotes::Table)
                    .if_not_exists()
                    .col(pk_auto(StockNotes::Id))
                    .col(string(StockNotes::Ticker))
                    .col(string(StockNotes::Note))
                    .col(string(StockNotes::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockNotes::Table).to_owned())
            .await
    }
}

/// Anotação livre por ticker pra tela "Stock Lookup" — uma row por ticker
/// (upsert feito na camada de comando, sem constraint de unicidade no
/// banco: app single-user local, sem risco real de concorrência).
#[derive(DeriveIden)]
enum StockNotes {
    Table,
    Id,
    Ticker,
    Note,
    UpdatedAt,
}
