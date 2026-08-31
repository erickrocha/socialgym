use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcceptConsentJson {
    pub document: String,
    pub version: String,
    pub accepted: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsentJson {
    pub id: i32,
    pub document: String,
    pub version: String,
    pub accepted_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl From<entity::consent_entity::Model> for ConsentJson {
    fn from(value: entity::consent_entity::Model) -> Self {
        Self {
            id: value.id,
            document: value.document,
            version: value.version,
            accepted_at: value.accepted_at,
            revoked_at: value.revoked_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PendingConsentJson {
    pub document: String,
    pub current_version: String,
    pub accepted_version: Option<String>,
}

impl From<business::use_cases::consent_use_case::PendingConsent> for PendingConsentJson {
    fn from(value: business::use_cases::consent_use_case::PendingConsent) -> Self {
        Self {
            document: value.document,
            current_version: value.current_version,
            accepted_version: value.accepted_version,
        }
    }
}
