use crate::{auth::web_app_data::InitDataTgWebApp, entity::users};
use crate::config;
use crate::secret::secret_service;
use async_graphql::{Error, FieldResult};
use sea_orm::Condition;
use crate::user::user_repository;

pub fn login(init_data: String) -> FieldResult<bool> {
    let data: InitDataTgWebApp =
        match InitDataTgWebApp::de_serialize_init_data(&init_data[..]) {
            Ok(data) => data,
            Err(err) => return Err(Error::new(format!("{}, parse error", err))),
        };
    let pass = check_init_data_tg(init_data, &data.hash);
    if pass {
        // TODO: jwt create
        Ok(true)
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
