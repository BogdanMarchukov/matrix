use std::collections::HashMap;
use std::fmt::Debug;

use super::auth_gql::LoginResult;
use crate::db_utils::get_transaction;
use crate::entity::users;
use crate::errors::gql_error::GqlError;
use crate::secret::secret_service;
use crate::secret::secret_service::JwtPayload;
use crate::user::user_gql_model::{User, UserGqlModel};
use crate::user::{user_info_repository, user_repository, user_tariff_plan_repository};
use crate::{auth::web_app_data::InitDataTgWebApp, secret};
use actix_web::HttpRequest;
use async_graphql::{ErrorExtensions, FieldResult};
use jsonwebtoken::TokenData;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection};
use uuid::Uuid;

pub async fn login(init_data: String, conn: &DatabaseConnection) -> FieldResult<LoginResult> {
    let data: InitDataTgWebApp = match InitDataTgWebApp::de_serialize_init_data(&init_data[..]) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };
    let pass = check_init_data_tg(init_data);
    if pass {
        let init_user = match data.user {
            Some(value) => value,
            None => {
                return Err(
                    GqlError::BadRequest("init data error: user not found".to_string()).extend(),
                )
            }
        };
        let user = match user_repository::find_one(
            Condition::all().add(users::Column::TelegramId.eq(init_user.id)),
            conn,
        )
        .await
        {
            Ok(data) => data,
            Err(_) => return Err(GqlError::ServerError("database error".to_string()).extend()),
        };
        let user = match user {
            Some(v) => v,
            None => match user_repository::create_one_by_tg(init_user, conn).await {
                Ok(data) => {
                    let trn = get_transaction().await?;
                    user_info_repository::create_one_by_user_id(data.0.user_id, &trn).await?;
                    user_tariff_plan_repository::find_or_create_free(data.0.user_id, conn).await?;
                    trn.commit().await?;
                    data
                }
                Err(err) => return Err(GqlError::ServerError(format!("{}", err)).extend()),
            },
        };
        let jwt = match secret::secret_service::create_jwt(user.0.user_id.to_string()) {
            Ok(data) => data,
            Err(err) => return Err(GqlError::ServerError(format!("{}", err)).extend()),
        };
        Ok(LoginResult { jwt, user })
    } else {
        Err(GqlError::BadRequest("init data is not check".to_string()).extend())
    }
}

pub async fn dev_login(user_id: Uuid, conn: &DatabaseConnection) -> FieldResult<LoginResult> {
    if let Ok(Some(user)) = user_repository::find_by_id(&user_id.to_string(), conn).await {
        let jwt = match secret::secret_service::create_jwt(user.0.user_id.to_string()) {
            Ok(data) => data,
            Err(err) => return Err(GqlError::ServerError(format!("{}", err)).extend()),
        };
        Ok(LoginResult { user, jwt })
    } else {
        Err(GqlError::NotFound("User not found".to_string()).extend())
    }
}

pub async fn get_user_from_request(
    http_request: &HttpRequest,
    conn: &DatabaseConnection,
) -> (HashMap<String, String>, Option<User>) {
    let mut headers: HashMap<String, String> = HashMap::new();
    for (key, value) in http_request.headers().iter() {
        headers.insert(
            key.to_string().to_lowercase(),
            value.to_str().unwrap_or("").to_string(),
        );
    }
    let jwt_payload: Option<TokenData<JwtPayload>> = match headers.get("authorization") {
        Some(token) => match secret_service::verify_jwt(token.to_owned()) {
            Ok(p) => Some(p),
            Err(_) => None,
        },
        None => None,
    };
    let user = match jwt_payload {
        Some(payload) => match user_repository::find_by_id(&payload.claims.sub, conn).await {
            Ok(user) => user,
            Err(_) => None,
        },
        None => None,
    };

    (headers, user)
}

fn check_init_data_tg(_: String) -> bool {
    true
    //  let params: HashMap<_, _> = form_urlencoded::parse(init_data.as_bytes())
    //      .into_owned()
    //      .collect();

    //  // Извлекаем хэш из параметров
    //  let hash = match params.get("hash") {
    //      Some(hash) => decode(hash).expect("error").to_string(),
    //      None => return false,
    //  };
    //  let mut sorted_params: Vec<_> = params.iter().filter(|&(k, _)| k != "hash").collect();
    //  sorted_params.sort();

    //  let data_check_string = sorted_params
    //      .into_iter()
    //      .map(|(k, v)| format!("{}={}", k, v))
    //      .collect::<Vec<_>>()
    //      .join("\n");

    //  println!("{}, data check:", &data_check_string);
    //  println!("{}, hesh data", &hash);

    //  let tg_salt = String::from("WebAppData");
    //  let bot_token = config::get_bot_token();
    //  let secret_key = secret_service::create_hash_sha256(&bot_token, &tg_salt);
    //  let init_data_hash = secret_service::create_hash_sha256(&data_check_string, &secret_key);
    //  init_data_hash == hash
}
