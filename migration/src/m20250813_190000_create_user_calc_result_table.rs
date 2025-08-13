use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserCalcResult::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserCalcResult::UserCalcResultId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserCalcResult::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCalcResult::CalculatorId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCalcResult::Result)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCalcResult::Key)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_calc_result-user_id")
                            .from(UserCalcResult::Table, UserCalcResult::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_calc_result-calculator_id")
                            .from(UserCalcResult::Table, UserCalcResult::CalculatorId)
                            .to(Calculator::Table, Calculator::CalculatorId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserCalcResult::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserCalcResult {
    Table,
    UserCalcResultId,
    UserId,
    CalculatorId,
    Result,
    Key,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum Calculator {
    Table,
    CalculatorId,
}
