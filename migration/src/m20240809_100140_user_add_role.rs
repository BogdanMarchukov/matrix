use sea_orm_migration::prelude::*;
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "CREATE TYPE user_role_type AS ENUM ('MEMBER', 'ADMIN', 'OWNER');";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())?;
        let alter_sql = "ALTER TABLE users ADD COLUMN role user_role_type NOT NULL DEFAULT 'MEMBER';";
        let stmt = Statement::from_string(manager.get_database_backend(), alter_sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}

