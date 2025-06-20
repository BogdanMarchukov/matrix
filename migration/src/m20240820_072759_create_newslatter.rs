use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Newsletter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Newsletter::NewsletterId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Newsletter::Title).string().not_null())
                    .col(ColumnDef::new(Newsletter::IsPublished).boolean().not_null().default(false))
                    .col(ColumnDef::new(Newsletter::Payload).string().not_null())
                    .col(ColumnDef::new(Newsletter::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Newsletter::PublishAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Newsletter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Newsletter {
    Table,
    NewsletterId,
    Title,
    IsPublished,
    Payload,
    CreatedAt,
    PublishAt,
}
