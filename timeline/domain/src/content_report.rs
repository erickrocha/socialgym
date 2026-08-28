use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModerationEvent {
    pub actor_person_uuid: String,
    pub action: String,
    pub reason: String,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentReport {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub target_type: String,
    pub target_id: String,
    pub post_id: String,
    pub reporter_person_uuid: String,
    pub reason: String,
    pub details: Option<String>,
    pub priority: String,
    pub status: String,
    pub assigned_moderator_uuid: Option<String>,
    pub decision: Option<String>,
    pub removal_reason: Option<String>,
    pub history: Vec<ModerationEvent>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ContentReport {
    pub fn new(
        target_type: String,
        target_id: String,
        post_id: String,
        reporter: String,
        reason: String,
        details: Option<String>,
    ) -> Self {
        let now = DateTime::now();
        let priority = if reason == "intimate_image" {
            "urgent"
        } else {
            "normal"
        };
        Self {
            uuid: Uuid::new_v4().to_string(),
            target_type,
            target_id,
            post_id,
            reporter_person_uuid: reporter,
            reason,
            details,
            priority: priority.into(),
            status: "open".into(),
            assigned_moderator_uuid: None,
            decision: None,
            removal_reason: None,
            history: vec![],
            created_at: now,
            updated_at: now,
        }
    }
}
