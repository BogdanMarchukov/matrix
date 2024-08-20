use async_graphql::{Context, FieldResult, InputObject, Object};
use chrono::{DateTime, Utc};

pub struct NewsletterMutation;

#[derive(InputObject)]
pub struct NewsLetterCreateInput {
    pub title: String,
    pub is_published: bool,
    pub payload: String,
    pub publish_at: DateTime<Utc>,
}

#[Object]
impl NewsletterMutation {
    async fn create_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: NewsLetterCreateInput,
    ) -> FieldResult<bool> {
        Ok(true)
    }
}
