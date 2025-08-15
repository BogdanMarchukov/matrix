use async_graphql::{ErrorExtensions, FieldResult, SimpleObject};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use crate::{entity::user_calc_result, errors::gql_error::GqlError};

use super::user_gql_model::{User, UserRoleGqlType};

#[derive(Clone, Debug, SimpleObject)]
#[graphql(name = "UserCalcResult")]
pub struct UserCalcResultGqlModel {
    pub user_calc_result_id: Uuid,
    pub user_id: Uuid,
    pub calculator_id: Uuid,
    pub result: Json,
    pub key: String,
}

impl From<user_calc_result::Model> for UserCalcResultGqlModel {
    fn from(model: user_calc_result::Model) -> Self {
        Self {
            user_calc_result_id: model.user_calc_result_id,
            user_id: model.user_id,
            calculator_id: model.calculator_id,
            result: model.result,
            key: model.key,
        }
    }
}

impl UserCalcResultGqlModel {
    pub fn check_role(&self, user: &User) -> FieldResult<&Self> {
        let allowed = match user.0.role {
            UserRoleGqlType::Owner => true,
            UserRoleGqlType::Admin => true,
            UserRoleGqlType::Member => self.user_id == user.0.user_id,
        };
        if allowed {
            return Ok(self);
        }
        Err(GqlError::Forbidden.extend())
    }
}
