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
    /// Consent state: `Pending` / `Accepted` / `Rejected` / `Cancelled`.
    /// Response-only — ignored on create/update, where the server decides it.
    #[serde(default)]
    pub status: Option<String>,
    /// The business profile that assigned this workout, when it is a
    /// team-member assignment. Response-only.
    #[serde(default)]
    pub assigned_by_profile_uuid: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    /// Only used on create, by a caller acting as a business profile, to
    /// create the workout owned by this team member instead of the profile
    /// itself. Requires an Accepted `team_members` relationship.
    #[serde(default)]
    pub target_person_uuid: Option<String>,
}
