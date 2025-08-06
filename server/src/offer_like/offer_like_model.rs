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

    pub async fn create(
        db: &DatabaseConnection,
        offer_id: Uuid,
        user_id: Uuid,
    ) -> Result<offer_like::Model, DbErr> {
        let new_offer_like = offer_like::ActiveModel {
            offer_like_id: Set(Uuid::new_v4()),
            offer_id: Set(offer_id),
            user_id: Set(user_id),
            ..Default::default()
        };
        let res = offer_like::Entity::insert(new_offer_like).exec(db).await?;
        offer_like::Entity::find_by_id(res.last_insert_id).one(db).await
    }
}
