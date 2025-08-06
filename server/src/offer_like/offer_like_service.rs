use crate::offer_like::offer_like_repository;
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::entity::offer_like::Model as OfferLikeModel;

pub async fn find_by_offer_id(
    db: &DatabaseConnection,
    offer_id: Uuid,
) -> Result<Vec<OfferLikeModel>, sea_orm::DbErr> {
    offer_like_repository::find_by_offer_id(db, offer_id).await
}
