use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationJson {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    pub notification_type: String,
    pub recipient_person_uuid: String,
    pub actor_person_uuid: String,
    pub actor_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_uuid: Option<String>,
    pub entity_type: String,
    pub entity_uuid: String,
    pub snippet: String,
    pub read: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarkNotificationReadJson {
    pub read: bool,
}

