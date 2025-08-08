use async_graphql::{ComplexObject, FieldResult, SimpleObject};
use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use tracing::info;
use uuid::Uuid;

use crate::{
    db_utils,
    news_like::{news_like_gql_model::NewsLikeGqlModel, news_like_service},
};

use super::news_repository;

#[derive(SimpleObject)]
pub struct ManyData {
    count: u64,
    value: Vec<NewsLikeGqlModel>,
}

#[derive(Clone, SimpleObject, Debug)]
#[graphql(complex)]
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

#[ComplexObject]
impl NewsGqlModel {
    async fn news_likes(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<ManyData> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        let value = news_like_service::find_by_news_id(&conn, self.news_id.to_owned()).await?;
        let count = news_like_service::find_count_by_news_id(&conn, self.news_id).await?;
        Ok(ManyData { value, count })
    }

    async fn count_news_like(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<u64> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        news_like_service::find_count_by_news_id(&conn, self.news_id).await
    }
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

    pub async fn send_notify(&self, conn: &DatabaseConnection) -> bool {
        let result = news_repository::create_for_all_users(self, conn).await;
        if result {
            info!("publish news is success Id: {}", &self.news_id);
        }
        result
    }
}
