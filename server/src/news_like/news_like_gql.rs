use crate::{
    guards::auth_guard::AuthGuard, news_like::news_like_gql_model::NewsLikeGqlModel,
    user::user_service,
};
use async_graphql::{Context, FieldResult, InputObject, Object};
use uuid::Uuid;

use super::news_like_service;

pub struct NewsLikeMutation;
pub struct NewsLikeQuery;

#[derive(InputObject)]
pub struct NewsLikeFindByUserId {
    pub user_id: Uuid,
    pub news_id: Uuid,
}

#[Object]
impl NewsLikeQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_news_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        news_id: Uuid,
    ) -> FieldResult<Vec<NewsLikeGqlModel>> {
        let (_, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        news_like_service::find_by_news_id(&conn, news_id).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn find_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: NewsLikeFindByUserId,
    ) -> FieldResult<Option<NewsLikeGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        news_like_service::find_by_user_id(&conn, data.user_id, data.news_id, request_user).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn find_count_by_news_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        news_id: Uuid,
    ) -> FieldResult<u64> {
        let (_, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        news_like_service::find_count_by_news_id(&conn, news_id).await
    }
}

#[Object]
impl NewsLikeMutation {
    #[graphql(guard = "AuthGuard")]
    async fn like<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        news_id: Uuid,
    ) -> FieldResult<Option<NewsLikeGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        news_like_service::like_news(&conn, news_id, request_user.0.user_id).await
    }
}
