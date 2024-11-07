use chrono::NaiveDateTime;

#[derive(Default)]
pub struct NewsletterUpdateInput {
    pub title: Option<String>,
    pub payload: Option<String>,
    pub publish_at: Option<NaiveDateTime>,
    pub is_published: Option<bool>,
}

