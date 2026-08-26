use crate::commons::auth_config;
use crate::domain::access_token::{AccessToken, Claims, PendingAccountDeletion};
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::BusinessProfile;
use crate::domain::user::{User, UserEntityMapper};
use crate::gateway::token_revocation_gateway::TokenRevocationGateway;
use crate::gateway::user_gateway::UserGateway;
use crate::use_cases::token_revocation::TokenRevocation;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::DbConn;
use std::env;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::person::{Person, PersonEntityMapper};
use crate::gateway::person_gateway::PersonGateway;

pub struct Authentication {}

#[derive(Debug)]
pub enum AuthenticationError {
    InvalidCredentials,
    AccountLocked { retry_after_seconds: i64 },
    AccountDisabled,
}

#[derive(Debug)]
pub enum ValidateError {
    Invalid,
    Revoked,
}

/// The caller's identity plus enough of its token's claims (`jti`/`exp`) to let
/// downstream use cases (logout, profile switch) revoke that specific token.
#[derive(Debug, Clone)]
pub struct AuthenticatedContext {
    pub user: User,
    pub active_business_profile_id: Option<i32>,
    pub jti: String,
    pub exp: i64,
}

impl Authentication {
    pub async fn execute(db: &DbConn, email: String, password: String) -> Result<AccessToken, AuthenticationError> {
        log::info!("Login user: {:?}", email);
        if email.is_empty() || password.is_empty() {
            log::info!("Email and password are required");
            return Err(AuthenticationError::InvalidCredentials);
        }
        let result = UserGateway::find_by_email(db, email).await;

        if result.is_err() {
            log::info!("User not found");
            return Err(AuthenticationError::InvalidCredentials);
        }

        let user_found = result.unwrap();

        if user_found.is_none() {
            log::info!("User not found");
            return Err(AuthenticationError::InvalidCredentials);
        }
        let mut user = UserEntityMapper::from_model(user_found.unwrap());
        let user_id = user.id.unwrap();

        let person_found = PersonGateway::find_by_id(db, user.person_id).await;

        if person_found.is_none() {
            log::info!("Person not found");
            return Err(AuthenticationError::InvalidCredentials);
        }

        let person = PersonEntityMapper::from_model(person_found.unwrap());

        let lockout_enabled = auth_config::login_lockout_enabled();

        if lockout_enabled {
            if let Some(locked_until) = user.locked_until {
                let now = Utc::now().naive_utc();
                if locked_until <= now {
                    log::info!("Lockout for user_id={} has expired, resetting", user_id);
                    let _ = UserGateway::reset_lockout(db, user_id).await;
                    user.locked_until = None;
                    user.failed_login_attempts = 0;
                } else {
                    let retry_after_seconds = (locked_until - now).num_seconds();
                    log::info!("Account user_id={} is locked for {}s more", user_id, retry_after_seconds);
                    return Err(AuthenticationError::AccountLocked { retry_after_seconds });
                }
            }
        }

        // A malformed stored hash must not panic the login handler.
        let password_matches = bcrypt::verify(password, user.password.as_str()).unwrap_or_else(|e| {
            log::error!("Error verifying password hash for user_id={}: {}", user_id, e);
            false
        });
        if password_matches {
            log::info!("User is valid, generating access token");
            if !user.enabled {
                if user.deletion_scheduled_at.is_none() {
                    log::info!("Account user_id={} is disabled", user_id);
                    return Err(AuthenticationError::AccountDisabled);
                }
                log::info!(
                    "Account user_id={} has a pending deletion; allowing login so it can be cancelled",
                    user_id
                );
            }
            if lockout_enabled && user.failed_login_attempts > 0 {
                let _ = UserGateway::reset_lockout(db, user_id).await;
            }
            let access_token = Self::generate_access_token(&user, &person, None, true);
            Ok(access_token)
        } else {
            log::info!("Password is invalid");
            if lockout_enabled {
                let new_count = user.failed_login_attempts + 1;
                let _ = UserGateway::increment_failed_attempts(db, user_id, new_count).await;
                if new_count >= auth_config::login_max_failed_attempts() as i32 {
                    let locked_until = Utc::now() + Duration::seconds(auth_config::login_lockout_duration_seconds());
                    log::warn!("user_id={} exceeded max failed attempts, locking until {}", user_id, locked_until);
                    let _ = UserGateway::lock_account(db, user_id, locked_until).await;
                    if auth_config::token_revocation_enabled() {
                        TokenRevocation::revoke_all_for_user(db, user_id).await;
                    }
                }
            }
            Err(AuthenticationError::InvalidCredentials)
        }
    }

    pub fn generate_access_token(user: &User,person: &Person,active_business_profile: Option<&BusinessProfile>,reissue_refresh_token: bool) -> AccessToken {
        log::info!("Generating access token to user: {:?}", user.email);
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(3))
            .expect("valid timestamp")
            .timestamp();
        let name = user.name.clone().unwrap_or_default();
        let person_uuid = person.uuid.clone().unwrap_or_default();


