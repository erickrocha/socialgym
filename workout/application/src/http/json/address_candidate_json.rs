use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddressCandidateJson {
    pub place_id: String,
    pub formatted_address: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub locality: String,
    pub administrative_area: String,
    pub administrative_area_code: String,
    pub postal_code: Option<String>,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
}
