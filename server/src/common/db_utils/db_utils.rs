use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};

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
