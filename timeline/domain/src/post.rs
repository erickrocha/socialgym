use mongodb::bson::DateTime;
use crate::media::Media;
use crate::reaction::Reaction;
use crate::comment::Comment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::mention::Mention;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub author_id: i32,
    pub author_uuid: String,
    pub author_name: String,
    pub author_object_key: Option<String>,
    pub author_avatar: Option<String>,
    pub content: String,
    #[serde(default)]
    pub media: Vec<Media>,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    #[serde(default)]
    pub comments: Vec<Comment>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(default)]
    pub mentions: Vec<Mention>,
}

impl Post {
    #[allow(clippy::too_many_arguments)]
    pub fn new(author_id: i32,author_uuid: String,author_name:String,author_object_key: Option<String>,
               author_avatar_url: Option<String>, content: String, media: Vec<Media>, mentions: Vec<Mention>) -> Self {
        Self::updated(Uuid::new_v4().to_string(),author_id, author_uuid, author_name, author_object_key, author_avatar_url, content, media, Vec::new(), Vec::new(), mentions)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn updated(uuid: String,author_id: i32, author_uuid: String,author_name:String,author_object_key: Option<String>,
               author_avatar: Option<String>, content: String, media: Vec<Media>, reactions: Vec<Reaction>, comments: Vec<Comment>, mentions: Vec<Mention>) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            author_id,
            author_uuid,
            author_name,
            author_object_key,
            author_avatar,
            content,
            media,
            reactions,
            comments,
            created_at: now,
            updated_at: now,
            mentions,
        }
    }
}
