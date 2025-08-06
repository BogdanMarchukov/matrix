use sea_orm::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entity::offer_like;

pub struct OfferLikeRepository;

impl OfferLikeRepository {
    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<offer_like::Model>, DbErr> {
        offer_like::Entity::find()
            .filter(offer_like::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn find_by_offer_id(
        db: &DatabaseConnection,
        offer_id: Uuid,
    ) -> Result<Vec<offer_like::Model>, DbErr> {
        offer_like::Entity::find()
            .filter(offer_like::Column::OfferId.eq(offer_id))
            .all(db)
            .await
    }

    pub async fn create_one(
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
