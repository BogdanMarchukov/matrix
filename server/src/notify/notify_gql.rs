use super::notify_gql_model::NotifyGqlModel;
use crate::guards::auth_guard::AuthGuard;
use async_graphql::{Context, FieldResult, InputObject, Object};
use uuid::Uuid;

pub struct NotifyQuery;

#[derive(InputObject)]
struct NotifyByUserIdFilter {
    user_id: Uuid,
    is_read: bool,
}

#[Object]
impl NotifyQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: NotifyByUserIdFilter,
    ) -> FieldResult<bool> {
        Ok(true)
    }
}
