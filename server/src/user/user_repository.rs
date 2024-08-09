use crate::auth::web_app_data;
use crate::entity::prelude::Users;
use crate::entity::users;
use crate::errors::gql_error::GqlError;
use async_graphql::FieldResult;
use uuid::Uuid;

use sea_orm::ActiveValue::Set;
use sea_orm::{Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn find_one(
    filter: Condition,
    conn: &DatabaseConnection,
) -> FieldResult<Option<users::Model>> {
    let user: Option<users::Model> = match Users::find().filter(filter).one(conn).await {
        Ok(u) => u,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).into()),
    };
    Ok(user)
}

pub async fn find_by_id(
    user_id: &String,
    conn: &DatabaseConnection,
) -> FieldResult<Option<users::Model>> {
    let uuid = match Uuid::parse_str(&user_id[..]) {
        Ok(id) => id,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).into()),
    };
    let user: Option<users::Model> = match Users::find_by_id(uuid).one(conn).await {
        Ok(u) => u,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).into()),
    };
    Ok(user)
}

pub async fn find_by_uuid(
    user_id: Uuid,
    conn: &DatabaseConnection,
) -> FieldResult<Option<users::Model>> {
    let user: Option<users::Model> = match Users::find_by_id(user_id).one(conn).await {
        Ok(u) => u,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).into()),
    };
    Ok(user)
}

pub async fn create_one_by_tg(
    tg_user: web_app_data::UserTgWebApp,
    conn: &DatabaseConnection,
) -> Result<users::Model, DbErr> {
    let new_user = users::ActiveModel {
        user_id: Set(Uuid::new_v4()),
        telegram_id: Set(tg_user.id),
        first_name: Set(Some(tg_user.first_name)),
        last_name: Set(Some(tg_user.last_name)),
        username: Set(tg_user.username),
        language_code: Set(tg_user.language_code),
        is_premium: Set(tg_user.is_premium),
        photo_url: Set(tg_user.photo_url),
        ..Default::default()
    };

    let result: users::Model = users::Entity::insert(new_user)
        .exec_with_returning(conn)
        .await?;
    Ok(result)
}
