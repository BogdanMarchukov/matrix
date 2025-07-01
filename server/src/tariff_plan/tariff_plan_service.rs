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

#[cfg(test)]
mod tests {

    use crate::db_utils::TestDb;

    use super::*;

    #[tokio::test]
    async fn test_tariff_plan_create_and_find() {
        let docker = testcontainers::clients::Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;

        let create_data = TariffPlanCreateData {
            title: "Test Plan".to_string(),
            price: 100_f64,
            expiry_days: 30,
            description: Some("Just a test plan".to_string()),
        };

        let created = tariff_plan_create(create_data.clone(), conn)
            .await
            .expect("Failed to create tariff");

        assert_eq!(created.title, create_data.title);
        assert_eq!(created.price, create_data.price);

        let fetched = find_by_id(created.tariff_plan_id, conn)
            .await
            .expect("Tariff not found by ID");

        assert_eq!(fetched.tariff_plan_id, created.tariff_plan_id);
        assert_eq!(fetched.title, "Test Plan");

        let many = find_by_ids(&vec![created.tariff_plan_id], conn)
            .await
            .expect("find_by_ids failed");

        assert_eq!(many.len(), 1);
        assert_eq!(many[0].tariff_plan_id, created.tariff_plan_id);

        let all = find_many(conn).await.expect("find_many failed");

        assert!(all
            .iter()
            .any(|t| t.tariff_plan_id == created.tariff_plan_id));
    }
}
