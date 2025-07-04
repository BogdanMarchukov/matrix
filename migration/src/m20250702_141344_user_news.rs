use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserNews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserNews::UserNewsId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserNews::NewsId).uuid().not_null())
                    .col(ColumnDef::new(UserNews::Title).string().not_null())
                    .col(ColumnDef::new(UserNews::Payload).string().not_null())
                    .col(ColumnDef::new(UserNews::Img).string())
                    .col(
                        ColumnDef::new(UserNews::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserNews::ReadingAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(UserNews::ReadingCount).integer().not_null())
                    .col(ColumnDef::new(UserNews::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_news-news_id")
                            .from(UserNews::Table, UserNews::NewsId)
                            .to(News::Table, News::NewsId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_news-user_id")
                            .from(UserNews::Table, UserNews::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserNews::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserNews {
    Table,
    UserNewsId,
    UserId,
    NewsId,
    Title,
    Payload,
    Img,
    ReadingCount,
    CreatedAt,
    ReadingAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum News {
    Table,
    NewsId,
}
