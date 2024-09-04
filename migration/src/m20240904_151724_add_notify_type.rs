use sea_orm_migration::prelude::*;
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "CREATE TYPE notify_type_enum AS ENUM ('DALY');";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())?;
        let alter_sql = "ALTER TABLE notify ADD COLUMN notify_type notify_type_enum NOT NULL DEFAULT 'DALY';";
        let stmt = Statement::from_string(manager.get_database_backend(), alter_sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}

