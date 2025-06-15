use super::newsletter_gql::NewsLetterCreateInput;
use super::newsletter_gql_model::NewsletterGqlModel;
use super::types::NewsletterUpdateInput;
use crate::entity::newsletter;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

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

pub async fn find_by_id(
    newsletter_id: &Uuid,
    conn: &DatabaseConnection,
) -> Option<newsletter::Model> {
    let result = newsletter::Entity::find_by_id(newsletter_id.to_owned())
        .one(conn)
        .await;
    match result {
        Ok(v) => v,
        Err(e) => {
            println!("find_by_id, database error {}", e);
            None
        }
    }
}

pub async fn update_one(
    newsletter_id: &Uuid,
    data: NewsletterUpdateInput,
    conn: &DatabaseConnection,
) -> bool {
    let result = find_by_id(newsletter_id, conn).await;
    if let Some(n) = result {
        let mut newsletter: newsletter::ActiveModel = n.into();
        if let Some(is_published) = data.is_published {
            newsletter.is_published = Set(is_published);
        }
        if let Some(title) = data.title {
            newsletter.title = Set(title);
        }
        if let Some(publish_at) = data.publish_at {
            newsletter.publish_at = Set(publish_at);
        }
        if let Some(is_published) = data.is_published {
            newsletter.is_published = Set(is_published);
        }
        newsletter.update(conn).await.is_ok()
    } else {
        false
    }
}

pub async fn find_all_active(conn: &DatabaseConnection) -> Result<Vec<NewsletterGqlModel>, DbErr> {
    let current_date = Local::now();
    let result = newsletter::Entity::find()
        .filter(newsletter::Column::PublishAt.lte(current_date))
        .filter(newsletter::Column::IsPublished.eq(false))
        .all(conn)
        .await?;
    Ok(result
        .into_iter()
        .map(NewsletterGqlModel::new)
        .rev()
        .collect())
}
