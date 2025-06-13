use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{ConnectionTrait, EntityTrait, Order, QueryOrder, QuerySelect, Set};
use uuid::Uuid;

use crate::{
    entity::{offer, prelude::Offer},
    errors::gql_error::GqlError,
};

use super::{offer_gql::OfferCreateData, offer_gql_model::OfferGqlModel};

pub async fn find_many<C>(
    conn: &C,
    order: Option<Order>,
    order_by: Option<offer::Column>,
    limit: Option<u64>,
) -> FieldResult<Vec<OfferGqlModel>>
where
    C: ConnectionTrait,
{
    let result = Offer::find()
        .order_by(
            order_by.unwrap_or(offer::Column::CreatedAt),
            order.unwrap_or(Order::Asc),
        )
        .limit(limit.unwrap_or(20))
        .all(conn)
        .await;
    if let Ok(offers) = result {
        Ok(offers
            .iter()
            .map(|o| OfferGqlModel::new(o.to_owned()))
            .collect())
    } else {
        Err(GqlError::ServerError("offer: find many error".to_string()).extend())
    }
}

pub async fn create_one<C>(data: OfferCreateData, conn: &C) -> FieldResult<OfferGqlModel>
where
    C: ConnectionTrait,
{
    let new_offer = offer::ActiveModel {
        offer_id: Set(Uuid::new_v4()),
        is_active: Set(data.is_active),
        title: Set(data.title),
        tariff_ids: Set(data.tariff_ids),
        ..Default::default()
    };
    if let Ok(result) = offer::Entity::insert(new_offer)
        .exec_with_returning(conn)
        .await
    {
        Ok(OfferGqlModel::new(result))
    } else {
        Err(GqlError::ServerError("offer create error: database error".to_string()).extend())
    }
}
