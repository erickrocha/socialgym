use crate::http::json::person_json::PersonJson;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FriendPageJson {
    pub suggestions: Vec<PersonJson>,
    pub friends: Vec<PersonJson>,
    pub receive_requests: Vec<PersonJson>,
    pub sent_requests: Vec<PersonJson>,
}
