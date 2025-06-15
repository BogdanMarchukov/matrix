use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entity::offer;

#[derive(Clone, SimpleObject)]
#[graphql(name = "Offer")]
pub struct OfferGqlModel {
    pub offer_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tariff_ids: Vec<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub img: Option<String>,
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
