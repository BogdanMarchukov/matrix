use async_graphql::{ErrorExtensions, FieldResult};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    entity::{prelude::UserTariffPlan, user_tariff_plan},
    errors::gql_error::GqlError,
    tariff_plan::tariff_plan_repository,
};

use super::user_tariff_plan_gql_model::UserTariffPlanGqlModel;

pub async fn find_by_user_id<C>(
    user_id: &Uuid,
    conn: &C,
) -> FieldResult<Vec<UserTariffPlanGqlModel>>
where
    C: ConnectionTrait,
{
    if let Ok(result) = UserTariffPlan::find()
        .filter(user_tariff_plan::Column::UserId.eq(user_id.to_owned()))
        .filter(user_tariff_plan::Column::ExpiresAt.gte(chrono::Utc::now()))
        .all(conn)
        .await
    {
        Ok(result
            .into_iter()
            .map(|u| UserTariffPlanGqlModel::new(u))
            .collect())
    } else {
        Err(GqlError::ServerError(("Database error".to_string())).extend())
    }
}

pub async fn crete<C>(
    tariff_plan_id: Uuid,
    tariff_plan_payment_id: Uuid,
    user_id: Uuid,
    conn: &C,
) -> FieldResult<UserTariffPlanGqlModel>
where
    C: ConnectionTrait,
{
    let tariff_plan = tariff_plan_repository::find_by_id(conn, tariff_plan_id)
        .await?
        .unwrap_or(return Err(GqlError::NotFound("tariff plan not found".to_string()).extend()));
    let exp = Utc::now() + Duration::days(tariff_plan.expiry_days.into());

    let user_tariff_plan = user_tariff_plan::ActiveModel {
        user_tariff_plan_id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        tariff_plan_id: Set(tariff_plan_id),
        tariff_plan_payment_id: Set(Some(tariff_plan_payment_id)),
        expires_at: Set(NaiveDateTime::new(exp.date_naive(), exp.time())),
        ..Default::default()
    };
    if let Ok(result) = UserTariffPlan::insert(user_tariff_plan)
        .exec_with_returning(conn)
        .await
    {
        Ok(UserTariffPlanGqlModel::new(result))
    } else {
        Err(GqlError::ServerError(("Database error".to_string())).extend())
    }
}

pub async fn find_or_create_free<C>(
    user_id: Uuid,
    conn: &C,
) -> FieldResult<Vec<UserTariffPlanGqlModel>>
where
    C: ConnectionTrait,
{
    let user_tariff_plans = find_by_user_id(&user_id, conn).await?;
    if user_tariff_plans.len() == 0 {
        let free_tariff_plan = tariff_plan_repository::find_free_tariff_plan(conn).await?;
        let exp = Utc::now() + Duration::days(free_tariff_plan.expiry_days.into());
        let new_user_tariff_plan = user_tariff_plan::ActiveModel {
            tariff_plan_id: Set(free_tariff_plan.tariff_plan_id),
            user_tariff_plan_id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            expires_at: Set(NaiveDateTime::new(exp.date_naive(), exp.time())),
            ..Default::default()
        };
        let result: user_tariff_plan::Model = UserTariffPlan::insert(new_user_tariff_plan)
            .exec_with_returning(conn)
            .await?;
        Ok(vec![UserTariffPlanGqlModel::new(result)])
    } else {
        Ok(user_tariff_plans)
    }
}
