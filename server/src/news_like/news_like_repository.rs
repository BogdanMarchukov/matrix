use sea_orm::{prelude::*, DeleteResult, Set};
use sea_orm::{EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entity::news_like;

pub struct NewsLikeFilter {
    pub news_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

pub struct NewsLikeRepository;

impl NewsLikeRepository {
    pub async fn create_one<C>(
        db: &C,
        news_id: Uuid,
        user_id: Uuid,
    ) -> Result<news_like::Model, DbErr>
    where
        C: ConnectionTrait,
    {
        let new_news_like = news_like::ActiveModel {
            news_like_id: Set(Uuid::new_v4()),
            news_id: Set(news_id),
            user_id: Set(user_id),
            ..Default::default()
        };
        news_like::Entity::insert(new_news_like)
            .exec_with_returning(db)
            .await
    }

    pub async fn find_many<C>(
        db: &C,
        filter: NewsLikeFilter,
    ) -> Result<Vec<news_like::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut query = news_like::Entity::find();

        if let Some(news_id) = filter.news_id {
            query = query.filter(news_like::Column::NewsId.eq(news_id));
        }

        if let Some(user_id) = filter.user_id {
            query = query.filter(news_like::Column::UserId.eq(user_id));
        }

        query.all(db).await
    }

    pub async fn find_one<C>(
        db: &C,
        filter: NewsLikeFilter,
    ) -> Result<Option<news_like::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut query = news_like::Entity::find();

        if let Some(news_id) = filter.news_id {
            query = query.filter(news_like::Column::NewsId.eq(news_id));
        }

        if let Some(user_id) = filter.user_id {
            query = query.filter(news_like::Column::UserId.eq(user_id));
        }

        query.one(db).await
    }

    pub async fn delete_one<C>(
        db: &C,
        news_like_id: Uuid,
    ) -> Result<DeleteResult, DbErr>
    where
        C: ConnectionTrait,
    {
        news_like::Entity::delete_by_id(news_like_id).exec(db).await
    }
}
