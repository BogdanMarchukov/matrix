use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(News::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(News::NewsId).uuid().not_null().primary_key())
                    .col(ColumnDef::new(News::Title).string().not_null())
                    .col(ColumnDef::new(News::Payload).string().not_null())
                    .col(ColumnDef::new(News::PublishAt).date_time().not_null())
                    .col(
                        ColumnDef::new(News::IsPublish)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(News::Img).string())
                    .col(
                        ColumnDef::new(News::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(News::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(News::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum News {
    Table,
    NewsId,
    Title,
    Payload,
    Img,
    PublishAt,
    IsPublish,
    CreatedAt,
    UpdatedAt,
}
