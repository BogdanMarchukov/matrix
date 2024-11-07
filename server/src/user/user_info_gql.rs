use crate::guards::auth_guard::AuthGuard;
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

use super::{
    types::UserInfoUpdateInput, user_info_gql_model::UserInfoGqlModel, user_info_service,
    user_service,
};

pub struct UserInfoQuery;

#[Object]
impl UserInfoQuery {
    #[graphql(guard = "AuthGuard")]
    async fn fund_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
    ) -> FieldResult<UserInfoGqlModel> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        user_info_service::find_by_user_id(user_id, request_user, &conn).await
    }
}

pub struct UserInfoMutation;

#[Object]
impl UserInfoMutation {
    #[graphql(guard = "AuthGuard")]
    async fn update_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_info_id: Uuid,
        data: UserInfoUpdateInput,
    ) -> FieldResult<UserInfoGqlModel> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        user_info_service::update_one(user_info_id, request_user, data, &conn).await
    }
}
