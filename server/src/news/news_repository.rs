use crate::{entity::news, errors::gql_error::GqlError};
use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Set};

use super::{news_gql::NewsCreateInput, news_gql_model::NewsGqlModel};

pub async fn create_one<C>(input_data: NewsCreateInput, conn: &C) -> FieldResult<NewsGqlModel>
where
    C: ConnectionTrait,
{
    let insert_data = crate::entity::news::ActiveModel {
        news_id: Set(uuid::Uuid::new_v4()),
        title: Set(input_data.title),
        payload: Set(input_data.payload),
        is_publish: Set(input_data.is_publish),
        publish_at: Set(input_data.publish_at),
        ..Default::default()
    };
    let result: news::Model = news::Entity::insert(insert_data)
        .exec_with_returning(conn)
        .await?;
    Ok(NewsGqlModel::new(result))
}

pub async fn find_by_pk<C>(id: uuid::Uuid, conn: &C) -> FieldResult<NewsGqlModel>
where
    C: ConnectionTrait,
{
    match news::Entity::find_by_id(id).one(conn).await {
        Ok(model) => match model {
            Some(model) => Ok(NewsGqlModel::new(model)),
            None => Err(GqlError::NotFound("news not found".to_string()).extend()),
        },
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}

pub async fn update_img_by_pk(
    id: uuid::Uuid,
    img_url: String,
    conn: &DatabaseConnection,
) -> FieldResult<NewsGqlModel> {
    match news::Entity::find_by_id(id).one(conn).await {
        Ok(model) => match model {
            Some(model) => {
                let mut updated: news::ActiveModel = model.into();
                updated.img = Set(Some(img_url));
                match updated.update(conn).await {
                    Ok(up_news) => Ok(NewsGqlModel::new(up_news)),
                    Err(_) => Err(GqlError::ServerError("update news error".to_string()).extend()),
                }
            }
            None => Err(GqlError::NotFound("news not found".to_string()).extend()),
        },
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}

#[cfg(test)]
mod tests {
    use crate::db_utils::TestDb;

    use super::*;

    #[tokio::test]
    async fn test_news_repository() {
        let docker = testcontainers::clients::Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;

        let create_data = NewsCreateInput {
            title: "Test News".to_string(),
            payload: "This is a test news".to_string(),
            is_publish: true,
            publish_at: chrono::Utc::now().naive_utc(),
        };
        let result = create_one(create_data, conn)
            .await
            .expect("create news error");
        assert_eq!(result.title, "Test News");
        assert_eq!(result.payload, "This is a test news");
        assert_eq!(result.is_publish, true);

        let result = find_by_pk(result.news_id, conn)
            .await
            .expect("find news error");
        assert_eq!(result.title, "Test News");
        assert_eq!(result.payload, "This is a test news");
        assert_eq!(result.is_publish, true);

        let result = update_img_by_pk(result.news_id, "test".to_string(), conn)
            .await
            .expect("update news error");
        assert_eq!(result.title, "Test News");
        assert_eq!(result.payload, "This is a test news");
        assert_eq!(result.is_publish, true);
        assert_eq!(result.img, Some("test".to_string()));
    }
}
