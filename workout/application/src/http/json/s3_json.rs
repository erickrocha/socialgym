use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct S3Json {
	pub url: String,
	pub object_key: String,
	pub owner_id: i32,
}

impl S3Json {
	
	pub fn new(url: String, object_key: String, owner_id: i32) -> Self {
		Self { url, object_key, owner_id }
	}
}