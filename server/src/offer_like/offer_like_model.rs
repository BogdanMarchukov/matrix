use crate::entity::offer_like;
use sea_orm::prelude::*;

pub struct OfferLikeModel;

impl OfferLikeModel {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<offer_like::Model>, DbErr> {
        offer_like::Entity::find_by_id(id).one(db).await
    }
}
