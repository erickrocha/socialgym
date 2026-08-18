use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SettingsJson {
	pub id: Option<i32>,
	pub uuid: Option<String>,
	pub person_id: i32,
	pub person_uuid: String,
	pub language: String,
	pub theme: String,
	pub notifications_enabled: bool,
	pub context_menu_position: String,
	pub home_page: String,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}