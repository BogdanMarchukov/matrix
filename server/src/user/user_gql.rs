use async_graphql::{Context, FieldResult, Object};
use crate::guards::auth_guard::AuthGuard;

pub struct UserQuery;

#[Object]
impl UserQuery {

    #[graphql(guard = "AuthGuard")]
    async fn find_by_pk<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<bool> {
        Ok(true)
    }
}
