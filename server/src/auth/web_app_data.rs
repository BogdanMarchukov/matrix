use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct InitDataTgWebApp {
    pub query_id: String,
    pub user: Option<UserTgWebApp>,
    pub receiver: Option<UserTgWebApp>,
    pub chat: Option<ChatTgWebApp>,
    pub chat_type: Option<ChatType>,
    pub chat_instance: Option<String>,
    pub start_param: Option<String>,
    pub can_send_after: Option<i32>,
    pub auth_date: Option<i32>,
    pub hash: String,
}

impl InitDataTgWebApp {
    pub fn de_serialize_init_data(init_data: &str) -> Result<InitDataTgWebApp> {
        let value: Result<InitDataTgWebApp> = serde_json::from_str(init_data);
        value
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserTgWebApp {
    pub id: i32,
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
