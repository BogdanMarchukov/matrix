use async_graphql::{Context, FieldResult};
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DatabaseTransaction, DbErr,
    TransactionTrait,
};
use testcontainers::{clients::Cli, images::postgres::Postgres, Container, RunnableImage};

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

pub struct TestDb<'a> {
    pub db: DatabaseConnection,
    pub _container: Container<'a, Postgres>,
    pub _docker: &'a Cli,
}

impl<'a> TestDb<'a> {
    pub async fn new(docker: &'a Cli) -> Self {
        let pg_image = RunnableImage::from(Postgres::default());
        let container = docker.run(pg_image);
        let port = container.get_host_port_ipv4(5432);
        let db_url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);
        let db = Database::connect(db_url).await.expect("DB connect failed");
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS pgcrypto")
            .await
            .expect("Enable pgcrypto");
        Migrator::up(&db, None).await.expect("Migration failed");

        Self {
            db,
            _container: container,
            _docker: docker,
        }
    }
}
