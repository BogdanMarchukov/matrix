use crate::auth::auth_gql::LoginInput;
use crate::config;
use crate::secret::secret_service;
use async_graphql::{Error, FieldResult};

pub fn login(init_data: LoginInput) -> FieldResult<bool> {
    let pass = check_init_data_tg(&init_data.init_data, &init_data.hash);
    if pass {
        Ok(true)
    } else {
        Err(Error::new(String::from("check hash error")))
    }
}

fn check_init_data_tg(init_data: &String, hash: &String) -> bool {
    let tg_salt = String::from("WebAppData");
    let bot_token = config::get_bot_token();
    let secret_key = secret_service::create_hash_sha256(&bot_token, &tg_salt);
    let init_data_hash = secret_service::create_hash_sha256(&init_data, &secret_key);
    &init_data_hash == hash
}
