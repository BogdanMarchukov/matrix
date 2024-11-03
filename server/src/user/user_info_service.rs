use async_graphql::FieldResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::{
    types::UserInfoUpdateInput, user_gql_model::UserGqlModel,
    user_info_gql_model::UserInfoGqlModel, user_info_repository,
};

pub async fn find_by_user_id(
    user_id: Uuid,
    user: UserGqlModel,
    conn: &DatabaseConnection,
) -> FieldResult<UserInfoGqlModel> {
    let user_info = user_info_repository::find_one_by_user_id(user_id, conn).await?;
    UserInfoGqlModel::check_role(&user_info, &user)?;
    Ok(user_info)
}

pub async fn update_one(
    user_info_id: Uuid,
    user: UserGqlModel,
    data: UserInfoUpdateInput,
    conn: &DatabaseConnection,
) -> FieldResult<UserInfoGqlModel> {
    let user_info = user_info_repository::find_one_by_pk(user_info_id, conn).await?;
    user_info.check_role(&user)?;
    user_info_repository::user_info_update_one(user_info_id, data, conn).await;
    user_info_repository::find_one_by_pk(user_info_id, conn).await
}
