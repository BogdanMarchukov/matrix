use async_graphql::{ErrorExtensions, FieldResult};
use chrono::{Duration, NaiveDateTime, Utc};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
    entity::{prelude::UserTariffPlan, user_tariff_plan},
    errors::gql_error::GqlError,
    tariff_plan::tariff_plan_repository,
};

use super::user_tariff_plan_gql_model::UserTariffPlanGqlModel;

pub async fn find_by_user_id(
    user_id: Uuid,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<UserTariffPlanGqlModel>> {
    if let Ok(result) = UserTariffPlan::find()
        .filter(user_tariff_plan::Column::UserId.eq(user_id))
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

pub async fn find_or_create_free(
    user_id: Uuid,
    conn: &DatabaseConnection,
    trn: &DatabaseTransaction,
) -> FieldResult<Vec<UserTariffPlanGqlModel>> {
    let user_tariff_plans = find_by_user_id(user_id, conn).await?;
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
            .exec_with_returning(trn)
            .await?;
        Ok(vec![UserTariffPlanGqlModel::new(result)])
    } else {
        Ok(user_tariff_plans)
    }
}
