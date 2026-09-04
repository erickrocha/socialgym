use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

pub const SENDER_KIND_PERSON: &str = "Person";
pub const SENDER_KIND_BUSINESS_PROFILE: &str = "BusinessProfile";

pub const MESSAGE_MEDIA_TYPE_IMAGE: &str = "Image";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageMedia {
    pub media_type: String,
    pub object_key: String,
    /// CloudFront-signed URL, filled on read only — never persisted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub conversation_uuid: String,
    /// The human who typed the message — always present.
    pub sender_person_uuid: String,
    pub sender_kind: String,
    /// Denormalized at write: the person's name, or the business profile name
    /// when `sender_kind == BusinessProfile`.
    pub sender_display_name: String,
    /// Person avatar object key, or business logo object key.
    pub sender_object_key: Option<String>,
    /// Set only when `sender_kind == BusinessProfile`.
    pub sender_business_profile_uuid: Option<String>,
    /// May be empty when `media` is non-empty.
    pub body: String,
    #[serde(default)]
    pub media: Vec<MessageMedia>,
    pub client_message_id: String,
    /// `format!("{conversation_uuid}:{client_message_id}")` — unique index,
    /// makes send idempotent under client retries.
    pub dedupe_key: String,
    /// Server-assigned at insert; the ordering key.
    pub sent_at: DateTime,
    pub created_at: DateTime,
}

impl Message {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uuid: String,
        conversation_uuid: String,
        sender_person_uuid: String,
        sender_kind: String,
        sender_display_name: String,
        sender_object_key: Option<String>,
        sender_business_profile_uuid: Option<String>,
        body: String,
        media: Vec<MessageMedia>,
        client_message_id: String,
    ) -> Self {
        let now = DateTime::now();
        let dedupe_key = format!("{conversation_uuid}:{client_message_id}");
        Self {
            uuid,
            conversation_uuid,
            sender_person_uuid,
            sender_kind,
            sender_display_name,
            sender_object_key,
            sender_business_profile_uuid,
            body,
            media,
            client_message_id,
            dedupe_key,
            sent_at: now,
            created_at: now,
        }
    }
}
