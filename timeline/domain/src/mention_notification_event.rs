use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MentionNotificationEvent {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub status: String,
    pub entity_type: String,
    pub entity_uuid: String,
    pub post_uuid: Option<String>,
    pub comment_uuid: Option<String>,
    pub author_person_id: i32,
    pub author_person_uuid: String,
    pub author_name: String,
    pub mentioned_person_uuid: String,
    pub snippet: String,
    pub retry_count: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub processed_at: Option<DateTime>,
}

impl MentionNotificationEvent {

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uuid: String,
        entity_type: String,
        entity_uuid: String,
        post_uuid: Option<String>,
        comment_uuid: Option<String>,
        author_person_id: i32,
        author_person_uuid: String,
        author_name: String,
        mentioned_person_uuid: String,
        snippet: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            status: "Pending".to_string(),
            entity_type,
            entity_uuid,
            post_uuid,
            comment_uuid,
            author_person_id,
            author_person_uuid,
            author_name,
            mentioned_person_uuid,
            snippet,
            retry_count: 0,
            last_error: None,
            created_at: now,
            updated_at: now,
            processed_at: None,
        }
    }
}

