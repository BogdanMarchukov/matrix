use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, SimpleObject, Debug)]
#[graphql(name = "News")]
pub struct NewsGqlModel {
    pub news_id: Uuid,
    pub title: String,
    pub payload: String,
    pub is_publish: bool,
    pub img: Option<String>,
    pub publish_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewsGqlModel {
    pub fn new(news: crate::entity::news::Model) -> Self {
        Self {
            news_id: news.news_id,
            title: news.title,
            payload: news.payload,
            is_publish: news.is_publish,
            img: news.img,
            publish_at: DateTime::<Utc>::from_naive_utc_and_offset(news.publish_at, Utc),
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(news.created_at, Utc),
            updated_at: DateTime::<Utc>::from_naive_utc_and_offset(news.updated_at, Utc),
        }
    }
}
