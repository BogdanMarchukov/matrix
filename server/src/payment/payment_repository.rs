use async_graphql::{ErrorExtensions, FieldResult};
use rust_decimal::Decimal;
use sea_orm::{ConnectionTrait, EntityTrait, Set};
use uuid::Uuid;

use crate::{
    entity::{payment, prelude::Payment},
    errors::gql_error::GqlError,
};

pub async fn create_payment<C>(sum: f64, user_id: Uuid, conn: &C) -> FieldResult<payment::Model>
where
    C: ConnectionTrait,
{
    if let Some(sum) = Decimal::from_f64_retain(sum) {
        let data = payment::ActiveModel {
            payment_id: Set(Uuid::new_v4()),
            sum: Set(sum),
            user_id: Set(user_id),
            ..Default::default()
        };
        match Payment::insert(data).exec_with_returning(conn).await {
            Ok(result) => Ok(result),
            Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
        }
    } else {
        Err(GqlError::ServerError("parse error".to_string()).extend())
    }
}
