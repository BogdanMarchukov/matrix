use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entity::notify;

#[derive(Clone, SimpleObject)]
#[graphql(name = "Notify")]
pub struct NotifyGqlModel {
    pub notify_id: Uuid,
    pub payload: String,
    pub title: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub user_id: Uuid,
}

impl NotifyGqlModel {
    pub fn new(notify_model: notify::Model) -> Self {
        Self {
            notify_id: notify_model.notify_id,
            payload: notify_model.payload,
            title: notify_model.title,
            is_read: notify_model.is_read,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(notify_model.created_at, Utc),
            user_id: notify_model.user_id,
        }
    }
}