        let person_avatar = person.object_key.clone().unwrap_or_else(|| "default".to_string());
        let active_business_profile_id = active_business_profile.and_then(|b| b.id);
        let active_business_profile_uuid = active_business_profile.and_then(|b| b.uuid.clone());
        let claims = Claims::new(user.email.clone(), expiration, user.uuid.clone().unwrap_or_default(), name, user.person_id,person_uuid, person_avatar, active_business_profile_id, active_business_profile_uuid.clone());
        let header = Header::new(Algorithm::HS512);
        let private_key = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(private_key.as_bytes()),
        )
        .unwrap();
        let refresh_token = if reissue_refresh_token {
            Some(Self::generate_refresh_token(user, person))
        } else {
            None
        };
        let pending_account_deletion = match (user.deletion_requested_at, user.deletion_scheduled_at) {
            (Some(requested_at), Some(scheduled_at)) => {
                Some(PendingAccountDeletion { requested_at, scheduled_at })
            }
            _ => None,
        };
        AccessToken {
            access_token: token,
            token_type: "Bearer".to_string(),
            expire_in:expiration,
            refresh_token,
            username: claims.sub.clone(),
            uuid: claims.uuid.clone(),
            name: claims.name.clone(),
            person_id: claims.person_id,
            person_uuid: claims.person_uuid.clone(),
            person_object_key: claims.person_object_key.clone(),
            active_business_profile_id: claims.active_business_profile_id,
            active_business_profile_uuid: claims.active_business_profile_uuid.clone(),
            pending_account_deletion,
        }
    }

    fn generate_refresh_token(user: &User, person: &Person) -> String {
        log::info!("Generating refresh token for user: {:?}", user.email);
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::days(7))
            .expect("valid timestamp")
            .timestamp();
        let name = user.name.clone().unwrap_or_default();
        let person_uuid = person.uuid.clone().unwrap_or_default();
        let person_object_key = person.object_key.clone().unwrap_or_else(|| "default".to_string());
        let claims = Claims::new(user.email.clone(), expiration, user.uuid.clone().unwrap_or_default(), name, user.person_id,person_uuid, person_object_key, None, None);

        let header = Header::new(Algorithm::HS512);
        let private_key = env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
        encode(&header,&claims,&EncodingKey::from_secret(private_key.as_bytes())).unwrap()
    }

    pub async fn validate(db: &DbConn, token: String) -> Result<AuthenticatedContext, ValidateError> {
        let public_key = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
        let result = decode::<Claims>(&token,&DecodingKey::from_secret(public_key.as_bytes()),&Validation::new(Algorithm::HS512));

        if result.is_err() {
            log::info!("Token is invalid: {:?}", result);
            return Err(ValidateError::Invalid);
        }

        let claims = result.unwrap().claims;
        log::info!("Token is valid {:?}", claims.sub.clone());
        let email = claims.sub.clone();
        let active_business_profile_id = claims.active_business_profile_id;

        let entity = UserGateway::find_by_email(db, email).await;

        if entity.is_err() {
            log::info!("User not found");
            return Err(ValidateError::Invalid);
        }

        let opt_entity = entity.unwrap();
        if opt_entity.is_none() {
            log::info!("User not found");
            return Err(ValidateError::Invalid);
        }
        let user = UserEntityMapper::from_model(opt_entity.unwrap());

        if auth_config::token_revocation_enabled() && Self::is_token_revoked(db, &user, &claims).await {
            return Err(ValidateError::Revoked);
        }

        Ok(AuthenticatedContext {
            user,
            active_business_profile_id,
            jti: claims.jti,
            exp: claims.exp,
        })
    }

    /// Validates a refresh token and returns it alongside its claims, so the
    /// caller (`RefreshToken::execute`) can rotate it: revoke this exact token
    /// once it's used, and detect reuse of an already-rotated one.
    pub async fn validate_refresh_token(db: &DbConn, token: String) -> Result<(User, Claims), BusinessError> {
        let public_key = env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
        let result = decode::<Claims>(&token,&DecodingKey::from_secret(public_key.as_bytes()),&Validation::new(Algorithm::HS512));

        if result.is_err() {
            log::info!("Token is invalid: {:?}", result);
            return Err(BusinessError::new("Token is invalid".to_string()));
        }

        let claims = result.unwrap().claims;
        log::info!("Token is valid {:?}", claims.sub.clone());
        let email = claims.sub.clone();

        let entity = UserGateway::find_by_email(db, email).await;

        if entity.is_err() {
            log::info!("User not found");
            return Err(BusinessError::new("User not found".to_string()));
        }

        let opt_entity = entity.unwrap();
        if opt_entity.is_none() {
            log::info!("User not found");
            return Err(BusinessError::new("User not found".to_string()));
        }
        let user = UserEntityMapper::from_model(opt_entity.unwrap());

        if auth_config::token_revocation_enabled() && Self::is_token_revoked(db, &user, &claims).await {
            // The presented refresh token was already rotated away: someone is
            // replaying a stolen/old token. Revoke the whole family and force
            // re-login rather than honoring it.
            log::warn!(
                "Refresh token reuse detected for user_id={:?}, revoking all tokens",
                user.id
            );
            if let Some(user_id) = user.id {
                TokenRevocation::revoke_all_for_user(db, user_id).await;
            }
            return Err(BusinessError::new("Token revoked".to_string()));
        }

        Ok((user, claims))
    }

    pub(crate) async fn is_token_revoked(db: &DbConn, user: &User, claims: &Claims) -> bool {
        if let Some(watermark) = user.token_valid_after {
            if let Some(iat) = DateTime::from_timestamp(claims.iat, 0).map(|dt| dt.naive_utc()) {
                if iat <= watermark {
                    return true;
                }
            }
        }
        TokenRevocationGateway::is_revoked(db, &claims.jti).await.unwrap_or(false)
    }
}
