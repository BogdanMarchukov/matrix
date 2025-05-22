use super::{
    notify_gql::{NotifyByUserIdFilter, NotifyOrderBy, NotifyUpdateData, Sort},
    notify_gql_model::NotifyGqlModel,
    notify_repository::{self, find_many},
};
use crate::{
    entity::{notify, sea_orm_active_enums::NotifyTypeEnum},
    gql_schema::GqlOrder,
    user::user_gql_model::UserGqlModel,
};
use async_graphql::FieldResult;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, Order};
use uuid::Uuid;

pub async fn find_many_by_user_id(
    user: UserGqlModel,
    input_data: NotifyByUserIdFilter,
    input_sort: Option<Sort>,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<NotifyGqlModel>> {
    let mut filter = Condition::all().add(notify::Column::UserId.eq(input_data.user_id));
    if let Some(is_read) = input_data.is_read {
        filter = filter.add(notify::Column::IsRead.eq(is_read));
    }
    if let Some(notify_type) = input_data.notify_type {
        filter = filter.add(notify::Column::NotifyType.eq(NotifyTypeEnum::from(notify_type)));
    }
    let order = match &input_sort {
        Some(sort) => match sort.order {
            Some(order) => match order {
                GqlOrder::Desc => Some(Order::Desc),
                GqlOrder::Asc => Some(Order::Asc),
            },
            None => None,
        },
        None => None,
    };
    let order_by = match &input_sort {
        Some(sort) => match sort.order_by {
            Some(order_by) => match order_by {
                NotifyOrderBy::CreatedAt => Some(notify::Column::CreatedAt),
            },
            None => None,
        },
        None => None,
    };
    let limit = match input_sort {
        Some(sort) => sort.limit,
        None => None
    };
    let all_notify = find_many(filter, conn, limit, order, order_by).await?;
    for n in all_notify.iter() {
        n.check_role(&user)?;
    }
    Ok(all_notify)
}

pub async fn update_one(
    notify_id: Uuid,
    data: NotifyUpdateData,
    user: UserGqlModel,
    conn: &DatabaseConnection,
) -> FieldResult<NotifyGqlModel> {
    find_by_pk(notify_id, user, conn).await?;
    notify_repository::update_one(
        notify_id,
        notify_repository::NotifyUpdateData {
            is_read: data.is_read,
        },
        conn,
    )
    .await
}

pub async fn find_by_pk(
    notify_id: Uuid,
    user: UserGqlModel,
    conn: &DatabaseConnection,
) -> FieldResult<NotifyGqlModel> {
    let notify = notify_repository::find_by_pk(notify_id, conn).await?;
    notify.check_role(&user)?;
    Ok(notify)
}
