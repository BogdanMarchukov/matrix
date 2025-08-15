use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use uuid::Uuid;

use crate::{
    calculator::{self, calculator_repository::CalculatorRepository},
    db_utils,
    errors::gql_error::GqlError,
};

use super::user_calc_result_gql_model::UserCalcResultGqlModel;

pub async fn create_calc(
    user_id: Uuid,
    calculator_id: Uuid,
    db: DatabaseConnection,
) -> FieldResult<()> {
    let transaction = match db_utils::get_transaction(Some(db)).await {
        Ok(transaction) => transaction,
        Err(_) => return Err(GqlError::ServerError("Database error".into()).extend()),
    };
    let (calculator, offer) = get_calculator_and_offer(calculator_id, &transaction).await?;
    Ok(())
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
