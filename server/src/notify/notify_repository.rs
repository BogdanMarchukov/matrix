use super::notify_gql_model::NotifyGqlModel;
use crate::db_utils::get_transaction;
use crate::entity::sea_orm_active_enums::NotifyTypeEnum;
use crate::notify::notify_sender::SubscribeSender;
use crate::TxSender;
use crate::{
    entity::notify, entity::prelude::Notify, errors::gql_error::GqlError,
    newsletter::newsletter_gql_model::NewsletterGqlModel, user_repository,
};
use async_graphql::{ErrorExtensions, FieldResult};
use chrono::Local;
use migration::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter,
    QueryOrder, QuerySelect,
};
use uuid::Uuid;

pub struct NotifyUpdateData {
    pub is_read: bool,
}

pub async fn find_many(
    filter: Condition,
    conn: &DatabaseConnection,
    limit: Option<u64>,
    order: Option<Order>,
    order_by: Option<notify::Column>,
) -> FieldResult<Vec<NotifyGqlModel>> {
    let notify_all: Vec<notify::Model> = match Notify::find()
        .order_by(
            order_by.unwrap_or(notify::Column::CreatedAt),
            order.unwrap_or(Order::Desc),
        )
        .filter(filter)
        .limit(limit.unwrap_or(20))
        .all(conn)
        .await
    {
        Ok(value) => value,
        Err(_) => return Err(GqlError::ServerError("database error".to_string()).into()),
    };
    let result = notify_all
        .iter()
        .map(|n| NotifyGqlModel::new(n.to_owned()))
        .collect::<Vec<NotifyGqlModel>>();
    Ok(result)
}

pub async fn find_by_pk(notify_id: Uuid, conn: &DatabaseConnection) -> FieldResult<NotifyGqlModel> {
    if let Ok(Some(notify)) = Notify::find_by_id(notify_id).one(conn).await {
        Ok(NotifyGqlModel::new(notify))
    } else {
        Err(GqlError::NotFound("notify not found".to_string()).extend())
    }
}

pub async fn update_one(
    notify_id: Uuid,
    data: NotifyUpdateData,
    conn: &DatabaseConnection,
) -> FieldResult<NotifyGqlModel> {
    if let Ok(Some(notify)) = Notify::find_by_id(notify_id).one(conn).await {
        let mut notify: notify::ActiveModel = notify.into();
        notify.is_read = Set(data.is_read);
        if let Ok(notify) = notify.update(conn).await {
            Ok(NotifyGqlModel::new(notify))
        } else {
            Err(GqlError::ServerError("Database error".to_string()).extend())
        }
    } else {
        Err(GqlError::NotFound("Notify not found".to_string()).extend())
    }
}

pub async fn create_for_all_users(
    newsletter: &NewsletterGqlModel,
    conn: &DatabaseConnection,
) -> bool {
    if let Ok(txn) = get_transaction(None).await {
        match user_repository::find_all(conn, None).await {
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
                    .col_expr(notify::Column::IsRead, Expr::value(true))
                    .filter(notify::Column::IsRead.eq(false))
                    .filter(notify::Column::NotifyType.eq(NotifyTypeEnum::Daly))
                    .exec(&txn)
                    .await
                {
                    println!("update all notify success: {:?}", update_result);
                    match Notify::insert_many(insert_data.to_owned()).exec(&txn).await {
                        Ok(_) => {
                            txn.commit().await.ok();
                            for notify in insert_data.iter() {
                                SubscribeSender::send(SubscribeSender::NofifyDely(TxSender {
                                    user_id: notify.user_id.to_owned().unwrap(),
                                    id: notify.notify_id.to_owned().unwrap(),
                                }));
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
    } else {
        false
    }
}
