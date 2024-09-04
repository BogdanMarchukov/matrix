use super::notify_gql_model::NotifyGqlModel;
use crate::{
    entity::notify, entity::prelude::Notify, errors::gql_error::GqlError,
    newsletter::newsletter_gql_model::NewsletterGqlModel, user_repository,
};
use crate::{TxSender, TxType, TX_NOTIFY};
use async_graphql::FieldResult;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{Condition, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn find_many(
    filter: Condition,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<NotifyGqlModel>> {
    let notify_all: Vec<notify::Model> = match Notify::find().filter(filter).all(conn).await {
        Ok(value) => value,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).into()),
    };
    let result = notify_all
        .iter()
        .map(|n| NotifyGqlModel::new(n.to_owned()))
        .collect::<Vec<NotifyGqlModel>>();
    Ok(result)
}

pub async fn create_for_all_users(
    newsletter: &NewsletterGqlModel,
    conn: &DatabaseConnection,
) -> bool {
    match user_repository::find_all(conn).await {
        Ok(v) => {
            let mut insert_data: Vec<notify::ActiveModel> = vec![];
            for user in v.iter() {
                let data = notify::ActiveModel {
                    notify_id: Set(Uuid::new_v4()),
                    title: Set(newsletter.title.to_owned()),
                    payload: Set(newsletter.payload.to_owned()),
                    is_read: Set(false),
                    user_id: Set(user.user_id.to_owned()),
                    created_at: Set(Local::now().naive_local()),
                    ..Default::default()
                };
                insert_data.push(data);
            }
            match Notify::insert_many(insert_data.clone()).exec(conn).await {
                Ok(_) => {
                    for notify in insert_data.iter() {
                        let tx = TX_NOTIFY.clone();
                        match tx.send(TxSender {
                            user_id: notify.user_id.to_owned().unwrap(),
                            id: notify.notify_id.to_owned().unwrap(),
                        }) {
                            Ok(_) => {}
                            Err(err) => println!("TX_NOTIFY send error: {}", err),
                        }
                    }
                    true
                }
                Err(e) => {
                    println!("insert many error: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("ger users error: {}", e);
            false
        }
    }
}
