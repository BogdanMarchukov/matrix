use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::DatabaseConnection;

use crate::{errors::gql_error::GqlError, types::traits::Repository};

use super::{
    calculator_gql::{CreateOneInput, FindOneFilterInput},
    calculator_gql_model::CalculatorGqlModel,
    calculator_repository::CalculatorRepository,
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

pub async fn create_one(
    data: CreateOneInput,
    db: &DatabaseConnection,
) -> FieldResult<CalculatorGqlModel> {
    match CalculatorRepository::create_one(data.into(), db).await {
        Ok(model) => Ok(model.into()),
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculator::calculator_gql::CreateOneInput;
    use crate::db_utils::TestDb;
    use crate::entity::sea_orm_active_enums::CalculatorType;
    use crate::offer::offer_repository;

    #[tokio::test]
    async fn test_calculator_service() {
        let docker = testcontainers::clients::Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;
        let default_calculator = find_one(
            FindOneFilterInput {
                r#type: CalculatorType::MatrixSchema.into(),
            },
            conn,
        )
        .await
        .expect("Failed to find default calculator");
        assert!(default_calculator.is_some());

        let result =
            CalculatorRepository::delete_one(default_calculator.unwrap().calculator_id, conn)
                .await
                .expect("Failed to delete all calculators");
        assert_eq!(result, true);
        let matrix_offer = offer_repository::find_many(conn, None, None, None)
            .await
            .expect("Failed to find offers");
        assert_eq!(matrix_offer.len(), 1);

        let create_data = CreateOneInput {
            offer_id: matrix_offer[0].offer_id,
            r#type: CalculatorType::MatrixSchema.into(),
            require_params: Some(vec!["param1".to_string(), "param2".to_string()]),
            options_params: Some(vec!["option1".to_string(), "option2".to_string()]),
        };

        let created = create_one(create_data.clone(), conn)
            .await
            .expect("Failed to create calculator");
        assert_eq!(created.r#type, create_data.r#type);
        assert_eq!(created.require_params, create_data.require_params);
        assert_eq!(created.options_params, create_data.options_params);

        let found = CalculatorRepository::find_by_pk(created.calculator_id, conn)
            .await
            .expect("Failed to find calculator")
            .unwrap();
        assert_eq!(created.calculator_id, found.calculator_id);
    }
}
