use crate::{
    entity::{notify, prelude::Notify},
    errors::gql_error::GqlError,
};
use async_graphql::FieldResult;
use sea_orm::{Condition, DatabaseConnection, EntityTrait, QueryFilter};
use super::notify_gql_model::NotifyGqlModel;

pub async fn find_many(
    filter:  Condition,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<NotifyGqlModel>> {
    let notify_all: Vec<notify::Model> = match Notify::find().filter(filter).all(conn).await {
        Ok(value) => value,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).into()),
    };
    let result = notify_all
        .iter()
        .map(|n| NotifyGqlModel::new(n.clone()))
        .collect::<Vec<NotifyGqlModel>>();
    Ok(result)
}
