use async_graphql::FieldResult;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};
use crate::{entity::notify, user::user_gql_model::UserGqlModel};
use super::{
    notify_gql::NotifyByUserIdFilter, notify_gql_model::NotifyGqlModel,
    notify_repository::find_many,
};

pub async fn find_many_by_user_id(
    user: UserGqlModel,
    input_data: NotifyByUserIdFilter,
    conn: &DatabaseConnection,
) -> FieldResult<Vec<NotifyGqlModel>> {
    let all_notify = find_many(
        Condition::all().add(
            notify::Column::UserId
                .eq(input_data.user_id)
                .add(notify::Column::IsRead.eq(input_data.is_read)),
        ),
        conn,
    )
    .await?;
    for n in all_notify.iter() {
        n.check_role(&user)?;
    }
    Ok(all_notify)
}
