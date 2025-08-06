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
    use crate::{db_utils::TestDb, user::user_gql_model::UserRoleGqlType};
    use testcontainers::clients::Cli;

    #[tokio::test]
    async fn test_like_offer() {
        let docker = Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;

        let offer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Test liking an offer
        let like_result = like_offer(conn, offer_id, user_id).await.expect("Failed to like offer");
        assert!(like_result.is_some());

        // Test unliking an offer
        let unlike_result = like_offer(conn, offer_id, user_id).await.expect("Failed to unlike offer");
        assert!(unlike_result.is_none());
    }

    #[tokio::test]
    async fn test_find_by_user_id() {
        let docker = Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;

        let offer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let tg_user = UserTgWebApp::test_data(Some(1));
        user_repository::create_one_by_tg(tg_user, conn)
            .await
            .expect("Create user error");

        let user = User {
            0: crate::user::user_gql_model::UserGqlModel {
                user_id,
                role: UserRoleGqlType::Member,
                ..Default::default()
            },
        };

        // Create a like
        like_offer(conn, offer_id, user_id).await.expect("Failed to like offer");

        // Find by user id
        let found_like = find_by_user_id(conn, user_id, offer_id, user).await.expect("Failed to find like");
        assert!(found_like.is_some());
    }
}
