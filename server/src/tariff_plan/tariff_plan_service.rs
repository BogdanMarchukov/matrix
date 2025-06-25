use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

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
