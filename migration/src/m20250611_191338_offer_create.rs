use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Offer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Offer::OfferId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Offer::Title).string_len(50).not_null())
                    .col(ColumnDef::new(Offer::Description).string())
                    .col(
                        ColumnDef::new(Offer::TariffIds)
                            .array(ColumnType::Uuid)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Offer::IsActive).not_null().boolean())
                    .col(
                        ColumnDef::new(Offer::CreatedAt)
                            .not_null()
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Offer::Img).string_len(100))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Offer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Offer {
    Table,
    OfferId,
    Title,
    Description,
    TariffIds,
    IsActive,
    CreatedAt,
    Img,
}
