use async_graphql::{FieldResult, InputObject, Object};
use crate::auth::auth_service;

#[derive(InputObject)]
pub struct LoginInput {
    pub init_data: String,
    pub hash: String,
}

pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login(&self, data: LoginInput) -> FieldResult<bool> {
        auth_service::login(data)
    }
}
