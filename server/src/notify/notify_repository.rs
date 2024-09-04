use super::notify_gql_model::NotifyGqlModel;
use crate::db_utils::get_transaction;
use crate::entity::sea_orm_active_enums::NotifyTypeEnum;
use crate::{
    entity::notify, entity::prelude::Notify, errors::gql_error::GqlError,
    newsletter::newsletter_gql_model::NewsletterGqlModel, user_repository,
};
use crate::{TxSender, TX_NOTIFY};
use async_graphql::FieldResult;
use chrono::Local;
use migration::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Value};
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
    let txn = get_transaction().await;
    let tx = TX_NOTIFY.clone();
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
            if let Ok(update_result) = Notify::update_many()
                .col_expr(
                    notify::Column::IsRead,
                    Expr::value(true),
                )
                .filter(notify::Column::IsRead.eq(false))
                .filter(notify::Column::NotifyType.eq(NotifyTypeEnum::Daly))
                .exec(&txn)
                .await
            {
                println!("update all notify success: {:?}", update_result);
                match Notify::insert_many(insert_data.clone()).exec(&txn).await {
                    Ok(_) => {
                        txn.commit().await.ok();
                        for notify in insert_data.iter() {
                            match tx.send(TxSender {
                                user_id: notify.user_id.to_owned().unwrap(),
                                id: notify.notify_id.to_owned().unwrap(),
                            }) {
                                Ok(_) => {}
                                Err(err) => {
                                    println!("TX_NOTIFY send error: {}", err);
                                    return true;
                                }
                            }
                        }
                        true
                    }
                    Err(e) => {
                        txn.rollback().await.ok();
                        println!("insert many error: {}", e);
                        false
                    }
                }
            } else {
                txn.rollback().await.ok();
                false
            }
        }
        Err(e) => {
            println!("ger users error: {}", e);
            false
        }
    }
}
