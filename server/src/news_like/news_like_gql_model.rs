use async_graphql::{ErrorExtensions, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::gql_error::GqlError;

#[derive(SimpleObject)]
pub struct NewsLikeGqlModel {
    pub news_like_id: Uuid,
    pub news_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<crate::entity::news_like::Model> for NewsLikeGqlModel {
    fn from(model: crate::entity::news_like::Model) -> Self {
        NewsLikeGqlModel {
            news_like_id: model.news_like_id,
            news_id: model.news_id,
            user_id: model.user_id,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(model.created_at, Utc),
        }
    }
}

impl NewsLikeGqlModel {
    pub fn check_role(
        &self,
        user: &crate::user::user_gql_model::User,
    ) -> async_graphql::FieldResult<&Self> {
        let allowed = match user.0.role {
            crate::user::user_gql_model::UserRoleGqlType::Owner => true,
            crate::user::user_gql_model::UserRoleGqlType::Admin => true,
            crate::user::user_gql_model::UserRoleGqlType::Member => self.user_id == user.0.user_id,
        };
        if allowed {
            return Ok(self);
        }
        Err(GqlError::Forbidden.extend())
    }
}
