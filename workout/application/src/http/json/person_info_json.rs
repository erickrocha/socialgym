use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonInfoJson {
    pub id: Option<i32>,
    pub person_id: i32,
    pub biography: Option<String>,
    pub relationship: Option<String>,
    pub job: Option<String>,
    pub home_town: Option<String>,
    pub current_city: Option<String>,
    pub weight: Option<f32>,
    pub height: Option<f32>,
    pub uuid: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
