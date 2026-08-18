use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::http::json::exercise_json::ExerciseJson;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkoutSessionJson {
	pub uuid: Option<String>,
	pub person_uuid: String,
	pub workout_name: Option<String>,
	pub duration: i32,
	pub started_at: Option<chrono::NaiveDateTime>,
	pub day_of_week: Option<String>,
	pub completed_at: Option<chrono::NaiveDateTime>,
	pub executed_sets: Vec<ExerciseJson>,
	pub total_volume: f32,
	pub total_sets: f32,
}
