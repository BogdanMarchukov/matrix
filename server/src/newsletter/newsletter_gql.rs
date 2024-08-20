use async_graphql::{Context, FieldResult, Object};

pub struct NewsletterMutation;

#[Object]
impl NewsletterMutation {
    async fn create_one<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<bool> {
        Ok(true)
    }
}
