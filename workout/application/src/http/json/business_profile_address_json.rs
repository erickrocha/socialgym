use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BusinessProfileAddressJson {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub business_profile_id: i32,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: Option<String>,
    pub country_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

