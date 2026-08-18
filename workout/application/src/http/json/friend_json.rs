use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FriendJson {
    pub id: Option<i32>,
    pub person_id: i32,
    pub friend_id: i32,
    pub person_uuid: String,
    pub friend_uuid: String,
}
