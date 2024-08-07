use async_graphql::{Error, FieldResult};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use url_search_params::parse_url_search_params;
use urlencoding::decode;

#[derive(Serialize, Deserialize)]
pub struct InitDataTgWebApp {
    pub query_id: Option<String>,
    pub user: Option<UserTgWebApp>,
    pub receiver: Option<UserTgWebApp>,
    pub chat: Option<ChatTgWebApp>,
    pub chat_type: Option<ChatType>,
    pub chat_instance: Option<String>,
    pub start_param: Option<String>,
    pub can_send_after: Option<i32>,
    pub auth_date: Option<i32>,
    pub hash: Option<String>,
}

impl InitDataTgWebApp {
    pub fn de_serialize_init_data(&mut self, init_data: &str) -> FieldResult<UserTgWebApp> {
        let decode_init_dat = decode(init_data).expect("UTF-8");
        let params: HashMap<String, String> = parse_url_search_params(&decode_init_dat);
        let value = match params.get("user") {
            Some(v) => v,
            None => return Err(Error::new("user not found, 1")),
        };
        let user: Result<UserTgWebApp> = serde_json::from_str(&value);
        let result = match user {
            Ok(v) => {
                self.user = Some(v.clone());
                Ok(v)
            }
            Err(err) => return Err(Error::new(format!("{}", err))),
        };
        result
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserTgWebApp {
    pub id: i64,
    pub is_bot: Option<bool>,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
    pub allows_write_to_pm: Option<bool>,
    pub photo_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatTgWebApp {
    pub id: i32,
    pub KW_TYPE: String,
    pub title: String,
    pub username: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ChatType {
    private,
    group,
    supergroup,
    channel,
}
