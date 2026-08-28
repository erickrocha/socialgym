use chrono::{DateTime, Utc};
use entity::data_export_entity;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DataExportJson {
    pub id: String,
    pub status: String,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl From<data_export_entity::Model> for DataExportJson {
    fn from(row: data_export_entity::Model) -> Self {
        Self {
            id: row.uuid.to_string(),
            status: row.status,
            error: row.error,
            created_at: row.created_at,
            updated_at: row.updated_at,
            expires_at: row.expires_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DataExportDownloadJson {
    pub url: String,
    pub expires_in_seconds: u32,
}
