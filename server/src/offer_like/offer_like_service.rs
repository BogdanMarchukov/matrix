use crate::entity::offer_like::Model as OfferLikeModel;
use crate::offer_like::offer_like_repository::OfferLikeRepository;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::offer_like_gql_model::OfferLikeGqlModel;

pub async fn find_by_offer_id(
    db: &DatabaseConnection,
    offer_id: Uuid,
) -> Result<Vec<OfferLikeModel>, sea_orm::DbErr> {
    let result = OfferLikeRepository::find_by_offer_id(db, offer_id).await?;
    let gql_models: Vec<OfferLikeGqlModel> = result
        .into_iter()
        .map(|r| OfferLikeGqlModel::from(r))
        .collect();
    Ok(gql_models)
    Ok(result)
}
