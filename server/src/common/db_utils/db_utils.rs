use actix::{Actor, SyncContext};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    use crate::config;
    let db_url = config::get_database_url();
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
