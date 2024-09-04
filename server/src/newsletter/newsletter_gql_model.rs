use super::newsletter_repository;
use crate::{
    entity::newsletter, newsletter::types::NewsletterUpdateInput, notify::notify_repository,
};
use async_graphql::*;
use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[derive(Clone, SimpleObject, Debug)]
#[graphql(name = "Newsletter")]
pub struct NewsletterGqlModel {
    pub newsletter_id: Uuid,
    pub title: String,
    pub is_published: bool,
    pub payload: String,
    pub created_at: DateTime<Utc>,
    pub publish_at: DateTime<Utc>,
}

impl NewsletterGqlModel {
    pub fn new(newsletter: newsletter::Model) -> Self {
        Self {
            newsletter_id: newsletter.newsletter_id,
            title: newsletter.title,
            is_published: newsletter.is_published,
            payload: newsletter.payload,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(newsletter.created_at, Utc),
            publish_at: DateTime::<Utc>::from_naive_utc_and_offset(newsletter.publish_at, Utc),
        }
    }

    pub async fn send_notify(&self, conn: &DatabaseConnection) -> bool {
        let result = notify_repository::create_for_all_users(&self, conn).await;
        if result {
            println!("publish newsletter is success Id: {}", &self.newsletter_id);
            let data = NewsletterUpdateInput {
                is_published: Some(true),
                ..Default::default()
            };
            let updated_result =
                newsletter_repository::update_one(&self.newsletter_id, data, conn).await;
            if updated_result {
                println!("updated published is success Id: {}", &self.newsletter_id)
            }
        } else {
            println!("error publish newsletter: Id: {}", &self.newsletter_id)
        };
        result
    }
}
