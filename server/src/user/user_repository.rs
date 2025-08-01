use super::user_gql_model::{UserGqlModel, User};
use crate::auth::web_app_data;
use crate::entity::prelude::Users;
use crate::entity::users;
use crate::errors::gql_error::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::ActiveValue::Set;
use sea_orm::{Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn find_one(
    filter: Condition,
    conn: &DatabaseConnection,
) -> FieldResult<Option<User>> {
    let user: Option<users::Model> = match Users::find().filter(filter).one(conn).await {
        Ok(u) => u,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).extend()),
    };
    match user {
        Some(u) => Ok(Some(User(UserGqlModel::new(u)))),
        None => Ok(None),
    }
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<users::Model>, DbErr> {
    let users = Users::find().all(conn).await?;
    Ok(users)
}

pub async fn find_by_id(
    user_id: &str,
    conn: &DatabaseConnection,
) -> FieldResult<Option<User>> {
    let uuid = match Uuid::parse_str(user_id) {
        Ok(id) => id,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).extend()),
    };
    let user: Option<users::Model> = match Users::find_by_id(uuid).one(conn).await {
        Ok(u) => u,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).extend()),
    };
    match user {
        Some(u) => Ok(Some(User(UserGqlModel::new(u)))),
        None => Ok(None),
    }
}

pub async fn find_by_uuid(
    user_id: Uuid,
    conn: &DatabaseConnection,
) -> FieldResult<Option<UserGqlModel>> {
    let user: Option<users::Model> = match Users::find_by_id(user_id).one(conn).await {
        Ok(u) => u,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).extend()),
    };
    match user {
        Some(u) => Ok(Some(UserGqlModel::new(u))),
        None => Ok(None),
    }
}

pub async fn create_one_by_tg(
    tg_user: web_app_data::UserTgWebApp,
    conn: &DatabaseConnection,
) -> Result<User, DbErr> {
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
    Ok(User(UserGqlModel::new(result)))
}
