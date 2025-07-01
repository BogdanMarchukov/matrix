use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use uuid::Uuid;

use crate::errors::gql_error::GqlError;

use super::{
    tariff_plan_gql::TariffPlanCreateData, tariff_plan_gql_model::TariffPlanGqlModel,
    tariff_plan_repository,
};

pub async fn tariff_plan_create(
    data: TariffPlanCreateData,
    conn: &DatabaseConnection,
) -> FieldResult<TariffPlanGqlModel> {
    let result = tariff_plan_repository::create_one(conn, data).await?;
    TariffPlanGqlModel::new(result)
}

pub async fn find_by_id<C>(id: Uuid, conn: &C) -> FieldResult<TariffPlanGqlModel>
where
    C: ConnectionTrait,
{
    if let Some(result) = tariff_plan_repository::find_by_id(conn, id).await? {
        TariffPlanGqlModel::new(result)
    } else {
        Err(GqlError::NotFound("tariff plan not found".to_string()).extend())
    }
}

pub async fn find_by_ids(
    ids: &Vec<Uuid>,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<TariffPlanGqlModel>> {
    let result = tariff_plan_repository::find_many(
        conn,
        Some(tariff_plan_repository::FindManyFilter::new(Some(
            ids.clone(),
        ))),
    )
    .await?;
    match result
        .iter()
        .map(|x| match TariffPlanGqlModel::new(x.to_owned()) {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        })
        .collect()
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn find_many(conn: &DatabaseConnection) -> FieldResult<Vec<TariffPlanGqlModel>> {
    let result = tariff_plan_repository::find_many(conn, None).await?;
    match result
        .iter()
        .map(|x| match TariffPlanGqlModel::new(x.to_owned()) {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        })
        .collect()
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
