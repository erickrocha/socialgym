use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expire_in: i64,
    pub refresh_token: String,
    pub username: String,
    pub uuid: String,
    pub name: String,
    pub person_id: i32,
    pub person_uuid: String,
    pub person_object_key: String,
}


#[derive(Debug,Clone, Deserialize,Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub uuid: String,
    pub name: String,
    pub person_id: i32,
    pub person_uuid: String,
    pub person_object_key: String,
    /// Present when the caller switched into one of their business profiles
    /// (workout's `switch_business_profile`). Optional so tokens issued before
    /// this field existed still decode.
    #[serde(default)]
    pub active_business_profile_id: Option<i32>,
    #[serde(default)]
    pub active_business_profile_uuid: Option<String>,
}
