use super::{
    notify_gql::NotifyByUserIdFilter,
    notify_gql_model::NotifyGqlModel,
    notify_repository::{self, find_many},
};
use crate::{
    entity::{notify, sea_orm_active_enums::NotifyTypeEnum},
    user::user_gql_model::UserGqlModel,
};
use async_graphql::FieldResult;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};
use uuid::Uuid;

pub async fn find_many_by_user_id(
    user: UserGqlModel,
    input_data: NotifyByUserIdFilter,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<NotifyGqlModel>> {
    let mut filter = Condition::all().add(notify::Column::UserId.eq(input_data.user_id));
    if let Some(is_read) = input_data.is_read {
        filter = filter.add(notify::Column::IsRead.eq(is_read));
    }
    if let Some(notify_type) = input_data.notify_type {
        filter = filter.add(notify::Column::NotifyType.eq(NotifyTypeEnum::from(notify_type)));
    }
    let all_notify = find_many(filter, conn).await?;
    for n in all_notify.iter() {
        n.check_role(&user)?;
    }
    Ok(all_notify)
}

pub async fn find_by_pk(
    notify_id: Uuid,
    user: UserGqlModel,
    conn: &DatabaseConnection,
) -> FieldResult<NotifyGqlModel> {
    let notify = notify_repository::find_by_pk(notify_id, conn).await?;
    notify.check_role(&user);
    Ok(notify)
}
