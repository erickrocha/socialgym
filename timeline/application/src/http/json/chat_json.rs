use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateDirectConversationJson {
    pub target_person_uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBusinessTeamGroupJson {
    pub business_profile_uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBusinessDirectJson {
    pub business_profile_uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_person_uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversationParticipantJson {
    pub person_uuid: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_read_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_read_message_uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LastMessagePreviewJson {
    pub message_uuid: String,
    pub sender_person_uuid: String,
    pub sender_display_name: String,
    pub snippet: String,
    pub sent_at: chrono::NaiveDateTime,
    pub has_media: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversationJson {
    pub uuid: String,
    pub conversation_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile_name: Option<String>,
    /// CloudFront-signed URL when a logo is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile_logo_url: Option<String>,
    pub participant_person_uuids: Vec<String>,
    pub participants: Vec<ConversationParticipantJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message: Option<LastMessagePreviewJson>,
    pub unread: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageMediaJson {
    pub media_type: String,
    pub object_key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageJson {
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub media: Vec<MessageMediaJson>,
    pub client_message_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageJson {
    pub uuid: String,
    pub conversation_uuid: String,
    pub sender_person_uuid: String,
    pub sender_kind: String,
    pub sender_display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_profile_uuid: Option<String>,
    pub body: String,
    pub media: Vec<MessageMediaJson>,
    pub client_message_id: String,
    pub sent_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarkReadJson {
    pub last_read_message_uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarkReadResultJson {
    pub read: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatPageQuery {
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessagesQuery {
    pub page: Option<u32>,
    /// Epoch milliseconds — when present, returns messages strictly newer than
    /// this (WebSocket reconnect replay).
    pub since: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceQuery {
    /// Comma-separated person uuids to check.
    pub uuids: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PresenceJson {
    /// Subset of the requested uuids currently holding a chat WebSocket.
    pub online: Vec<String>,
}
