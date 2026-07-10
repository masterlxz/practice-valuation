use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("stock_dividends_avg"))
                    .rename_column(
                        Alias::new("avg_dividend5y"),
                        Alias::new("avg_dividend_5y"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("stock_dividends_avg"))
                    .rename_column(
                        Alias::new("avg_dividend_5y"),
                        Alias::new("avg_dividend5y"),
                    )
                    .to_owned(),
            )
            .await
    }
}
