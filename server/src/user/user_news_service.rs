use async_graphql::{ErrorExtensions, FieldResult};
use migration::{Expr, IntoCondition};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{entity::user_news, errors::gql_error::GqlError};

use super::{user_gql_model::User, user_news_gql_model::UserNewsGqlModel, user_news_repository};

pub async fn find_all_by_user_id(
    user_id: Uuid,
    request_user: User,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<UserNewsGqlModel>> {
    let condition = Expr::col(user_news::Column::UserId)
        .eq(user_id)
        .into_condition();
    let user_news = match user_news_repository::find_many(conn, Some(condition)).await {
        Ok(user_news) => user_news,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).extend()),
    };
    let mut result: Vec<UserNewsGqlModel> = vec![];

    for u in user_news {
        match UserNewsGqlModel::new(u).check_role(&request_user) {
            Ok(user_news) => result.push(user_news.to_owned()),
            Err(err) => return Err(err),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        auth::web_app_data::UserTgWebApp,
        db_utils::TestDb,
        news::{news_gql::NewsCreateInput, news_repository},
        user::user_repository,
    };

    use super::*;

    #[tokio::test]
    async fn test_find_all_by_user_id() {
        let docker = testcontainers::clients::Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;
        let crete_user_data = UserTgWebApp::test_data(None);

        let user = user_repository::create_one_by_tg(crete_user_data, conn)
            .await
            .expect("create user error");
        let create_news_data = NewsCreateInput {
            title: "test news".to_string(),
            payload: "test payload".to_string(),
            is_publish: true,
            publish_at: chrono::Utc::now().naive_utc(),
        };

        let news = news_repository::create_one(create_news_data, conn)
            .await
            .expect("create news error");

        let result = news_repository::create_for_all_users(&news, conn).await;
        assert_eq!(result, true);

        let result = find_all_by_user_id(user.0.user_id, user.clone(), conn)
            .await
            .expect("find news error");
        assert_eq!(result.len(), 1);

        let result = news_repository::create_for_all_users(&news, conn).await;
        assert_eq!(result, true);

        let result = find_all_by_user_id(user.0.user_id, user, conn)
            .await
            .expect("find news error");
        assert_eq!(result.len(), 1);
    }
}
