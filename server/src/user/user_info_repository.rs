use crate::entity::user_info;
use crate::{entity::prelude::UserInfo, errors::gql_error::GqlError};
use async_graphql::{ErrorExtensions, FieldResult};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

use super::types::UserInfoUpdateInput;
use super::user_info_gql_model::UserInfoGqlModel;

pub async fn find_one_by_user_id(
    user_id: Uuid,
    conn: &DatabaseConnection,
) -> FieldResult<UserInfoGqlModel> {
    if let Ok(Some(result)) = UserInfo::find()
        .filter(user_info::Column::UserId.eq(user_id))
        .one(conn)
        .await
    {
        Ok(UserInfoGqlModel::new(result))
    } else {
        Err(GqlError::NotFound("user not found".to_owned()).extend())
    }
}

pub async fn find_one_by_pk(
    user_info_id: Uuid,
    conn: &DatabaseConnection,
) -> FieldResult<UserInfoGqlModel> {
    if let Ok(result) = UserInfo::find_by_id(user_info_id).one(conn).await {
        if let Some(u) = result {
            Ok(UserInfoGqlModel::new(u))
        } else {
            Err(GqlError::NotFound("userInfo not found".to_owned()).extend())
        }
    } else {
        Err(GqlError::ServerError("database error".to_owned()).extend())
    }
}

pub async fn user_info_update_one(
    user_info_id: Uuid,
    data: UserInfoUpdateInput,
    conn: &DatabaseConnection,
) -> bool {
    if let Ok(Some(result)) = UserInfo::find_by_id(user_info_id).one(conn).await {
        let mut user_info_active: user_info::ActiveModel = result.into();
        if let Some(city) = data.city {
            user_info_active.city = Set(Some(city));
        }
        if let Some(date_of_birth) = data.date_of_birth {
            user_info_active.date_of_birth = Set(Some(date_of_birth));
        }
        if let Some(hour_of_birth) = data.hour_of_birth {
            user_info_active.hour_of_birth = Set(Some(hour_of_birth));
        }
        if let Some(min_of_birth) = data.min_of_birth {
            user_info_active.min_of_birth = Set(Some(min_of_birth));
        }
        user_info_active.update(conn).await.is_ok()
    } else {
        false
    }
}

pub async fn create_one_by_user_id(user_id: Uuid, conn: &DatabaseTransaction) -> FieldResult<bool> {
    let new_user_info = user_info::ActiveModel {
        user_info_id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        ..Default::default()
    };
    user_info::Entity::insert(new_user_info).exec(conn).await?;
    Ok(true)
}
