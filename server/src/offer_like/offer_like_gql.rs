use async_graphql::{Object, SimpleObject, ID};
use uuid::Uuid;

use crate::entity::offer_like;
use crate::db_utils;
use crate::guards::auth_guard::AuthGuard;
use crate::guards::system_guard::SystemGuard;

pub struct OfferLikeMutation;
pub struct OfferLikeQuery;

#[derive(SimpleObject)]
pub struct OfferLikeGql {
    pub offer_like_id: ID,
    pub offer_id: ID,
    pub user_id: ID,
}

impl From<offer_like::Model> for OfferLikeGql {
    fn from(model: offer_like::Model) -> Self {
        OfferLikeGql {
            offer_like_id: ID::from(model.offer_like_id.to_string()),
            offer_id: ID::from(model.offer_id.to_string()),
            user_id: ID::from(model.user_id.to_string()),
        }
    }
}

#[Object]
impl OfferLikeQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        offer_like_id: Uuid,
    ) -> FieldResult<OfferLikeGql> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        let offer_like = offer_like::Entity::find_by_id(offer_like_id)
            .one(&conn)
            .await?
            .ok_or_else(|| "OfferLike not found")?;
        Ok(offer_like.into())
    }
}

#[Object]
impl OfferLikeMutation {
    #[graphql(guard = "SystemGuard")]
    async fn create_offer_like<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        offer_id: Uuid,
        user_id: Uuid,
    ) -> FieldResult<OfferLikeGql> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        let new_offer_like = offer_like::ActiveModel {
            offer_like_id: Set(Uuid::new_v4()),
            offer_id: Set(offer_id),
            user_id: Set(user_id),
            ..Default::default()
        };
        let offer_like = offer_like::Entity::insert(new_offer_like)
            .exec(&conn)
            .await?;
        Ok(offer_like.last_insert_id.into())
    }
}
