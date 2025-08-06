use async_graphql::{ErrorExtensions, FieldResult, SimpleObject};
use uuid::Uuid;

use crate::{
    entity::offer_like,
    errors::gql_error::GqlError,
    user::user_gql_model::{User, UserRoleGqlType},
};

#[derive(SimpleObject)]
pub struct OfferLikeGqlModel {
    pub offer_like_id: Uuid,
    pub offer_id: Uuid,
    pub user_id: Uuid,
}

impl From<offer_like::Model> for OfferLikeGqlModel {
    fn from(model: offer_like::Model) -> Self {
        OfferLikeGqlModel {
            offer_like_id: model.offer_like_id,
            offer_id: model.offer_id,
            user_id: model.user_id,
        }
    }
}

impl OfferLikeGqlModel {
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
