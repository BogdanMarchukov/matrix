use crate::entity::sea_orm_active_enums::UserRoleType;
use crate::entity::users;
use crate::errors::gql_error::GqlError;
use async_graphql::*;
use async_graphql::{FieldResult, SimpleObject};
use uuid::Uuid;

#[derive(SimpleObject, Clone)]
#[graphql(name = "User")]
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

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "UserRoleType", name = "UserRoleType")]
pub enum UserRoleGqlType {
    Admin,
    Member,
    Owner,
}

impl UserGqlModel {
    pub fn check_role(&self, user_id: Uuid) -> FieldResult<&Self> {
        let allowed = match &self.role {
            UserRoleGqlType::Owner => true,
            UserRoleGqlType::Admin => true,
            UserRoleGqlType::Member => &self.user_id == &user_id,
        };
        if allowed {
            return Ok(&self);
        }
        Err(GqlError::Forbidden.into())
    }

    pub fn new(user_model: users::Model ) -> Self {
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
