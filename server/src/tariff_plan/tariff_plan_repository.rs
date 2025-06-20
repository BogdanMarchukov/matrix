use async_graphql::{ErrorExtensions, FieldResult};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    entity::{prelude::TariffPlan, tariff_plan},
    errors::gql_error::GqlError,
};

use super::tariff_plan_gql::TariffPlanCreateData;

pub async fn find_free_tariff_plan<C>(conn: &C) -> FieldResult<tariff_plan::Model>
where
    C: ConnectionTrait,
{
    TariffPlan::find()
        .filter(tariff_plan::Column::Title.eq("free"))
        .one(conn)
        .await?
        .ok_or_else(|| GqlError::NotFound("free tariff plan not found".to_string()).extend())
}

pub async fn create_one<C>(conn: &C, data: TariffPlanCreateData) -> FieldResult<tariff_plan::Model>
where
    C: ConnectionTrait,
{
    if let Some(price) = Decimal::from_f64_retain(data.price) {
        let data = tariff_plan::ActiveModel {
            tariff_plan_id: Set(Uuid::new_v4()),
            title: Set(data.title),
            description: Set(data.description),
            price: Set(price),
            expiry_days: Set(data.expiry_days),
        };
        match data.insert(conn).await {
            Ok(result) => Ok(result),
            Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
        }
    } else {
        Err(GqlError::ServerError("parse error".to_string()).extend())
    }
}
