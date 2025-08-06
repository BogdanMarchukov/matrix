
// сделай как в OfferGqlModel AI!

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "offer_like")]
pub struct OfferLikeGqlModel {
    #[sea_orm(primary_key, auto_increment = false)]
    pub offer_like_id: Uuid,
    pub offer_id: Uuid,
    pub user_id: Uuid,
}
