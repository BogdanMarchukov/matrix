use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Calculator::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Calculator::CalculatorId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Calculator::Type)
                            .enumeration(Calculator::Type, vec![CalculatorType::MatrixSchema])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Calculator::RequireParams)
                            .array(ColumnType::String(StringLen::N(60))),
                    )
                    .col(
                        ColumnDef::new(Calculator::OptionsParams)
                            .array(ColumnType::String(StringLen::N(60))),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Calculator::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Calculator {
    Table,
    CalculatorId,
    Type,
    RequireParams,
    OptionsParams,
}

#[derive(Iden)]
enum CalculatorType {
    MatrixSchema,
}
