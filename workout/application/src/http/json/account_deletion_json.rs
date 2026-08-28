use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountDeletionRequestJson {
    /// `true` deletes right away (picked up on the next purge sweep); `false`
    /// (default) schedules deletion after the grace period, cancellable by
    /// logging back in before then.
    #[serde(default)]
    pub immediate: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountDeletionStatusJson {
    pub requested_at: chrono::NaiveDateTime,
    pub scheduled_at: chrono::NaiveDateTime,
}
