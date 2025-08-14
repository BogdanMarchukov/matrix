use crate::calculator::calculator_gql::CalculatorMutation;
use crate::calculator::calculator_gql::CalculatorQuery;
use crate::guards::auth_guard::AuthGuard;
use crate::news_like::news_like_gql::NewsLikeMutation;
use crate::news_like::news_like_gql::NewsLikeQuery;
use crate::notify::notify_gql::NotifySub;
use crate::notify::notify_gql::UserNewsSub;
use crate::offer_like::offer_like_gql::OfferLikeMutation;
use crate::offer_like::offer_like_gql::OfferLikeQuery;
use crate::user::user_news_gql::UserNewsMutation;
use crate::GqlCtx;
use crate::TX_NEWS;
use crate::TX_NOTIFY;
use crate::{
    auth::auth_gql::AuthMutation,
    news::news_gql::NewsMutation,
    newsletter::newsletter_gql::NewsletterMutation,
    notify::notify_gql::{NotifyMutation, NotifyQuery},
    offer::offer_gql::{OfferMutation, OfferQuery},
    tariff_plan::tariff_plan_gql::TariffPlanMutation,
    user::{
        user_gql::UserQuery,
        user_info_gql::{UserInfoMutation, UserInfoQuery},
        user_news_gql::UserNewsQuery,
        user_tariff_plan_gql::UserTariffPlanMutation,
    },
};
use actix::dev::Stream;
use async_graphql::async_stream::stream;
use async_graphql::{Context, Enum, FieldResult, Object, Subscription};

pub struct Query;

pub struct Mutation;

pub struct Subscription;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum GqlOrder {
    Asc,
    Desc,
}

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

    async fn user_info(&self) -> FieldResult<UserInfoMutation> {
        Ok(UserInfoMutation)
    }

    async fn offer(&self) -> FieldResult<OfferMutation> {
        Ok(OfferMutation)
    }

    async fn tariff_plan(&self) -> FieldResult<TariffPlanMutation> {
        Ok(TariffPlanMutation)
    }

    async fn user_tariff_plan(&self) -> FieldResult<UserTariffPlanMutation> {
        Ok(UserTariffPlanMutation)
    }

    async fn news(&self) -> FieldResult<NewsMutation> {
        Ok(NewsMutation)
    }

    async fn user_news(&self) -> FieldResult<UserNewsMutation> {
        Ok(UserNewsMutation)
    }

    async fn offer_like(&self) -> FieldResult<OfferLikeMutation> {
        Ok(OfferLikeMutation)
    }

    async fn news_like(&self) -> FieldResult<NewsLikeMutation> {
        Ok(NewsLikeMutation)
    }

    async fn calculator(&self) -> FieldResult<CalculatorMutation> {
        Ok(CalculatorMutation)
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

    async fn user_info(&self) -> FieldResult<UserInfoQuery> {
        Ok(UserInfoQuery)
    }

    async fn offer(&self) -> FieldResult<OfferQuery> {
        Ok(OfferQuery)
    }

    async fn user_news(&self) -> FieldResult<UserNewsQuery> {
        Ok(UserNewsQuery)
    }

    async fn offer_like(&self) -> FieldResult<OfferLikeQuery> {
        Ok(OfferLikeQuery)
    }

    async fn news_like(&self) -> FieldResult<NewsLikeQuery> {
        Ok(NewsLikeQuery)
    }

    async fn calculator(&self) -> FieldResult<CalculatorQuery> {
        Ok(CalculatorQuery)
    }
}

#[Subscription]
impl Subscription {
    #[graphql(guard = "AuthGuard")]
    async fn notify_delay(&self, ctx: &Context<'_>) -> impl Stream<Item = NotifySub> {
        let data = ctx.data::<GqlCtx>().unwrap();
        let mut sender = TX_NOTIFY.to_owned().subscribe();
        let user = data.user.to_owned();
        stream! {
            while let Ok(message) = sender.recv().await {
                if let Some(user) = user.to_owned() {
                    if user.0.user_id == message.user_id  {
                        yield NotifySub {
                           notify_id: message.id
                        }
                    }
                }
            }
        }
    }

    #[graphql(guard = "AuthGuard")]
    async fn user_news(&self, ctx: &Context<'_>) -> impl Stream<Item = UserNewsSub> {
        let data = ctx.data::<GqlCtx>().unwrap();
        let mut sender = TX_NEWS.to_owned().subscribe();
        let user = data.user.to_owned();
        stream! {
            while let Ok(message) = sender.recv().await {
                if let Some(user) = user.to_owned() {
                    if user.0.user_id == message.user_id  {
                        yield UserNewsSub {
                           user_news_id: message.id
                        }
                    }
                }
            }
        }
    }
}
