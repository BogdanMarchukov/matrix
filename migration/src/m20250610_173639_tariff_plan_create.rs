use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TariffPlan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TariffPlan::TariffPlanId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TariffPlan::Title)
                            .unique_key()
                            .not_null()
                            .string_len(50),
                    )
                    .col(ColumnDef::new(TariffPlan::Description).string())
                    .col(ColumnDef::new(TariffPlan::Price).not_null().money())
                    .col(
                        ColumnDef::new(TariffPlan::ExpiryDays)
                            .integer()
                            .not_null()
                            .check(Expr::col(TariffPlan::ExpiryDays).gte(0)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TariffPlan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TariffPlan {
    Table,
    TariffPlanId,
    Title,
    Description,
    Price,
    ExpiryDays,
}
