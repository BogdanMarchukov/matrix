use crate::{auth::auth_gql::AuthMutation, notify::notify_gql::NotifyQuery, user::user_gql::UserQuery};
use async_graphql::{FieldResult, Object};

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

    async fn notify(&self) -> FieldResult<NotifyQuery> {
        Ok(NotifyQuery)
    }
}
