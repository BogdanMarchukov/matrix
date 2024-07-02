use crate::{models::User, schema::users::first_name};
use actix::fut::ok;
use diesel::{connection, prelude::*};

async fn main(connection: &mut PgConnection) -> diesel::QueryResult<User> {
    use crate::schema::users::dsl::*;

    let results = users
        .filter(last_name.eq("kjik"))
        .first::<User>(connection)
        .expect("Error loading posts");

    Ok(results)
}

async fn get_user(connection: &mut PgConnection) {
    let user = main(connection).await;
}
