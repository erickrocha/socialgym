use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::mention::Mention;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub post_uuid: String,
    pub author_uuid: String,
    pub author_name: String,
    pub author_object_key: Option<String>,
    pub author_avatar: Option<String>,
    pub content: String,
    pub parent_uuid: Option<String>, // For replies
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(default)]
    pub mentions: Vec<Mention>,
}

impl Comment {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uuid: String,
        post_uuid: String,
        author_uuid: String,
        author_name: String,
        author_object_key: Option<String>,
        author_avatar: Option<String>,
        content: String,
        parent_uuid: Option<String>,
        mentions: Vec<Mention>,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            post_uuid,
            author_uuid,
            author_name,
            author_object_key,
            author_avatar,
            content,
            parent_uuid,
            created_at: now,
            updated_at: now,
            mentions,
        }
    }

    pub fn update(&mut self, content: String) {
        self.content = content;
        self.updated_at = DateTime::now();
    }
}
