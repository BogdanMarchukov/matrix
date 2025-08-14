use sea_orm::{Statement, QueryResult};
use sea_orm::prelude::Uuid;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let offer_sql = "insert into offer (offer_id, title, tariff_ids, is_active)
                        values (gen_random_uuid(), 'matrix_schema', ARRAY(select tariff_plan_id from tariff_plan where title = 'free'), true)
                        returning offer_id;";

        let offer_stmt = Statement::from_string(manager.get_database_backend(), offer_sql.to_owned());
        let offer_id: Uuid = manager
            .get_connection()
            .query_one(offer_stmt)
            .await?
            .expect("Failed to retrieve offer_id")
            .try_get("", "offer_id")?;

        let calculator_sql = format!("insert into calculator (calculator_id, type, require_params, offer_id)
                        values (gen_random_uuid(), 'MATRIX_SCHEMA', ARRAY['date_of_birth'], '{}');", offer_id);
        let calculator_stmt = Statement::from_string(manager.get_database_backend(), calculator_sql);
        manager.get_connection().execute(calculator_stmt).await.map(|_| ())
    }
}
