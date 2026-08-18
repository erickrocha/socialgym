use crate::gateway::user_gateway::UserGateway;
use chrono::Utc;
use sea_orm::DbConn;

pub struct TokenRevocation {}

impl TokenRevocation {
    /// Invalidates every token issued to `user_id` before now, by moving the user's
    /// revoke-all watermark (`token_valid_after`) forward. Reused by the lockout path
    /// (`Authentication::execute`) and, once it exists, a future change-password use case.
    pub async fn revoke_all_for_user(db: &DbConn, user_id: i32) {
        log::info!("Revoking all tokens for user_id={}", user_id);
        if let Err(e) = UserGateway::set_token_valid_after(db, user_id, Utc::now()).await {
            log::error!("Failed to revoke all tokens for user_id={}: {}", user_id, e);
        }
    }
}
