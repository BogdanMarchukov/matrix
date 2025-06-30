use async_graphql::FieldResult;
use async_graphql::*;
use uuid::Uuid;

use crate::guards::auth_guard::AuthGuard;

use super::user_tariff_plan_gql_model::UserTariffPlanGqlModel;
use super::{user_service, user_tariff_plan_service};
pub struct UserTariffPlanMutation;

#[Object]
impl UserTariffPlanMutation {
    #[graphql(guard = "AuthGuard")]
    async fn buy_tariff_plan<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        tariff_plan_id: Uuid,
    ) -> FieldResult<UserTariffPlanGqlModel> {
        let (request_user, _) = user_service::get_auth_user_from_ctx(ctx)?;
        user_tariff_plan_service::buy_tariff_plan(tariff_plan_id, request_user.0.user_id).await
    }
}
