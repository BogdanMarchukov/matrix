use super::auth_gql::LoginResult;
use crate::entity::users;
use crate::errors::gql_error::GqlError;
use crate::user::user_repository;
use crate::{auth::web_app_data::InitDataTgWebApp, secret};
use async_graphql::{ErrorExtensions, FieldResult};
use migration::IntoCondition;
use sea_orm::{ColumnTrait, DatabaseConnection};

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
            users::Column::TelegramId.eq(init_user.id).into_condition(),
            conn,
        )
        .await
        {
            Ok(data) => data,
            Err(err) => return Err(GqlError::ServerError("database error".to_string()).extend()),
        };
        let user = match user {
            Some(v) => v,
            None => match user_repository::create_one_by_tg(init_user, conn).await {
                Ok(data) => data,
                Err(err) => return Err(GqlError::ServerError(format!("{}", err)).extend()),
            },
        };
        let jwt = match secret::secret_service::create_jwt(user.user_id.to_string()) {
            Ok(data) => data,
            Err(err) => return Err(GqlError::ServerError(format!("{}", err)).extend()),
        };
        Ok(LoginResult { jwt, user })
    } else {
        Err(GqlError::BadRequest("init data is not check".to_string()).extend())
    }
}

fn check_init_data_tg(init_data: String) -> bool {
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
