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
        user_tariff_plan_gql::UserTariffPlanMutation,
    },
};
use async_graphql::{Enum, FieldResult, Object};

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
}
