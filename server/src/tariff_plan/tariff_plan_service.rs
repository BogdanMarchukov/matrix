use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;

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
