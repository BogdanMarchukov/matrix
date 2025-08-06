use crate::{offer_like::offer_like_repository::OfferLikeRepository, user::user_gql_model::User};
use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::offer_like_gql_model::OfferLikeGqlModel;

pub async fn find_by_offer_id(
    db: &DatabaseConnection,
    offer_id: Uuid,
) -> FieldResult<Vec<OfferLikeGqlModel>> {
    let result = OfferLikeRepository::find_by_offer_id(db, offer_id).await?;
    // исправь маппинг AI!
    Ok(result.into_iter().map(OfferLikeGqlModel::from).collect())
}

pub async fn find_by_user_id(
    db: &DatabaseConnection,
    user_id: Uuid,
    user: User,
) -> FieldResult<Option<OfferLikeGqlModel>> {
    if let Ok(Some(offer)) = OfferLikeRepository::find_by_user_id(db, user_id).await {
        let result = OfferLikeGqlModel::from(offer);
        result.check_role(&user)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}
