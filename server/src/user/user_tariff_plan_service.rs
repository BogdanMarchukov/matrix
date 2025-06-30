use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{
    db_utils,
    payment::{payment_repository, tariff_plan_payment_repository},
    tariff_plan::{tariff_plan_repository, tariff_plan_service},
    GqlError,
};

use super::{user_tariff_plan_gql_model::UserTariffPlanGqlModel, user_tariff_plan_repository};

pub async fn buy_tariff_plan(
    tariff_plan_id: Uuid,
    user_id: Uuid,
) -> FieldResult<UserTariffPlanGqlModel> {
    let transaction = db_utils::get_transaction().await?;
    let tariff_plan = tariff_plan_service::find_by_id(tariff_plan_id, &transaction).await?;
    let result_check_free_tariff_plan = is_free_tariff_plan(tariff_plan_id, &transaction).await?;
    if result_check_free_tariff_plan == false {
        let payment =
            payment_repository::create_payment(tariff_plan.price, user_id, &transaction).await?;
        let tariff_plan_payment =
            tariff_plan_payment_repository::create(payment.payment_id, user_id, &transaction)
                .await?;
        let result = user_tariff_plan_repository::crete(
            tariff_plan_id,
            tariff_plan_payment.tariff_plan_payment_id,
            user_id,
            &transaction,
        )
        .await?;
        transaction.commit().await?;
        Ok(result)
    } else {
        Err(GqlError::BadRequest("tariff plan is free".to_string()).extend())
    }
}

async fn is_free_tariff_plan<C>(tariff_plan_id: Uuid, conn: &C) -> FieldResult<bool>
where
    C: ConnectionTrait,
{
    if let Ok(free_tariff_plan) = tariff_plan_repository::find_free_tariff_plan(conn).await {
        Ok(free_tariff_plan.tariff_plan_id == tariff_plan_id)
    } else {
        Err(GqlError::ServerError("database error".to_string()).extend())
    }
}
