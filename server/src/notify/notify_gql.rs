use super::notify_gql_model::NotifyGqlModel;
use super::notify_service;
use crate::{gql_schema::Subscription, guards::auth_guard::AuthGuard, user::user_service};
use crate::{AppState, TxSender};
use actix::dev::Stream;
use async_graphql::*;
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, Subscription};
use uuid::Uuid;

pub struct NotifyQuery;

#[derive(InputObject)]
pub struct NotifyByUserIdFilter {
    pub user_id: Uuid,
    pub is_read: bool,
}

#[derive(SimpleObject)]
pub struct NotifySub {
    pub notify_id: Uuid,
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
    async fn messages(&self, ctx: &Context<'_>) -> impl Stream<Item = NotifySub> {
        let data = ctx.data::<AppState>().unwrap();
        let mut sender = data.tx.clone().subscribe();
        async_stream::stream! {
            while let Ok(message) = sender.recv().await {
                yield NotifySub {
                    notify_id: message.id
                }
            }
        }
    }
}
