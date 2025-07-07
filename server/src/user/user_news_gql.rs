use crate::{
    gql_schema::Subscription, guards::auth_guard::AuthGuard,
    user::user_news_gql_model::UserNewsGqlModel, GqlCtx, TX_NEWS,
};
use actix::dev::Stream;
use async_graphql::{Context, FieldResult, Object, Subscription};
use uuid::Uuid;

use super::{user_news_service, user_service};

pub struct UserNewsQuery;

#[Object]
impl UserNewsQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
    ) -> FieldResult<Vec<UserNewsGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        user_news_service::find_all_by_user_id(user_id, request_user, &conn).await
    }
}
