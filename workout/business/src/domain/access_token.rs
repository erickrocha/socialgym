use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expire_in: i64,
    pub refresh_token: Option<String>,
    pub username: String,
    pub uuid: String,
    pub name: String,
    pub person_id: i32,
    pub person_uuid: String,
    pub person_object_key: String,
    pub active_business_profile_id: Option<i32>,
    pub active_business_profile_uuid: Option<String>,
    pub pending_account_deletion: Option<PendingAccountDeletion>,
}

/// Present on the login response only when the account has a not-yet-purged
/// deletion request, so the client can offer to cancel it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingAccountDeletion {
    pub requested_at: NaiveDateTime,
    pub scheduled_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub uuid: String,
    pub name: String,
    pub person_id: i32,
    pub person_uuid: String,
    pub person_object_key: String,
    pub active_business_profile_id: Option<i32>,
    pub active_business_profile_uuid: Option<String>,
}

impl Claims {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        email: String,
        expiration: i64,
        uuid: String,
        name: String,
        person_id: i32,
        person_uuid: String,
        person_object_key: String,
        active_business_profile_id: Option<i32>,
        active_business_profile_uuid: Option<String>,
    ) -> Claims {
        Claims {
            sub: email,
            exp: expiration,
            iat: chrono::Utc::now().timestamp(),
            jti: Uuid::new_v4().to_string(),
            uuid,
            name,
            person_id,
            person_uuid,
            person_object_key,
            active_business_profile_id,
            active_business_profile_uuid,
        }
    }
}
