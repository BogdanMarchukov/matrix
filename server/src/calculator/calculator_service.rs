use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::DatabaseConnection;

use crate::{errors::gql_error::GqlError, types::traits::Repository};

use super::{
    calculator_gql::FindOneFilterInput, calculator_gql_model::CalculatorGqlModel,
    calculator_repository::{CalculatorInsertData, CalculatorRepository},
};

pub async fn find_one(
    filter: FindOneFilterInput,
    db: &DatabaseConnection,
) -> FieldResult<Option<CalculatorGqlModel>> {
    match CalculatorRepository::find_one(filter.into(), db).await {
        Ok(model) => match model {
            Some(model) => Ok(Some(model.into())),
            None => Ok(None),
        },
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}

pub async fn create_one(data: CalculatorInsertData, db: &DatabaseConnection) -> FieldResult<CalculatorGqlModel> {
    match CalculatorRepository::create_one(data.into(), db).await {
        Ok(model) => Ok(model.into()),
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}
