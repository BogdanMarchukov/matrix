use async_graphql::SimpleObject;
use uuid::Uuid;

use crate::entity::offer_like;

#[derive(SimpleObject)]
pub struct OfferLikeGqlModel {
    pub offer_like_id: Uuid,
    pub offer_id: Uuid,
    pub user_id: Uuid,
}

impl From<offer_like::Model> for OfferLikeGqlModel {
    fn from(model: offer_like::Model) -> Self {
        OfferLikeGqlModel {
            offer_like_id: model.offer_like_id,
            offer_id: model.offer_id,
            user_id: model.user_id,
        }
    }
}

impl OfferLikeGqlModel {
    // напиши как в NotofyGqlModel, обрати внимание на роли Admin и Owner AI!
    pub fn check_role(&self, user_id: Uuid) -> bool {
        self.user_id == user_id
    }
}
