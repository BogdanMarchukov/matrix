use crate::db_utils;
use crate::guards::auth_guard::AuthGuard;
use crate::guards::system_guard::SystemGuard;
use crate::user::user_service;
use async_graphql::Object;
use async_graphql::*;

use super::tariff_plan_gql_model::TariffPlanGqlModel;
use super::tariff_plan_service;

pub struct TariffPlanMutation;
pub struct TariffPlanQuery;

#[derive(InputObject)]
pub struct TariffPlanCreateData {
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub expiry_days: i32,
}

#[Object]
impl TariffPlanQuery {
    #[graphql(guard = "AuthGuard")]
    async fn find_many<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<Vec<TariffPlanGqlModel>> {
        let (request_user, conn) = user_service::get_auth_user_from_ctx(ctx)?;
        tariff_plan_service::find_many(&conn).await
    }
}

#[Object]
impl TariffPlanMutation {
    #[graphql(guard = "SystemGuard")]
    async fn create_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: TariffPlanCreateData,
    ) -> FieldResult<TariffPlanGqlModel> {
        let conn = db_utils::get_connection_from_gql_ctx(ctx)?;
        tariff_plan_service::tariff_plan_create(data, &conn).await
    }
}
