use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub name:Option<String>,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub first_login: bool,
    pub person_id: Option<i32>,
    pub person_uuid: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
