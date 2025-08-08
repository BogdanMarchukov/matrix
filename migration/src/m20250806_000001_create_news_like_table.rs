use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::prelude::DateTimeWithTimeZone;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250806_000001_create_news_like_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NewsLike::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NewsLike::NewsLikeId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NewsLike::NewsId).uuid().not_null())
                    .col(ColumnDef::new(NewsLike::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(NewsLike::CreatedAt)
                            .date_time_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-news-like-news")
                            .from(NewsLike::Table, NewsLike::NewsId)
                            .to(News::Table, News::NewsId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-news-like-user")
                            .from(NewsLike::Table, NewsLike::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NewsLike::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum NewsLike {
    Table,
    NewsLikeId,
    NewsId,
    UserId,
    CreatedAt,
}

#[derive(Iden)]
enum News {
    Table,
    NewsId,
}

#[derive(Iden)]
enum Users {
    Table,
    UserId,
}
