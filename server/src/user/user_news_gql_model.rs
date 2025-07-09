use async_graphql::{ComplexObject, ErrorExtensions, FieldResult, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::gql_error::GqlError;

use super::user_gql_model::{User, UserRoleGqlType};
#[derive(SimpleObject, Clone, Debug)]
#[graphql(complex)]
#[graphql(name = "UserNews")]
pub struct UserNewsGqlModel {
    pub user_news_id: Uuid,
    pub news_id: Uuid,
    pub title: String,
    pub payload: String,
    #[graphql(skip)]
    pub img: Option<String>,
    pub created_at: DateTime<Utc>,
    pub reading_at: Option<DateTime<Utc>>,
    pub reading_count: i32,
    pub user_id: Uuid,
}

#[ComplexObject]
impl UserNewsGqlModel {
    async fn img(&self) -> Option<String> {
        let config = crate::config::S3Config::new();
        match &self.img {
            Some(img) => Some(format!("{}/{}", config.endpoint, img)),
            None => None,
        }
    }
}

impl UserNewsGqlModel {
    pub fn new(user_news: crate::entity::user_news::Model) -> Self {
        Self {
            user_news_id: user_news.user_news_id,
            news_id: user_news.news_id,
            title: user_news.title,
            payload: user_news.payload,
            img: user_news.img,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(user_news.created_at, Utc),
            reading_at: user_news
                .reading_at
                .map(|v| DateTime::<Utc>::from_naive_utc_and_offset(v, Utc)),
            reading_count: user_news.reading_count,
            user_id: user_news.user_id,
        }
    }

    pub fn check_role(&self, user: &User) -> FieldResult<&Self> {
        let allowed = match user.0.role {
            UserRoleGqlType::Owner => true,
            UserRoleGqlType::Admin => true,
            UserRoleGqlType::Member => self.user_id == user.0.user_id,
        };
        if allowed {
            return Ok(self);
        }
        Err(GqlError::Forbidden.extend())
    }
}
