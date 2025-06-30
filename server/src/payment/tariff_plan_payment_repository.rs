use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{ConnectionTrait, EntityTrait, Set};
use uuid::Uuid;

use crate::{
    entity::{prelude::TariffPlanPayment, tariff_plan_payment},
    errors::gql_error::GqlError,
};

pub async fn create<C>(
    payment_id: Uuid,
    user_id: Uuid,
    conn: &C,
) -> FieldResult<tariff_plan_payment::Model>
where
    C: ConnectionTrait,
{
    let data = tariff_plan_payment::ActiveModel {
        tariff_plan_payment_id: Set(Uuid::new_v4()),
        payment_id: Set(payment_id),
        user_id: Set(user_id),
        ..Default::default()
    };
    match TariffPlanPayment::insert(data)
        .exec_with_returning(conn)
        .await
    {
        Ok(result) => Ok(result),
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}
