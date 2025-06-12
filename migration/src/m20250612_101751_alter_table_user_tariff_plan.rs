use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserTariffPlan::Table)
                    .add_column(ColumnDef::new(UserTariffPlan::TariffPlanPaymentId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-user-tariff-plan-payment-tariff-plan")
                            .from_tbl(UserTariffPlan::Table)
                            .from_col(UserTariffPlan::TariffPlanPaymentId)
                            .to_tbl(TariffPlanPayment::Table)
                            .to_col(TariffPlanPayment::TariffPlanPaymentId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserTariffPlan::Table)
                    .drop_column(UserTariffPlan::TariffPlanPaymentId)
                    .drop_foreign_key(Alias::new("fk-user-tariff-plan-payment-tariff-plan"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserTariffPlan {
    Table,
    TariffPlanPaymentId,
}

#[derive(DeriveIden)]
enum TariffPlanPayment {
    Table,
    TariffPlanPaymentId,
}
