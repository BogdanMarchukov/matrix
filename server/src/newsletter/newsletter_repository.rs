use crate::entity::newsletter;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
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
