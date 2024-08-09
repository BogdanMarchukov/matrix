use crate::auth::auth_service;
use crate::entity::users;
use crate::errors::gql_error::GqlError;
use crate::guards::auth_guard::AuthGuard;
use crate::GqlCtx;
use async_graphql::{Context, ErrorExtensions, FieldResult, InputObject, Object, SimpleObject};

#[derive(InputObject)]
pub struct LoginInput {
    pub init_data: String,
}

#[derive(SimpleObject)]
pub struct LoginResult {
    pub jwt: String,
    pub user: users::Model,
}

pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login<'ctx>(&self, ctx: &Context<'ctx>, data: LoginInput) -> FieldResult<LoginResult> {
        let ctx_data = match ctx.data::<GqlCtx>() {
            Ok(data) => data,
            Err(_) => return Err(GqlError::ServerError("get ctx data errors".to_string()).extend()),
        };
        auth_service::login(data.init_data, &ctx_data.db).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn test(&self, data: i64) -> FieldResult<i64> {
        Ok(data)
    }
}
