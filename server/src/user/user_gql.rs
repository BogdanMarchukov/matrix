use super::user_gql_model::User;
use super::user_repository;
use super::user_service;
use crate::errors::gql_error::GqlError;
use crate::guards::auth_guard::AuthGuard;
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_pk<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
    ) -> FieldResult<User> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        let fined_user = user_repository::find_by_uuid(user_id, &conn).await?;
        let result_user = match fined_user {
            Some(u) => u,
            None => return Err(GqlError::NotFound("user not found".to_string()).into()),
        };
        result_user.check_role(&request_user.0.user_id)?;
        Ok(User(result_user))
    }
}
