use crate::{
    db_utils::get_transaction,
    entity::{news, user_news, users},
    errors::gql_error::GqlError,
    notify::notify_sender::SubscribeSender,
    user::user_repository,
    TxSender,
};
use async_graphql::{ErrorExtensions, FieldResult};
use migration::{Expr, IntoCondition};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::{ColumnTrait, RelationTrait};

use super::{news_gql::NewsCreateInput, news_gql_model::NewsGqlModel};

pub async fn create_one<C>(input_data: NewsCreateInput, conn: &C) -> FieldResult<NewsGqlModel>
where
    C: ConnectionTrait,
{
    let insert_data = crate::entity::news::ActiveModel {
        news_id: Set(uuid::Uuid::new_v4()),
        title: Set(input_data.title),
        payload: Set(input_data.payload),
        is_publish: Set(input_data.is_publish),
        publish_at: Set(input_data.publish_at),
        ..Default::default()
    };
    let result: news::Model = news::Entity::insert(insert_data)
        .exec_with_returning(conn)
        .await?;
    Ok(NewsGqlModel::new(result))
}

pub async fn find_by_pk<C>(id: uuid::Uuid, conn: &C) -> FieldResult<NewsGqlModel>
where
    C: ConnectionTrait,
{
    match news::Entity::find_by_id(id).one(conn).await {
        Ok(model) => match model {
            Some(model) => Ok(NewsGqlModel::new(model)),
            None => Err(GqlError::NotFound("news not found".to_string()).extend()),
        },
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}

pub async fn find_all_active(conn: &DatabaseConnection) -> FieldResult<Vec<NewsGqlModel>> {
    let current_date = chrono::Local::now();
    let result = news::Entity::find()
        .filter(news::Column::PublishAt.lte(current_date))
        .filter(news::Column::IsPublish.eq(true))
        .all(conn)
        .await?;
    let mut news: Vec<NewsGqlModel> = vec![];
    for n in result {
        news.push(NewsGqlModel::new(n));
    }
    Ok(news)
}

pub async fn create_for_all_users(news: &NewsGqlModel, conn: &DatabaseConnection) -> bool {
    if let Ok(txn) = get_transaction(Some(conn.clone())).await {
        let news_id = news.news_id.to_owned();
        let relation = users::Relation::UserNews
            .def()
            .on_condition(move |_left, right| {
                Expr::col((right, user_news::Column::NewsId))
                    .ne(news_id)
                    .into_condition()
            });
        match user_repository::find_all(&txn, Some(relation)).await {
            Ok(users) => {
                let mut insert_data: Vec<user_news::ActiveModel> = vec![];
                for user in users {
                    let data = user_news::ActiveModel {
                        user_news_id: Set(uuid::Uuid::new_v4()),
                        user_id: Set(user.user_id.to_owned()),
                        news_id: Set(news.news_id.to_owned()),
                        title: Set(news.title.to_owned()),
                        payload: Set(news.payload.to_owned()),
                        img: Set(news.img.to_owned()),
                        reading_count: Set(0),
                        ..Default::default()
                    };
                    insert_data.push(data);
                }
                match user_news::Entity::insert_many(insert_data.to_owned())
                    .exec(&txn)
                    .await
                {
                    Ok(_) => {
                        txn.commit().await.ok();
                        for user_news in insert_data.iter() {
                            SubscribeSender::send(SubscribeSender::News(TxSender {
                                user_id: user_news.user_id.to_owned().unwrap(),
                                id: user_news.news_id.to_owned().unwrap(),
                            }));
                        }
                        true
                    }
                    Err(_) => {
                        txn.rollback().await.ok();
                        false
                    }
                }
            }
            Err(_) => {
                txn.rollback().await.ok();
                return false;
            }
        }
    } else {
        false
    }
}

pub async fn update_img_by_pk(
    id: uuid::Uuid,
    img_url: String,
    conn: &DatabaseConnection,
) -> FieldResult<NewsGqlModel> {
    match news::Entity::find_by_id(id).one(conn).await {
        Ok(model) => match model {
            Some(model) => {
                let mut updated: news::ActiveModel = model.into();
                updated.img = Set(Some(img_url));
                match updated.update(conn).await {
                    Ok(up_news) => Ok(NewsGqlModel::new(up_news)),
                    Err(_) => Err(GqlError::ServerError("update news error".to_string()).extend()),
                }
            }
            None => Err(GqlError::NotFound("news not found".to_string()).extend()),
        },
        Err(_) => Err(GqlError::ServerError("database error".to_string()).extend()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::web_app_data::UserTgWebApp, db_utils::TestDb};

    use super::*;

    #[tokio::test]
    async fn test_news_repository() {
        let docker = testcontainers::clients::Cli::default();
        let test_db = TestDb::new(&docker).await;
        let conn = &test_db.db;
        let tg_user = UserTgWebApp::test_data(Some(1));
        user_repository::create_one_by_tg(tg_user, conn)
            .await
            .expect("Create user error");
        let tg_second_user = UserTgWebApp::test_data(Some(2));
        user_repository::create_one_by_tg(tg_second_user, conn)
            .await
            .expect("Create user error");

        let create_data = NewsCreateInput {
            title: "Test News".to_string(),
            payload: "This is a test news".to_string(),
            is_publish: true,
            publish_at: chrono::Utc::now().naive_utc(),
        };
        let result = create_one(create_data, conn)
            .await
            .expect("create news error");
        assert_eq!(result.title, "Test News");
        assert_eq!(result.payload, "This is a test news");
        assert_eq!(result.is_publish, true);

        let is_publish = find_all_active(conn).await.expect("find news error");
        assert_eq!(is_publish.len(), 1);

        let result = find_by_pk(result.news_id, conn)
            .await
            .expect("find news error");
        assert_eq!(result.title, "Test News");
        assert_eq!(result.payload, "This is a test news");
        assert_eq!(result.is_publish, true);

        let result = update_img_by_pk(result.news_id, "test".to_string(), conn)
            .await
            .expect("update news error");
        assert_eq!(result.title, "Test News");
        assert_eq!(result.payload, "This is a test news");
        assert_eq!(result.is_publish, true);
        assert_eq!(result.img, Some("test".to_string()));

        let result = create_for_all_users(&result, conn).await;
        assert_eq!(result, true);
    }
}
