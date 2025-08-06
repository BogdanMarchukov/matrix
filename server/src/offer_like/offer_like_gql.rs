use async_graphql::{SimpleObject, ID};
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct OfferLikeGql {
    pub offer_like_id: ID,
    pub offer_id: ID,
    pub user_id: ID,
}

impl From<crate::entity::offer_like::Model> for OfferLikeGql {
    fn from(model: crate::entity::offer_like::Model) -> Self {
        OfferLikeGql {
            offer_like_id: ID::from(model.offer_like_id.to_string()),
            offer_id: ID::from(model.offer_id.to_string()),
            user_id: ID::from(model.user_id.to_string()),
        }
    }
}
