use crate::{
    errors::gql_error::GqlError, news_like::news_like_repository::NewsLikeRepository,
    user::user_gql_model::User,
};
use async_graphql::{ErrorExtensions, FieldResult};
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

pub async fn find_count_by_news_id(db: &DatabaseConnection, news_id: Uuid) -> FieldResult<u64> {
    match NewsLikeRepository::count(
        db,
        NewsLikeFilter {
            news_id: Some(news_id),
            user_id: None,
        },
    )
    .await
    {
        Ok(count) => Ok(count),
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth::web_app_data::UserTgWebApp, db_utils::TestDb, news::news_repository,
        user::user_repository,
    };
    use testcontainers::clients::Cli;

    #[tokio::test]
    async fn test_find_by_user_id() {
        let docker = Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;

        let news = news_repository::create_test_news(conn)
            .await
            .expect("Create news error");

        let tg_user = UserTgWebApp::test_data(Some(1));
        let user = user_repository::create_one_by_tg(tg_user, conn)
            .await
            .expect("Create user error");

        like_news(conn, news.news_id, user.0.user_id)
            .await
            .expect("Failed to like news");

        let found_like = find_by_user_id(conn, user.0.user_id, news.news_id, user.clone())
            .await
            .expect("Failed to find like");
        assert!(found_like.is_some());

        let unlike_result = like_news(conn, news.news_id, user.0.user_id)
            .await
            .expect("Failed to unlike news");
        assert!(unlike_result.is_none());
    }
}
