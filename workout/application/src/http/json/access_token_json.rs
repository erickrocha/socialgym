use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenJson {
    pub access_token: String,
    pub token_type: String,
    pub expire_in: i64,
    pub refresh_token: Option<String>,
    pub username: String,
    pub uuid: String,
    pub name: String,
    pub person_id: i32,
    pub person_uuid: String,
    pub person_object_key: String,
    pub active_business_profile_id: Option<i32>,
    pub active_business_profile_uuid: Option<String>,
}
