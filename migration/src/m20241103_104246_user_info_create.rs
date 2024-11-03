use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInfo::UserInfoId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserInfo::City).string())
                    .col(ColumnDef::new(UserInfo::DateOfBirth).date())
                    .col(ColumnDef::new(UserInfo::TimeOfBirth).date())
                    .col(ColumnDef::new(UserInfo::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-info-user_id")
                            .from(UserInfo::Table, UserInfo::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserInfo {
    Table,
    UserInfoId,
    City,
    DateOfBirth,
    TimeOfBirth,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}
