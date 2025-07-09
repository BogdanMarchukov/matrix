use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::{
    news_gql::{NewsCreateInput, NewsUpdateInput},
    news_gql_model::NewsGqlModel,
    news_repository,
};

pub async fn create_one(
    data: NewsCreateInput,
    conn: &DatabaseConnection,
) -> FieldResult<NewsGqlModel> {
    news_repository::create_one(data, conn).await
}

pub async fn update_one(
    news_id: Uuid,
    data: NewsUpdateInput,
    conn: &DatabaseConnection,
) -> FieldResult<NewsGqlModel> {
    news_repository::update_one(news_id, data, conn).await
}

#[cfg(test)]
mod tests {
    use crate::db_utils::TestDb;

    use super::*;

    #[tokio::test]
    async fn test_create_news() {
        let docker = testcontainers::clients::Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;

        let create_data = NewsCreateInput {
            title: "Test News".to_string(),
            payload: "This is a test news".to_string(),
            is_publish: true,
            publish_at: chrono::Utc::now().naive_utc(),
        };

        let created = create_one(create_data.clone(), conn)
            .await
            .expect("Failed to create news");
        assert_eq!(created.title, create_data.title);
        assert_eq!(created.payload, create_data.payload);
        let found = news_repository::find_by_pk(created.news_id, conn)
            .await
            .expect("Failed to find news");
        assert_eq!(created.news_id, found.news_id);

        let update_data = NewsUpdateInput {
            title: Some("Updated Title".to_string()),
            payload: Some("Updated Payload".to_string()),
            is_publish: Some(false),
        };

        let updated = update_one(found.news_id, update_data.clone(), conn)
            .await
            .expect("Failed to update news");
        assert_eq!(updated.title, update_data.title.unwrap());
        assert_eq!(updated.payload, update_data.payload.unwrap());
        assert_eq!(updated.is_publish, update_data.is_publish.unwrap());
    }
}
