use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CountryJson {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub acronym: Option<String>,
    pub currency: Option<String>,
}
