use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InAppNotification {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub notification_type: String,
    pub recipient_person_uuid: String,
    pub actor_person_uuid: String,
    pub actor_name: String,
    pub post_uuid: Option<String>,
    pub comment_uuid: Option<String>,
    pub entity_type: String,
    pub entity_uuid: String,
    pub snippet: String,
    pub read: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl InAppNotification {
    #[allow(clippy::too_many_arguments)]
    pub fn from_mention_event(
        uuid: String,
        recipient_person_uuid: String,
        actor_person_uuid: String,
        actor_name: String,
        post_uuid: Option<String>,
        comment_uuid: Option<String>,
        entity_type: String,
        entity_uuid: String,
        snippet: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            notification_type: "Mention".to_string(),
            recipient_person_uuid,
            actor_person_uuid,
            actor_name,
            post_uuid,
            comment_uuid,
            entity_type,
            entity_uuid,
            snippet,
            read: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// A new chat message notification, surfaced through the same pull path as
    /// mentions so the unread badge keeps working. `uuid` is the idempotency
    /// key `format!("{message_uuid}:{recipient_person_uuid}")`.
    pub fn from_chat_message(
        uuid: String,
        recipient_person_uuid: String,
        actor_person_uuid: String,
        actor_name: String,
        conversation_uuid: String,
        snippet: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            notification_type: "ChatMessage".to_string(),
            recipient_person_uuid,
            actor_person_uuid,
            actor_name,
            post_uuid: None,
            comment_uuid: None,
            entity_type: "conversation".to_string(),
            entity_uuid: conversation_uuid,
            snippet,
            read: false,
            created_at: now,
            updated_at: now,
        }
    }
}

