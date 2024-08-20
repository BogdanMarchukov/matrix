use super::{newsletter_gql_model::NewsletterGqlModel, newsletter_repository};
use crate::guards::system_guard::SystemGuard;
use crate::{errors::gql_error::GqlError, GqlCtx};
use async_graphql::{Context, ErrorExtensions, FieldResult, InputObject, Object};
use chrono::NaiveDateTime;

pub struct NewsletterMutation;

#[derive(InputObject)]
pub struct NewsLetterCreateInput {
    pub title: String,
    pub payload: String,
    pub publish_at: NaiveDateTime,
}

#[Object]
impl NewsletterMutation {
    #[graphql(guard = "SystemGuard")]
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
