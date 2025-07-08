use crate::{guards::auth_guard::AuthGuard, user::user_news_gql_model::UserNewsGqlModel};
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

use super::{user_news_service, user_service};

pub struct UserNewsQuery;
pub struct UserNewsMutation;

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

    async fn find_by_pk<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_news_id: Uuid,
    ) -> FieldResult<UserNewsGqlModel> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        user_news_service::find_by_pk(user_news_id, request_user, &conn).await
    }
}

#[Object]
impl UserNewsMutation {
    #[graphql(guard = "AuthGuard")]
    async fn read_increment<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_news_id: Uuid,
    ) -> FieldResult<UserNewsGqlModel> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        user_news_service::reading_increment(user_news_id, request_user, &conn).await
    }
}
