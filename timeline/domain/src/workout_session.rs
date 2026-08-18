use mongodb::bson::{DateTime};
use serde::{Deserialize, Serialize};
use crate::exercise::Exercise;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default,rename_all = "camelCase")]
pub struct WorkoutSession {
    #[serde(rename = "_id")]
    pub uuid: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_uuid: Option<String>,
    #[serde(default)]
    pub workout_name: Option<String>,
    #[serde(default)]
    pub duration: i32,
    pub started_at: Option<DateTime>,
    pub day_of_week: Option<String>,
    pub completed_at: Option<DateTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exercises: Vec<Exercise>,
    #[serde(default)]
    pub total_volume: f32,
    #[serde(default)]
    pub total_sets: f32,
}

