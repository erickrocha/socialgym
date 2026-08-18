use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LogoutRequest {
    /// Optional: also revoke the refresh token paired with this session.
    pub refresh_token: Option<String>,
}
