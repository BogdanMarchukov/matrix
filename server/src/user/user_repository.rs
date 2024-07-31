use crate::entity::prelude::Users;
use crate::entity::users;

use sea_orm::{Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn user_find_one(
    filter: Condition,
    conn: &DatabaseConnection,
) -> Result<Option<users::Model>, DbErr> {
    let user: Option<users::Model> = Users::find().filter(filter).one(conn).await?;
    Ok(user)
}
