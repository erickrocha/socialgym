use crate::http::json::business_profile_address_json::BusinessProfileAddressJson;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BusinessProfileJson {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub owner_id: i32,
    pub owner_uuid: String,
    pub tax_id: String,
    pub business_name: String,
    pub business_type: String,
    pub social_name: Option<String>,
    pub logo: Option<String>,
    pub object_key: Option<String>,
    pub cover_image: Option<String>,
    pub addresses: Vec<BusinessProfileAddressJson>,
}

