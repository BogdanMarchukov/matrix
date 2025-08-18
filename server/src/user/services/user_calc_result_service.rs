use std::result;
use std::sync::Arc;

use crate::compute::matrix_schema_client::{self, MatrixSchemaClient};
use crate::compute::MatrixSchemaRequest;
use crate::entity::calculator as calculator_entity;
use crate::entity::sea_orm_active_enums::CalculatorType;
use crate::types::structurs::MatrixSchemaSvcWrapper;
use crate::types::traits::{MatrixSchemaSvc, Repository};
use async_graphql::{ErrorExtensions, FieldResult};
use chrono::{Datelike, NaiveDate};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use tonic::transport::Channel;
use tonic::Request;
use uuid::Uuid;

use crate::{
    calculator::calculator_repository::CalculatorRepository, db_utils, errors::gql_error::GqlError,
};

use super::user_calc_result_gql_model::UserCalcResultGqlModel;
use super::user_calc_result_repository::{UserCalcResultInsertData, UserCalcResultRepository};
use super::user_info_gql_model::UserInfoGqlModel;
use super::{user_info_repository, user_tariff_plan_service};

pub async fn create_calc(
    user_id: Uuid,
    calculator_id: Uuid,
    db: &DatabaseConnection,
    matrix_schema_client: Arc<MatrixSchemaClient<Channel>>,
) -> FieldResult<UserCalcResultGqlModel> {
    let transaction = match db_utils::get_transaction(Some(db.clone())).await {
        Ok(transaction) => transaction,
        Err(_) => return Err(GqlError::ServerError("Database error".into()).extend()),
    };
    let (calculator, offer) = get_calculator_and_offer(calculator_id, &transaction).await?;
    if let false = user_tariff_plan_service::check_user_tariff_plan_by_tariff_ids(
        offer.tariff_ids,
        &user_id,
        &transaction,
    )
    .await?
    {
        return Err(GqlError::BadRequest("Tariff plan not found".into()).extend());
    }
    let user_info = check_require_fields(&calculator, &user_id, &transaction).await?;
    let matrix_calc = MatrixSchemaSvcWrapper::new(matrix_schema_client);
    let (calc_result, key) = match calculator.r#type {
        CalculatorType::MatrixSchema => {
            let date_of_birth = user_info
                .date_of_birth
                .ok_or_else(|| GqlError::BadRequest("Date of birth is required".into()).extend())?;

            create_matrix_calc(date_of_birth, Arc::new(matrix_calc)).await?
        }
    };
    let insert_data = UserCalcResultInsertData {
        user_id,
        calculator_id,
        key,
        result: serde_json::json!(calc_result),
        ..Default::default()
    };
    let user_matrix_calc_result = UserCalcResultRepository::create_one(insert_data, db)
        .await
        .map_err(|_| GqlError::ServerError("Database error".into()).extend())?;

    Ok(user_matrix_calc_result.into())
}

async fn create_matrix_calc<S: MatrixSchemaSvc>(
    date_of_birth: NaiveDate,
    matrix_schema_client: Arc<S>,
) -> FieldResult<(Vec<i32>, String)> {
    let year = date_of_birth.year();
    let month = date_of_birth.month() as i32;
    let day = date_of_birth.day() as i32;

    let request = Request::new(MatrixSchemaRequest { year, month, day });

    let response = matrix_schema_client
        .as_ref()
        .clone()
        .calc_matrix_schema(request)
        .await?;
    let schema = response
        .into_inner()
        .schema
        .into_iter()
        .flat_map(|arr| arr.values)
        .collect();
    let key = date_of_birth.to_string();

    Ok((schema, key))
}

async fn check_require_fields<C>(
    calculator: &calculator_entity::Model,
    user_id: &Uuid,
    conn: &C,
) -> FieldResult<UserInfoGqlModel>
where
    C: ConnectionTrait,
{
    let user_info = user_info_repository::find_one_by_user_id(user_id, conn).await?;
    let require_params = match calculator.require_params.as_ref() {
        Some(params) => params,
        None => return Ok(user_info),
    };
    for p in require_params.iter() {
        match p.as_str() {
            "date_of_birth" => {
                if user_info.date_of_birth.is_none() {
                    return Err(GqlError::BadRequest("Date of birth is required".into()).extend());
                }
            }
            "hour_of_birth" => {
                if user_info.hour_of_birth.is_none() {
                    return Err(GqlError::BadRequest("Hour of birth is required".into()).extend());
                }
            }
            "min_of_birth" => {
                if user_info.min_of_birth.is_none() {
                    return Err(GqlError::BadRequest("Min of birth is required".into()).extend());
                }
            }
            "city" => {
                if user_info.city.is_none() {
                    return Err(GqlError::BadRequest("City is required".into()).extend());
                }
            }
            _ => {
                return Err(GqlError::ServerError("unknown params".into()).extend());
            }
        }
    }
    Ok(user_info)
}

async fn get_calculator_and_offer<C>(
    calculator_id: Uuid,
    db: &C,
) -> FieldResult<(
    crate::entity::calculator::Model,
    crate::entity::offer::Model,
)>
where
    C: ConnectionTrait,
{
    let (calculator, offer) =
        match CalculatorRepository::find_by_pk_with_offer(calculator_id, db).await {
            Ok(calculator) => match calculator {
                Some((calc, offer)) => match offer {
                    Some(offer) => (calc, offer),
                    None => return Err(GqlError::NotFound("offer not found".into()).extend()),
                },
                None => return Err(GqlError::NotFound("Calculator not found".into()).extend()),
            },
            Err(_) => return Err(GqlError::ServerError("Database error".into()).extend()),
        };
    Ok((calculator, offer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use tonic::Response;

    use crate::{
        compute::{Int32Array, MatrixSchemaResponse},
        types::traits::MockMatrixSchemaSvc,
    };

    #[tokio::test]
    async fn test_create_matrix_calc() {
        let mut mock = MockMatrixSchemaSvc::new();

        mock.expect_calc_matrix_schema().returning(|_req| {
            Box::pin(async {
                Ok(Response::new(MatrixSchemaResponse {
                    schema: vec![Int32Array {
                        values: vec![1, 2, 3],
                    }],
                }))
            })
        });

        let date_of_birth = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
        let (result, key) = create_matrix_calc(date_of_birth, Arc::new(mock))
           
                .await
                .unwrap();

        assert_eq!(key, date_of_birth.to_string());
    }
}
