use super::notify_gql_model::NotifyGqlModel;
use crate::{guards::auth_guard::AuthGuard, user::user_service};
use async_graphql::{Context, FieldResult, InputObject, Object};
use uuid::Uuid;
use super::notify_service;

pub struct NotifyQuery;

#[derive(InputObject)]
pub struct NotifyByUserIdFilter {
    pub user_id: Uuid,
    pub is_read: bool,
}

#[Object]
impl NotifyQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: NotifyByUserIdFilter,
    ) -> FieldResult<Vec<NotifyGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        notify_service::find_many_by_user_id(request_user, data, &conn).await
    }
}
