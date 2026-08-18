use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseJson {

	#[serde(skip_serializing_if = "Option::is_none")]
	pub uuid: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exercise_name: Option<String>,
	pub owner_id: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub owner_name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub visibility: Option<String>,
	pub set_number: i32,
	pub reps_or_duration: i32,
	pub weight: f32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub started_at: Option<chrono::NaiveDateTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completed_at: Option<chrono::NaiveDateTime>,
}
