use async_graphql::InputObject;
use chrono::NaiveDate;

#[derive(Default, InputObject)]
pub struct UserInfoUpdateInput {
    pub city: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub hour_of_birth: Option<i32>,
    pub min_of_birth: Option<i32>,
}
