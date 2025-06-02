use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::errors::gql_error::GqlError;

use super::{
    types::UserInfoUpdateInput, user_gql_model::{UserGqlModel, User},
    user_info_gql_model::UserInfoGqlModel, user_info_repository,
};

pub async fn find_by_user_id(
    user_id: Uuid,
    user: User,
    conn: &DatabaseConnection,
) -> FieldResult<UserInfoGqlModel> {
    let user_info = user_info_repository::find_one_by_user_id(user_id, conn).await?;
    UserInfoGqlModel::check_role(&user_info, &user)?;
    Ok(user_info)
}

pub async fn update_one(
    user_info_id: Uuid,
    user: User,
    data: UserInfoUpdateInput,
    conn: &DatabaseConnection,
) -> FieldResult<UserInfoGqlModel> {
    let user_info = user_info_repository::find_one_by_pk(user_info_id, conn).await?;
    user_info.check_role(&user)?;
    if let Some(hour) = data.hour_of_birth {
        if !(0..24).contains(&hour) {
            return Err(GqlError::BadRequest(("hour_of_birth is not valid".to_string())).extend());
        }
    }
    if let Some(min) = data.min_of_birth {
        if !(0..60).contains(&min) {
            return Err(GqlError::BadRequest(("min_of_birth is not valid".to_string())).extend());
        }
    }
    user_info_repository::user_info_update_one(user_info_id, data, conn).await;
    user_info_repository::find_one_by_pk(user_info_id, conn).await
}
