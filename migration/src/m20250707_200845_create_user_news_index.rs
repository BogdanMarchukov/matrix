use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_user_news_user_id_news_id")
                    .table(UserNews::Table)
                    .col(UserNews::UserId)
                    .col(UserNews::NewsId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_news_user_id_news_id")
                    .table(UserNews::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserNews {
    Table,
    UserId,
    NewsId,
}
