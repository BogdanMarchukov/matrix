use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notify::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notify::NotifyId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notify::Payload).string().not_null())
                    .col(ColumnDef::new(Notify::Title).string().not_null())
                    .col(
                        ColumnDef::new(Notify::IsRead)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Notify::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Notify::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notify-user_id")
                            .from(Notify::Table, Notify::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notify::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Notify {
    Table,
    NotifyId,
    Payload,
    Title,
    IsRead,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}
