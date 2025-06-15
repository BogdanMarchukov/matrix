use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{
    entity::{prelude::TariffPlan, tariff_plan},
    errors::gql_error::GqlError,
};

pub async fn find_free_tariff_plan<C>(conn: &C) -> FieldResult<tariff_plan::Model>
where
    C: ConnectionTrait,
{
    TariffPlan::find()
        .filter(tariff_plan::Column::Title.eq("free"))
        .one(conn)
        .await?
        .ok_or_else(|| GqlError::NotFound("free tariff plan not found".to_string()).extend())
}
