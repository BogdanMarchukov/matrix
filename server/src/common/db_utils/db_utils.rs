use async_graphql::{Context, FieldResult};
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};

use crate::{errors::gql_error::GqlError, GqlCtx};

pub async fn get_pool() -> DatabaseConnection {
    use crate::config;
    let db_url = config::get_database_url();
    let opt = ConnectOptions::new(db_url);
    let conn = Database::connect(opt)
        .await
        .expect("database connection error");
    Migrator::up(&conn, None).await.expect("migration error");
    conn
}

pub async fn get_transaction() -> DatabaseTransaction {
    let connection = get_pool().await;
    connection.begin().await.expect("transaction error")
}

pub fn get_connection_from_gql_ctx(ctx: &Context) -> FieldResult<DatabaseConnection> {
    let ctx_data = match ctx.data::<GqlCtx>() {
        Ok(data) => data,
        Err(_) => return Err(GqlError::ServerError("get cxt error".to_string()).into()),
    };
    Ok(ctx_data.db.to_owned())
}
