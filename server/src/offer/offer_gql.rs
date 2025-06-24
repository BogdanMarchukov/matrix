use async_graphql::Object;
use async_graphql::*;
use uuid::Uuid;

use super::offer_gql_model::OfferGqlModel;
use super::offer_repository;
use crate::db_utils;
use crate::guards::auth_guard::AuthGuard;
use crate::guards::system_guard::SystemGuard;
use crate::user::user_service;

pub struct OfferMutation;
pub struct OfferQuery;

#[derive(InputObject)]
pub struct OfferCreateData {
    pub title: String,
    pub description: Option<String>,
    pub tariff_ids: Vec<Uuid>,
    pub is_active: bool,
    pub img: Option<String>,
}

#[Object]
impl OfferQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_many<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<Vec<OfferGqlModel>> {
        let (_, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        offer_repository::find_many(&conn, None, None, None).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn find_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        offer_id: Uuid,
    ) -> FieldResult<OfferGqlModel> {
        let (_, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        offer_repository::find_by_pk(offer_id, &conn).await
    }
}

#[Object]
impl OfferMutation {
    #[graphql(guard = "SystemGuard")]
    async fn create_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: OfferCreateData,
    ) -> FieldResult<OfferGqlModel> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        offer_repository::create_one(data, &conn).await
    }
}
