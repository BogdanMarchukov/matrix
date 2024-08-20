use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::entity::newsletter;

#[derive(Clone, SimpleObject)]
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
}
