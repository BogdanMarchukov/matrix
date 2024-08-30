use crate::entity::newsletter;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use super::newsletter_gql::NewsLetterCreateInput;
use super::newsletter_gql_model::NewsletterGqlModel;

pub async fn create_one(
    input_data: NewsLetterCreateInput,
    conn: &DatabaseConnection,
) -> Result<NewsletterGqlModel, DbErr> {
    let new_newsletter = newsletter::ActiveModel {
        newsletter_id: Set(Uuid::new_v4()),
        title: Set(input_data.title),
        payload: Set(input_data.payload),
        publish_at: Set(input_data.publish_at),
        ..Default::default()
    };
    let result: newsletter::Model = newsletter::Entity::insert(new_newsletter)
        .exec_with_returning(conn)
        .await?;
    Ok(NewsletterGqlModel::new(result))
}

pub async fn find_all_active(conn: &DatabaseConnection) -> Result<Vec<NewsletterGqlModel>, DbErr> {
    let current_date = Utc::now().date_naive();
    let result = newsletter::Entity::find()
        .filter(newsletter::Column::PublishAt.lte(current_date))
        .all(conn)
        .await?;
    Ok(result
        .into_iter()
        .map(NewsletterGqlModel::new)
        .rev()
        .collect())
}
