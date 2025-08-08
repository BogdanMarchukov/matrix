use crate::{news_like::news_like_repository::NewsLikeRepository, user::user_gql_model::User};
use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::{news_like_gql_model::NewsLikeGqlModel, news_like_repository::NewsLikeFilter};

pub async fn find_by_news_id(
    db: &DatabaseConnection,
    news_id: Uuid,
) -> FieldResult<Vec<NewsLikeGqlModel>> {
    let result = NewsLikeRepository::find_many(
        db,
        NewsLikeFilter {
            news_id: Some(news_id),
            user_id: None,
        },
    )
    .await?;
    Ok(result.into_iter().map(NewsLikeGqlModel::from).collect())
}

pub async fn find_by_user_id(
    db: &DatabaseConnection,
    user_id: Uuid,
    news_id: Uuid,
    user: User,
) -> FieldResult<Option<NewsLikeGqlModel>> {
    if let Ok(Some(news)) = NewsLikeRepository::find_one(
        db,
        NewsLikeFilter {
            news_id: Some(news_id),
            user_id: Some(user_id),
        },
    )
    .await
    {
        let result = NewsLikeGqlModel::from(news);
        result.check_role(&user)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

pub async fn like_news(
    db: &DatabaseConnection,
    news_id: Uuid,
    user_id: Uuid,
) -> FieldResult<Option<NewsLikeGqlModel>> {
    if let Ok(Some(existing_like)) = NewsLikeRepository::find_one(
        db,
        NewsLikeFilter {
            news_id: Some(news_id),
            user_id: Some(user_id),
        },
    )
    .await
    {
        NewsLikeRepository::delete_one(db, existing_like.news_like_id).await?;
        return Ok(None);
    } else {
        let new_like = NewsLikeRepository::create_one(db, news_id, user_id).await?;
        return Ok(Some(NewsLikeGqlModel::from(new_like)));
    }
}
