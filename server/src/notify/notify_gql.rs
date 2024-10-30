use super::notify_gql_model::{NotifyGqlModel, NotifyTypeGql};
use super::{notify_repository, notify_service};
use crate::guards::auth_guard::AuthGuard;
use crate::{gql_schema::Subscription, user::user_service};
use crate::{GqlCtx, TX_NOTIFY};
use actix::dev::Stream;
use async_graphql::*;
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, Subscription};
use uuid::Uuid;

pub struct NotifyQuery;
pub struct NotifyMutation;

#[derive(InputObject)]
pub struct NotifyByUserIdFilter {
    pub user_id: Uuid,
    pub is_read: Option<bool>,
    pub notify_type: Option<NotifyTypeGql>,
}

#[derive(SimpleObject)]
pub struct NotifySub {
    pub notify_id: Uuid,
}

#[derive(InputObject)]
pub struct NotifyUpdateData {
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

    #[graphql(guard = "AuthGuard")]
    async fn find_by_pk<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        notify_id: Uuid,
    ) -> FieldResult<NotifyGqlModel> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        notify_service::find_by_pk(notify_id, request_user, &conn).await
    }
}

#[Object]
impl NotifyMutation {
    #[graphql(guard = "AuthGuard")]
    async fn update_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        notify_id: Uuid,
        data: NotifyUpdateData,
    ) -> FieldResult<NotifyGqlModel> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        notify_service::update_one(notify_id, data, request_user, &conn).await
    }
}

#[Subscription]
impl Subscription {
    #[graphql(guard = "AuthGuard")]
    async fn notify_delay(&self, ctx: &Context<'_>) -> impl Stream<Item = NotifySub> {
        let data = ctx.data::<GqlCtx>().unwrap();
        let mut sender = TX_NOTIFY.clone().subscribe();
        let user = data.user.to_owned();
        async_stream::stream! {
            while let Ok(message) = sender.recv().await {
                if let Some(user) = user.to_owned() {
                    if user.user_id == message.user_id  {
                        yield NotifySub {
                           notify_id: message.id
                        }
                    }
                }
            }
        }
    }
}
