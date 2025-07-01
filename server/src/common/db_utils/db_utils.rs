use async_graphql::{Context, FieldResult};
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait,
};

use crate::{errors::gql_error::GqlError, GqlCtx};

pub async fn get_pool() -> Result<DatabaseConnection, DbErr> {
    use crate::config;
    let db_url = config::get_database_url();
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);
    Database::connect(opt).await
}

pub async fn get_transaction(
    db: Option<DatabaseConnection>,
) -> Result<DatabaseTransaction, GqlError> {
    let db = match db {
        Some(conn) => conn,
        None => get_pool()
            .await
            .map_err(|_| GqlError::ServerError("database error".into()))?,
    };

    db.begin()
        .await
        .map_err(|_| GqlError::ServerError("database error".into()))
}

pub fn get_connection_from_gql_ctx(ctx: &Context) -> FieldResult<DatabaseConnection> {
    let ctx_data = match ctx.data::<GqlCtx>() {
        Ok(data) => data,
        Err(_) => return Err(GqlError::ServerError("get cxt error".to_string()).into()),
    };
    Ok(ctx_data.db.to_owned())
}
