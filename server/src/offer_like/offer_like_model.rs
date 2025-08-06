use sea_orm::entity::prelude::*;
use async_graphql::{SimpleObject, ID};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "offer_like")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub offer_like_id: Uuid,
    pub offer_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(SimpleObject)]
pub struct OfferLikeGqlModel {
    pub offer_like_id: ID,
    pub offer_id: ID,
    pub user_id: ID,
}

impl From<Model> for OfferLikeGqlModel {
    fn from(model: Model) -> Self {
        OfferLikeGqlModel {
            offer_like_id: ID::from(model.offer_like_id.to_string()),
            offer_id: ID::from(model.offer_id.to_string()),
            user_id: ID::from(model.user_id.to_string()),
        }
    }
}
