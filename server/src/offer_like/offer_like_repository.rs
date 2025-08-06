use sea_orm::{prelude::*, Set};
use sea_orm::{EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entity::offer_like;

pub struct OfferLikeFilter {
    pub offer_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

pub struct OfferLikeRepository;

impl OfferLikeRepository {

    pub async fn create_one<C>(
        db: &C,
        offer_id: Uuid,
        user_id: Uuid,
    ) -> Result<offer_like::Model, DbErr>
    where
        C: ConnectionTrait,
    {
        let new_offer_like = offer_like::ActiveModel {
            offer_like_id: Set(Uuid::new_v4()),
            offer_id: Set(offer_id),
            user_id: Set(user_id),
            ..Default::default()
        };
        offer_like::Entity::insert(new_offer_like)
            .exec_with_returning(db)
            .await
    }

    pub async fn find_many<C>(
        db: &C,
        filter: OfferLikeFilter,
    ) -> Result<Vec<offer_like::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut query = offer_like::Entity::find();

        if let Some(offer_id) = filter.offer_id {
            query = query.filter(offer_like::Column::OfferId.eq(offer_id));
        }

        if let Some(user_id) = filter.user_id {
            query = query.filter(offer_like::Column::UserId.eq(user_id));
        }

        query.all(db).await
    }

    pub async fn find_one<C>(
        db: &C,
        filter: OfferLikeFilter,
    ) -> Result<Option<offer_like::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut query = offer_like::Entity::find();

        if let Some(offer_id) = filter.offer_id {
            query = query.filter(offer_like::Column::OfferId.eq(offer_id));
        }

        if let Some(user_id) = filter.user_id {
            query = query.filter(offer_like::Column::UserId.eq(user_id));
        }

        query.one(db).await
    }
}
