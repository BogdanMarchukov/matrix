use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TariffPlanPayment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TariffPlanPayment::TariffPlanPaymentId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TariffPlanPayment::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(TariffPlanPayment::PaymentId)
                            .not_null()
                            .uuid(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payment-tariff-plan-users")
                            .from(TariffPlanPayment::Table, TariffPlanPayment::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payment-tariff-plan-payment")
                            .from(TariffPlanPayment::Table, TariffPlanPayment::PaymentId)
                            .to(Payment::Table, Payment::PaymentId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TariffPlanPayment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TariffPlanPayment {
    Table,
    TariffPlanPaymentId,
    UserId,
    PaymentId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum Payment {
    Table,
    PaymentId,
}
