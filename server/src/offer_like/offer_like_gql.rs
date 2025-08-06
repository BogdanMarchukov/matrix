use crate::{
    guards::auth_guard::AuthGuard, offer_like::offer_like_gql_model::OfferLikeGqlModel,
    user::user_service,
};
use async_graphql::{Context, FieldResult, InputObject, Object};
use uuid::Uuid;

use super::offer_like_service;

pub struct OfferLikeMutation;
pub struct OfferLikeQuery;

#[derive(InputObject)]
pub struct OfferLikeFindByUserId {
    pub user_id: Uuid,
    pub offer_id: Uuid,
}

#[Object]
impl OfferLikeQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_offer_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        offer_id: Uuid,
    ) -> FieldResult<Vec<OfferLikeGqlModel>> {
        let (_, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        offer_like_service::find_by_offer_id(&conn, offer_id).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn find_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: OfferLikeFindByUserId,
    ) -> FieldResult<Option<OfferLikeGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        offer_like_service::find_by_user_id(&conn, data.user_id, data.offer_id, request_user).await
    }
}
