use crate::{
    guards::auth_guard::AuthGuard, offer_like::offer_like_gql_model::OfferLikeGqlModel,
    user::user_service,
};
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

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
        // допиши реалезацию, метод find_by_offer_id напиши в offer_like_service, используй
        // offer_like_repository AI!

    }

    #[graphql(guard = "AuthGuard")]
    async fn find_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
    ) -> FieldResult<Vec<OfferLikeGqlModel>> {
        let (_, conn) = user_service::get_auth_user_from_ctx(ctx)?;
    }
}
