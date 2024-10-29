use crate::{
    auth::auth_gql::AuthMutation,
    newsletter::newsletter_gql::NewsletterMutation,
    notify::notify_gql::{NotifyMutation, NotifyQuery},
    user::user_gql::UserQuery,
};
use async_graphql::{FieldResult, Object};

pub struct Query;

pub struct Mutation;

pub struct Subscription;

#[Object]
impl Mutation {
    async fn auth(&self) -> FieldResult<AuthMutation> {
        Ok(AuthMutation)
    }

    async fn newsletter(&self) -> FieldResult<NewsletterMutation> {
        Ok(NewsletterMutation)
    }

    async fn notify(&self) -> FieldResult<NotifyMutation> {
        Ok(NotifyMutation)
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
