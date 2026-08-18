use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseJson {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub owner_id: i32,
    pub owner_name: String,
    pub owner_uuid: String,
    pub description: Option<String>,
    pub sets: Option<i32>,
    pub category: Option<String>,
    pub reps_or_duration: Option<i32>,
    pub uuid: Option<String>,
    pub visibility: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedExerciseJson {
    pub content: Vec<ExerciseJson>,
    pub total_count: i64,
    pub page_number: u64,
    pub page_size: u64,
    pub has_next_page: bool,
}
