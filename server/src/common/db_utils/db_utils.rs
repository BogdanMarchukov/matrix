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

pub async fn get_transaction() -> Result<DatabaseTransaction, GqlError> {
    match get_pool().await {
        Ok(db) => match db.begin().await {
            Ok(transaction) => Ok(transaction),
            Err(_) => Err(GqlError::ServerError("database error".to_string())),
        },
        Err(_) => Err(GqlError::ServerError("database error".to_string())),
    }
}

pub fn get_connection_from_gql_ctx(ctx: &Context) -> FieldResult<DatabaseConnection> {
    let ctx_data = match ctx.data::<GqlCtx>() {
        Ok(data) => data,
        Err(_) => return Err(GqlError::ServerError("get cxt error".to_string()).into()),
    };
    Ok(ctx_data.db.to_owned())
}
