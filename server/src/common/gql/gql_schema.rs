use async_graphql::{FieldResult, Object};

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self) -> FieldResult<i32> {
        Ok(32)
    }
}
