use crate::enums::MediaType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {

    pub uuid: String,
    pub url: String,
    pub media_type: MediaType,
    pub object_key: String,
}
