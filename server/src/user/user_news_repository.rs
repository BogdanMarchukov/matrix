use chrono::NaiveDateTime;
use migration::IntoCondition;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entity::user_news;

pub struct UserNewsUpdateData {
    pub reading_count: Option<i32>,
    pub reading_at: Option<NaiveDateTime>,
}

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

pub async fn update_one<C>(
    id: Uuid,
    update_data: UserNewsUpdateData,
    conn: &C,
) -> Result<user_news::Model, DbErr>
where
    C: ConnectionTrait,
{
    let user_news = find_by_pk(conn, id)
        .await?
        .ok_or(DbErr::RecordNotFound("user news not found".to_string()))?;
    let mut user_news: user_news::ActiveModel = user_news.into();
    if let Some(reading_count) = update_data.reading_count {
        user_news.reading_count = Set(reading_count);
    }
    if let Some(reading_at) = update_data.reading_at {
        user_news.reading_at = Set(Some(reading_at));
    }
    user_news.update(conn).await
}
