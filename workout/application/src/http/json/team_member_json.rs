use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TeamMemberJson {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub business_profile_id: i32,
    pub business_profile_uuid: String,
    pub person_id: i32,
    pub person_uuid: String,
    pub status: String,
}
