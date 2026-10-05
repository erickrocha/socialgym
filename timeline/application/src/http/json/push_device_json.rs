use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPushDeviceJson {
    pub platform: String,
    pub registration_token: String,
}
