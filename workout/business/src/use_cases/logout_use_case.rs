use crate::commons::auth_config;
use crate::domain::access_token::Claims;
use crate::domain::business_error::BusinessError;
use crate::gateway::token_revocation_gateway::TokenRevocationGateway;
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sea_orm::DbConn;
use std::env;

pub struct LogoutUseCase {}

impl LogoutUseCase {
    /// Revokes the caller's current access token (`jti`/`exp`, taken from the already
    /// validated request) and, if provided, its paired refresh token. A no-op when
    /// `TOKEN_REVOCATION_ENABLED`/`AUTH_RULES_ENABLED` disable revocation for dev.
    pub async fn execute(db: &DbConn, user_id: i32, jti: String, exp: i64, refresh_token: Option<String>) -> Result<(), BusinessError> {
        if !auth_config::token_revocation_enabled() {
            return Ok(());
        }

        if let Err(e) = TokenRevocationGateway::revoke(db, jti, user_id, "access", Self::naive_from_timestamp(exp)).await {
            log::error!("Failed to revoke access token on logout for user_id={}: {}", user_id, e);
        }

        if let Some(refresh_token) = refresh_token {
            if let Some((refresh_jti, refresh_exp)) = Self::decode_refresh_claims(&refresh_token) {
                if let Err(e) = TokenRevocationGateway::revoke(db, refresh_jti, user_id, "refresh", Self::naive_from_timestamp(refresh_exp)).await {
                    log::error!("Failed to revoke refresh token on logout for user_id={}: {}", user_id, e);
                }
            }
        }

        Ok(())
    }

    fn decode_refresh_claims(token: &str) -> Option<(String, i64)> {
        let public_key = env::var("REFRESH_TOKEN_SECRET").ok()?;
        let result = decode::<Claims>(
            token,
            &DecodingKey::from_secret(public_key.as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .ok()?;
        Some((result.claims.jti, result.claims.exp))
    }

    fn naive_from_timestamp(ts: i64) -> NaiveDateTime {
        DateTime::from_timestamp(ts, 0)
            .map(|dt| dt.naive_utc())
            .unwrap_or_else(|| Utc::now().naive_utc())
    }
}
