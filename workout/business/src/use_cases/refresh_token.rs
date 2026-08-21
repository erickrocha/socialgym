use crate::commons::auth_config;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::access_token::AccessToken;
use crate::domain::business_error::BusinessError;
use crate::domain::person::PersonEntityMapper;
use crate::gateway::person_gateway::PersonGateway;
use crate::gateway::token_revocation_gateway::TokenRevocationGateway;
use crate::use_cases::authentication::Authentication;
use chrono::DateTime;
use sea_orm::DbConn;

pub struct RefreshToken {}

impl RefreshToken {
    pub async fn execute(db: &DbConn, refresh_token: String) -> Result<AccessToken, BusinessError> {
        log::info!("Refreshing access token");
        let (user, claims) = Authentication::validate_refresh_token(db, refresh_token).await?;
        let person_found = PersonGateway::find_by_id(db, user.person_id).await;

        if person_found.is_none() {
            log::warn!("Person not found for person_id={}", user.person_id);
            return Err(BusinessError::new("Invalid credentials".to_string()));
        }
        let person = PersonEntityMapper::from_model(person_found.unwrap());

        // Rotate: the presented refresh token is single-use. Revoke it now so a
        // second use of the same token is caught as reuse next time around.
        if auth_config::token_revocation_enabled() {
            if let (Some(user_id), Some(expires_at)) =
                (user.id, DateTime::from_timestamp(claims.exp, 0))
            {
                if let Err(e) = TokenRevocationGateway::revoke(
                    db,
                    claims.jti,
                    user_id,
                    "refresh",
                    expires_at.naive_utc(),
                )
                .await
                {
                    log::error!("Failed to revoke rotated refresh token: {}", e);
                }
            }
        }

        log::info!("Access token refreshed for person_id={}", user.person_id);
        Ok(Authentication::generate_access_token(&user, &person, None, true))
    }
}
