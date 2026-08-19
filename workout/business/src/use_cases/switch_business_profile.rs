use crate::commons::auth_config;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::access_token::AccessToken;
use crate::domain::business_profile::BusinessProfileEntityMapper;
use crate::domain::person::PersonEntityMapper;
use crate::domain::user::User;
use crate::gateway::business_profile_gateway::BusinessProfileGateway;
use crate::gateway::person_gateway::PersonGateway;
use crate::gateway::token_revocation_gateway::TokenRevocationGateway;
use crate::use_cases::authentication::Authentication;
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::DbConn;

pub enum SwitchBusinessProfileError {
    NotFound,
    Forbidden,
}

pub struct SwitchBusinessProfile {}

impl SwitchBusinessProfile {
    pub async fn activate(db: &DbConn,user: &User,business_profile_uuid: String,current_jti: String,current_exp: i64) -> Result<AccessToken, SwitchBusinessProfileError> {
        log::info!("Activating business profile uuid={} for user_id={:?}",business_profile_uuid,user.id);

        let model = BusinessProfileGateway::find_by_uuid(db, business_profile_uuid.as_str())
            .await
            .map_err(|_| SwitchBusinessProfileError::NotFound)?
            .ok_or(SwitchBusinessProfileError::NotFound)?;
        let business_profile = BusinessProfileEntityMapper::from_model(model);

        if business_profile.owner_id != user.id.unwrap() {
            log::warn!("User_id={:?} is not the owner of business profile uuid={}",user.id,business_profile_uuid);
            return Err(SwitchBusinessProfileError::Forbidden);
        }

        let person = Self::load_person(db, user).await;
        let access_token = Authentication::generate_access_token(user,&person,Some(&business_profile),true);
        Self::revoke_previous_token(db, user, current_jti, current_exp).await;
        Ok(access_token)
    }

    pub async fn deactivate(db: &DbConn, user: &User, current_jti: String, current_exp: i64) -> AccessToken {
        log::info!("Deactivating business profile context for user_id={:?}", user.id);

        let person = Self::load_person(db, user).await;
        let access_token = Authentication::generate_access_token(user, &person, None, true);
        Self::revoke_previous_token(db, user, current_jti, current_exp).await;
        access_token
    }

    /// Revokes the access token the caller authenticated with, now that a fresh one
    /// reflecting the new profile context has been minted. The paired refresh token
    /// is left alone — this endpoint never receives it, and a reused refresh token
    /// just mints a token without the profile context, which is acceptable drift.
    async fn revoke_previous_token(db: &DbConn, user: &User, current_jti: String, current_exp: i64) {
        if !auth_config::token_revocation_enabled() {
            return;
        }
        let expires_at: NaiveDateTime = DateTime::from_timestamp(current_exp, 0)
            .map(|dt| dt.naive_utc())
            .unwrap_or_else(|| Utc::now().naive_utc());
        if let Err(e) = TokenRevocationGateway::revoke(db, current_jti, user.id.unwrap(), "access", expires_at).await {
            log::error!("Failed to revoke previous access token for user_id={:?}: {}", user.id, e);
        }
    }

    async fn load_person(db: &DbConn, user: &User) -> crate::domain::person::Person {
        let model = PersonGateway::find_by_id(db, user.person_id)
            .await
            .expect("person must exist for an authenticated user");
        PersonEntityMapper::from_model(model)
    }
}
