use async_graphql::{Context, FieldResult, Error, InputObject, Object, SimpleObject};
use crate::auth::auth_service;
use crate::GqlCtx;
use crate::entity::users;

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
            Err(_) => return Err(Error::new("gql ctx error".to_owned())),
        };
        auth_service::login(data.init_data, &ctx_data.db).await
    }
}
