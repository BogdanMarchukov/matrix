use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;

use super::{news_gql::NewsCreateInput, news_gql_model::NewsGqlModel, news_repository};

pub async fn create_one(
    data: NewsCreateInput,
    conn: &DatabaseConnection,
) -> FieldResult<NewsGqlModel> {
    news_repository::create_one(data, conn).await
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
    }
}
