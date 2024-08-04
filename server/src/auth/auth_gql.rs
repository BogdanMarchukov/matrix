use async_graphql::{Context, FieldResult, Error, InputObject, Object};
use crate::auth::auth_service;
use crate::GqlCtx;

#[derive(InputObject)]
pub struct LoginInput {
    pub init_data: String,
}

pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login<'ctx>(&self, ctx: &Context<'ctx>, data: LoginInput) -> FieldResult<bool> {
        let ctx_data = match ctx.data::<GqlCtx>() {
            Ok(data) => data,
            Err(_) => return Err(Error::new("gql ctx error".to_owned())),
        };
        auth_service::login(data.init_data, &ctx_data.db).await
    }
}
