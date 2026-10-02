use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PushDevice {
    #[serde(rename = "_id")]
    pub id: String,
    pub device_uuid: String,
    pub person_uuid: String,
    pub platform: String,
    pub registration_token: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
