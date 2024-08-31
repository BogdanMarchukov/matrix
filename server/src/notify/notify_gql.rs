use super::notify_gql_model::NotifyGqlModel;
use super::notify_service;
use crate::{gql_schema::Subscription, guards::auth_guard::AuthGuard, user::user_service};
use actix::dev::Stream;
use async_graphql::*;
use async_graphql::{Context, FieldResult, InputObject, Object, Subscription};
use tokio::sync::broadcast::Sender;
use uuid::Uuid;

pub struct NotifyQuery;

#[derive(InputObject)]
pub struct NotifyByUserIdFilter {
    pub user_id: Uuid,
    pub is_read: bool,
}

#[Object]
impl NotifyQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_by_user_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: NotifyByUserIdFilter,
    ) -> FieldResult<Vec<NotifyGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        notify_service::find_many_by_user_id(request_user, data, &conn).await
    }
}

#[Subscription]
impl Subscription {
    async fn messages(&self, ctx: &Context<'_>) -> impl Stream<Item = String> {
        let mut sender = ctx.data::<Sender<String>>().unwrap().subscribe();

        async_stream::stream! {
            loop {
                match sender.recv().await {
                    Ok(message) => yield message,
                    Err(_) => break,
                }
            }
        }
    }
}
