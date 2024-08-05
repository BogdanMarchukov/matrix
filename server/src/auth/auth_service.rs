use crate::{auth::web_app_data::InitDataTgWebApp, secret};
use crate::config;
use crate::entity::users;
use crate::secret::secret_service;
use crate::user::user_repository;
use async_graphql::{Error, FieldResult};
use migration::IntoCondition;
use sea_orm::{ColumnTrait, DatabaseConnection};
use super::auth_gql::LoginResult;

pub async fn login(init_data: String, conn: &DatabaseConnection) -> FieldResult<LoginResult> {
    let data: InitDataTgWebApp = match InitDataTgWebApp::de_serialize_init_data(&init_data[..]) {
        Ok(data) => data,
        Err(err) => return Err(Error::new(format!("{}, parse error", err))),
    };
    let pass = check_init_data_tg(init_data, &data.hash);
    if pass {
        let init_user = match data.user {
            Some(value) => value,
            None => return Err(Error::new(String::from("init data error: user not found"))),
        };
        let user = match user_repository::find_one(
            users::Column::TelegramId.eq(init_user.id).into_condition(),
            conn,
        )
        .await
        {
            Ok(data) => data,
            Err(err) => return Err(Error::new(format!("{}", err))),
        };
        let user = match user {
            Some(v) => v,
            None => match user_repository::create_one_by_tg(init_user, conn).await {
                Ok(data) => data,
                Err(err) => return Err(Error::new(format!("{}", err))),
            },
        };
        let jwt = match secret::secret_service::create_jwt(user.user_id.to_string()) {
            Ok(data) => data,
            Err(err) => return Err(Error::new(format!("{}, create jwt error", err))),
        };
        Ok(LoginResult {
            jwt,
            user,
        })
    } else {
        Err(Error::new(String::from("check init data error")))
    }
}

fn check_init_data_tg(init_data: String, hash: &String) -> bool {
    let tg_salt = String::from("WebAppData");
    let bot_token = config::get_bot_token();
    let secret_key = secret_service::create_hash_sha256(&bot_token, &tg_salt);
    let init_data_hash = secret_service::create_hash_sha256(&init_data, &secret_key);
    &init_data_hash == hash
}
