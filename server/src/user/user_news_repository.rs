use migration::IntoCondition;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entity::user_news;

pub async fn find_many<C, F>(conn: &C, filter: Option<F>) -> Result<Vec<user_news::Model>, DbErr>
where
    C: ConnectionTrait,
    F: IntoCondition,
{
    let select = user_news::Entity::find();
    match filter {
        Some(filter) => select.filter(filter).all(conn).await,
        None => select.all(conn).await,
    }
}

pub async fn find_by_pk<C>(conn: &C, id: Uuid) -> Result<Option<user_news::Model>, DbErr>
where
    C: ConnectionTrait,
{
    user_news::Entity::find_by_id(id).one(conn).await
}
