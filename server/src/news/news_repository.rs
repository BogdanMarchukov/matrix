use crate::entity::news;
use async_graphql::FieldResult;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

use super::{news_gql::NewsCreateInput, news_gql_model::NewsGqlModel};

pub async fn create_one(
    input_data: NewsCreateInput,
    conn: &DatabaseConnection,
) -> FieldResult<NewsGqlModel> {
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
