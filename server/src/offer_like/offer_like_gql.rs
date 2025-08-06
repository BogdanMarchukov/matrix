use crate::{
    guards::auth_guard::AuthGuard, offer_like::offer_like_gql_model::OfferLikeGqlModel,
    user::user_service,
};
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

use super::offer_like_service;

pub struct OfferLikeMutation;
pub struct OfferLikeQuery;

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
        user_id: Uuid,
    ) -> FieldResult<Option<OfferLikeGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        offer_like_service::find_by_user_id(&conn, user_id, request_user).await
    }
}
