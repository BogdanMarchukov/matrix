use async_graphql::{ComplexObject, FieldResult, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    config::S3Config,
    db_utils,
    entity::offer,
    offer_like::{offer_like_gql_model::OfferLikeGqlModel, offer_like_service},
    tariff_plan::{tariff_plan_gql_model::TariffPlanGqlModel, tariff_plan_service},
};

#[derive(Clone, SimpleObject)]
#[graphql(complex)]
#[graphql(name = "Offer")]
pub struct OfferGqlModel {
    pub offer_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tariff_ids: Vec<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub img: Option<String>,
}

#[ComplexObject]
impl OfferGqlModel {
    async fn img(&self) -> Option<String> {
        let config = S3Config::new();
        match &self.img {
            Some(img) => Some(format!("{}/{}", config.endpoint, img.to_owned())),
            None => None,
        }
    }

    async fn tariffs(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<Vec<TariffPlanGqlModel>> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        tariff_plan_service::find_by_ids(&self.tariff_ids, &conn).await
    }

    async fn likes(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<Vec<OfferLikeGqlModel>> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        offer_like_service::find_by_offer_id(&conn, self.offer_id).await
    }
}

impl OfferGqlModel {
    pub fn new(offer: offer::Model) -> Self {
        Self {
            offer_id: offer.offer_id,
            title: offer.title,
            description: offer.description,
            tariff_ids: offer.tariff_ids,
            is_active: offer.is_active,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(offer.created_at, Utc),
            img: offer.img,
        }
    }
}
