use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250806_000000_create_offer_like_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OfferLike::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OfferLike::OfferLikeId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OfferLike::OfferId).uuid().not_null())
                    .col(ColumnDef::new(OfferLike::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-offer-like-offer")
                            .from(OfferLike::Table, OfferLike::OfferId)
                            .to(Offer::Table, Offer::OfferId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-offer-like-user")
                            .from(OfferLike::Table, OfferLike::UserId)
                            .to(User::Table, User::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OfferLike::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum OfferLike {
    Table,
    OfferLikeId,
    OfferId,
    UserId,
}

#[derive(Iden)]
enum Offer {
    Table,
    OfferId,
}

#[derive(Iden)]
enum User {
    Table,
    UserId,
}
