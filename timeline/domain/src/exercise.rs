use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::enums::{Category, Visibility};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default,rename_all = "camelCase")]
pub struct Exercise {
	#[serde(rename = "_id")]
	pub uuid: String,
	pub exercise_name: Option<String>,
	pub owner_id: i32,
	pub owner_name: String,
	pub category: Category,
	pub visibility: Visibility,
	pub set_number: i32,
	pub reps_or_duration: i32,
	pub weight: f32,
	pub started_at: Option<DateTime>,
	pub completed_at: Option<DateTime>,
}