use crate::entity::calculator as calculator_entity;
use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use uuid::Uuid;

use crate::{
    calculator::{self, calculator_repository::CalculatorRepository},
    db_utils,
    errors::gql_error::GqlError,
    tariff_plan::tariff_plan_service,
};

use super::user_info_gql_model::UserInfoGqlModel;
use super::{
    user_calc_result_gql_model::UserCalcResultGqlModel, user_info_repository,
    user_tariff_plan_service,
};

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
    if let false = user_tariff_plan_service::check_user_tariff_plan_by_tariff_ids(
        offer.tariff_ids,
        &user_id,
        &transaction,
    )
    .await?
    {
        return Err(GqlError::BadRequest("Tariff plan not found".into()).extend());
    }

    Ok(())
}

async fn check_require_fields<C>(
    calculator: &calculator_entity::Model,
    user_id: &Uuid,
    conn: &C,
) -> FieldResult<(UserInfoGqlModel, bool)>
where
    C: ConnectionTrait,
{
    let mut checked = false;
    let user_info = user_info_repository::find_one_by_user_id(user_id, conn).await?;
    let require_params = match calculator.require_params.as_ref() {
        Some(params) => params,
        None => return Ok((user_info, checked)),
    };
    for p in require_params.iter() {
        if p.as_str() == "date_of_birth" && user_info.date_of_birth.is_some() {
            return Ok((user_info, true));
        }
        if p.as_str() == "hour_of_birth" && user_info.hour_of_birth.is_some() {
            return Ok((user_info, true));
        }
        if p.as_str() == "min_of_birth" && user_info.min_of_birth.is_some() {
            return Ok((user_info, true));
        }
        if p.as_str() == "city" && user_info.city.is_some() {
            return Ok((user_info, true));
        }
    }
    Ok((user_info, false))
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
