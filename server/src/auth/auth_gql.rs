use crate::errors::gql_error::GqlError;
use crate::GqlCtx;
use crate::{auth::auth_service, user::user_gql_model::UserGqlModel};
use async_graphql::{Context, ErrorExtensions, FieldResult, InputObject, Object, SimpleObject};

#[derive(InputObject)]
pub struct LoginInput {
    pub init_data: String,
}

#[derive(SimpleObject)]
pub struct LoginResult {
    pub jwt: String,
    pub user: UserGqlModel,
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
}
