use async_graphql::{Context, FieldResult, InputObject, Object};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::db_utils;
use crate::guards::system_guard::SystemGuard;
use crate::news::news_gql_model::NewsGqlModel;

use super::news_service;

pub struct NewsMutation;

#[derive(InputObject, Clone)]
pub struct NewsCreateInput {
    pub title: String,
    pub payload: String,
    pub is_publish: bool,
    pub publish_at: NaiveDateTime,
}

#[derive(InputObject, Clone)]
pub struct NewsUpdateInput {
    pub title: Option<String>,
    pub payload: Option<String>,
    pub is_publish: Option<bool>,
}

#[Object]
impl NewsMutation {
    #[graphql(guard = "SystemGuard")]
    async fn create_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: NewsCreateInput,
    ) -> FieldResult<NewsGqlModel> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        news_service::create_one(data, &conn).await
    }

    #[graphql(guard = "SystemGuard")]
    async fn update_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        news_id: Uuid,
        data: NewsUpdateInput,
    ) -> FieldResult<NewsGqlModel> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        news_service::update_one(news_id, data, &conn).await
    }
}
