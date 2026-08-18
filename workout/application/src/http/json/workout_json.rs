use crate::http::json::exercise_json::ExerciseJson;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkoutJson {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub owner_id: i32,
    pub owner_uuid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub muscle_group: Option<String>,
    pub visibility: String,
    #[serde(default)]
    pub exercises: Vec<ExerciseJson>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
