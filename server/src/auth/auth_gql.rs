use crate::{
    auth::auth_service, guards::system_guard::SystemGuard, helpers,
    user::user_gql_model::User,
};
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject};
use uuid::Uuid;

#[derive(InputObject)]
pub struct LoginInput {
    pub init_data: String,
}

#[derive(SimpleObject)]
pub struct LoginResult {
    pub jwt: String,
    pub user: User,
}

#[derive(InputObject)]
pub struct DevLoginInput {
    pub user_id: Uuid,
}

pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login<'ctx>(&self, ctx: &Context<'ctx>, data: LoginInput) -> FieldResult<LoginResult> {
        let ctx_data = helpers::ctx::get_ctx_data(ctx)?;
        auth_service::login(data.init_data, &ctx_data.db).await
    }

    #[graphql(guard = "SystemGuard")]
    async fn dev_login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: DevLoginInput,
    ) -> FieldResult<LoginResult> {
        let ctx_data = helpers::ctx::get_ctx_data(ctx)?;
        auth_service::dev_login(data.user_id, &ctx_data.db).await
    }
}
