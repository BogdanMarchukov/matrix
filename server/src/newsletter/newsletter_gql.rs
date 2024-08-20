use async_graphql::{Context, ErrorExtensions, FieldResult, InputObject, Object};
use chrono::NaiveDateTime;

use crate::{errors::gql_error::GqlError, GqlCtx};

use super::{newsletter_gql_model::NewsletterGqlModel, newsletter_repository};

pub struct NewsletterMutation;

#[derive(InputObject)]
pub struct NewsLetterCreateInput {
    pub title: String,
    pub is_published: bool,
    pub payload: String,
    pub publish_at: NaiveDateTime,
}

#[Object]
impl NewsletterMutation {
    async fn create_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: NewsLetterCreateInput,
    ) -> FieldResult<NewsletterGqlModel> {
        let ctx_data = match ctx.data::<GqlCtx>() {
            Ok(data) => data,
            Err(_) => return Err(GqlError::ServerError("get ctx data errors".to_string()).extend()),
        };
        let result = newsletter_repository::create_one(data, &ctx_data.db).await?;
        Ok(result)
    }
}
