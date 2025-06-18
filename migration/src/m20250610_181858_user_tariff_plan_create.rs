use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserTariffPlan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTariffPlan::UserTariffPlanId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserTariffPlan::TariffPlanId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserTariffPlan::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserTariffPlan::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserTariffPlan::ExpiresAt)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-user-tariff-plan")
                            .from(UserTariffPlan::Table, UserTariffPlan::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-tariff-plan-tariff-plan")
                            .from(UserTariffPlan::Table, UserTariffPlan::TariffPlanId)
                            .to(TariffPlan::Table, TariffPlan::TariffPlanId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTariffPlan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserTariffPlan {
    Table,
    UserTariffPlanId,
    TariffPlanId,
    UserId,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum TariffPlan {
    Table,
    TariffPlanId,
}
