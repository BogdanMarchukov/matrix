use crate::db_utils::get_connection_from_gql_ctx;
use crate::entity::sea_orm_active_enums::UserRoleType;
use crate::entity::users;
use crate::errors::gql_error::GqlError;
use async_graphql::FieldResult;
use async_graphql::*;
use uuid::Uuid;

use super::user_info_gql_model::UserInfoGqlModel;
use super::user_info_service::find_by_user_id;

#[derive(Clone, Debug)]
pub struct UserGqlModel {
    pub user_id: Uuid,
    pub telegram_id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub photo_url: Option<String>,
    pub role: UserRoleGqlType,
}

#[derive(Clone)]
pub struct User(pub UserGqlModel);

#[Object]
impl User {
    async fn user_info(&self, ctx: &Context<'_>) -> FieldResult<UserInfoGqlModel> {
        let conn = get_connection_from_gql_ctx(ctx)?;
        find_by_user_id(&self.0.user_id, &self, &conn).await
    }

    async fn user_id(&self) -> &Uuid {
        &self.0.user_id
    }

    async fn telegram_id(&self) -> &i64 {
        &self.0.telegram_id
    }

    async fn first_name(&self) -> &Option<String> {
        &self.0.first_name
    }

    async fn last_name(&self) -> &Option<String> {
        &self.0.last_name
    }

    async fn username(&self) -> &Option<String> {
        &self.0.username
    }

    async fn language_code(&self) -> &Option<String> {
        &self.0.language_code
    }

    async fn is_premium(&self) -> &Option<bool> {
        &self.0.is_premium
    }

    async fn photo_url(&self) -> &Option<String> {
        &self.0.photo_url
    }

    async fn role(&self) -> &UserRoleGqlType {
        &self.0.role
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(remote = "UserRoleType", name = "UserRoleType")]
pub enum UserRoleGqlType {
    Admin,
    Member,
    Owner,
}

impl UserGqlModel {
    pub fn check_role(&self, user_id: &Uuid) -> FieldResult<&Self> {
        let allowed = match &self.role {
            UserRoleGqlType::Owner => true,
            UserRoleGqlType::Admin => true,
            UserRoleGqlType::Member => &self.user_id == user_id,
        };
        if allowed {
            return Ok(&self);
        }
        Err(GqlError::Forbidden.into())
    }

    pub fn new(user_model: users::Model) -> Self {
        Self {
            user_id: user_model.user_id,
            telegram_id: user_model.telegram_id,
            last_name: user_model.last_name,
            username: user_model.username,
            language_code: user_model.language_code,
            is_premium: user_model.is_premium,
            photo_url: user_model.photo_url,
            first_name: user_model.first_name,
            role: user_model.role.into(),
        }
    }
}
