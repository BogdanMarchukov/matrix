use super::user_gql_model::{UserGqlModel, UserRoleGqlType};
use crate::entity::user_info;
use crate::errors::gql_error::GqlError;
use async_graphql::*;
use async_graphql::{FieldResult, SimpleObject};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug)]
#[graphql(name = "UserInfo")]
pub struct UserInfoGqlModel {
    pub user_info_id: Uuid,
    pub city: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub time_of_birth: Option<NaiveDate>,
    pub user_id: Uuid,
}

impl UserInfoGqlModel {
    pub fn check_role(&self, user: &UserGqlModel) -> FieldResult<&Self> {
        let allowed = match &user.role {
            UserRoleGqlType::Owner => true,
            UserRoleGqlType::Admin => true,
            UserRoleGqlType::Member => &self.user_id == &user.user_id,
        };
        if allowed {
            return Ok(&self);
        }
        Err(GqlError::Forbidden.into())
    }

    pub fn new(user_info: user_info::Model) -> Self {
        Self {
            user_info_id: user_info.user_info_id,
            city: user_info.city,
            date_of_birth: user_info.date_of_birth,
            time_of_birth: user_info.time_of_birth,
            user_id: user_info.user_id,
        }
    }
}
