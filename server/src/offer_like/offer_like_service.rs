use crate::{offer_like::offer_like_repository::OfferLikeRepository, user::user_gql_model::User};
use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::{offer_like_gql_model::OfferLikeGqlModel, offer_like_repository::OfferLikeFilter};

pub async fn find_by_offer_id(
    db: &DatabaseConnection,
    offer_id: Uuid,
) -> FieldResult<Vec<OfferLikeGqlModel>> {
    let result = OfferLikeRepository::find_many(
        db,
        OfferLikeFilter {
            offer_id: Some(offer_id),
            user_id: None,
        },
    )
    .await?;
    Ok(result.into_iter().map(OfferLikeGqlModel::from).collect())
}

pub async fn find_by_user_id(
    db: &DatabaseConnection,
    user_id: Uuid,
    offer_id: Uuid,
    user: User,
) -> FieldResult<Option<OfferLikeGqlModel>> {
    if let Ok(Some(offer)) = OfferLikeRepository::find_one(
        db,
        OfferLikeFilter {
            offer_id: Some(offer_id),
            user_id: Some(user_id),
        },
    )
    .await
    {
        let result = OfferLikeGqlModel::from(offer);
        result.check_role(&user)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

pub async fn like_offer(
    db: &DatabaseConnection,
    offer_id: Uuid,
    user_id: Uuid,
) -> FieldResult<Option<OfferLikeGqlModel>> {
    if let Ok(Some(existing_like)) = OfferLikeRepository::find_one(
        db,
        OfferLikeFilter {
            offer_id: Some(offer_id),
            user_id: Some(user_id),
        },
    )
    .await
    {
        OfferLikeRepository::delete_one(db, existing_like.offer_like_id).await?;
        return Ok(None);
    } else {
        let new_like = OfferLikeRepository::create_one(db, offer_id, user_id).await?;
        return Ok(Some(OfferLikeGqlModel::from(new_like)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth::web_app_data::UserTgWebApp, db_utils::TestDb, offer::offer_repository,
        user::user_repository,
    };
    use testcontainers::clients::Cli;

    #[tokio::test]
    async fn test_find_by_user_id() {
        let docker = Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;

        let offer = offer_repository::create_test_offer(conn)
            .await
            .expect("Create offer error");

        let tg_user = UserTgWebApp::test_data(Some(1));
        let user = user_repository::create_one_by_tg(tg_user, conn)
            .await
            .expect("Create user error");

        like_offer(conn, offer.offer_id, user.0.user_id)
            .await
            .expect("Failed to like offer");

        let found_like = find_by_user_id(conn, user.0.user_id, offer.offer_id, user.clone())
            .await
            .expect("Failed to find like");
        assert!(found_like.is_some());

        let unlike_result = like_offer(conn, offer.offer_id, user.0.user_id)
            .await
            .expect("Failed to unlike offer");
        assert!(unlike_result.is_none());
    }
}
