use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payment::PaymentId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Payment::Sum).not_null().decimal())
                    .col(ColumnDef::new(Payment::UserId).not_null().uuid())
                    .col(
                        ColumnDef::new(Payment::CreatedAt)
                            .not_null()
                            .date_time()
                            .default(Expr::current_date()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payment-user")
                            .from(Payment::Table, Payment::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Payment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Payment {
    Table,
    PaymentId,
    Sum,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}
