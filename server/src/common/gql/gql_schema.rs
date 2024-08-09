use crate::{auth::auth_gql::AuthMutation, user::user_gql::UserQuery};
use async_graphql::{FieldResult, Object, InputObject};

pub struct Query;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn auth(&self) -> FieldResult<AuthMutation> {
        Ok(AuthMutation)
    }
}

#[Object]
impl Query {
    async fn user(&self) -> FieldResult<UserQuery> {
        Ok(UserQuery)
    }
}
