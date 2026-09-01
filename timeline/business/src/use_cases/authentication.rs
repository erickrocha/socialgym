use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use domain::access_token::Claims;
use domain::business_error::BusinessError;
use domain::user::User;
use std::env;

pub struct Authentication {}

impl Authentication {
    pub async fn validate(token: String) -> Result<User, BusinessError> {
        log::info!("Validating access token");
        let public_key = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(public_key.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        if result.is_err() {
            log::warn!("Token is invalid: {:?}", result);
            return Err(BusinessError::new("Token is invalid".to_string()));
        }

        let authentication = result.unwrap();
        log::info!("Token is valid {:?}", authentication.claims.sub.clone());
        let email = authentication.claims.sub;
        let uuid = authentication.claims.uuid;
        let person_id = authentication.claims.person_id;
        let name = authentication.claims.name;
        let person_uuid = authentication.claims.person_uuid;
        let person_object_key = authentication.claims.person_object_key;
        let active_business_profile_uuid = authentication.claims.active_business_profile_uuid;

        let user = User::new(
            name,
            email,
            uuid,
            person_id,
            person_uuid,
            person_object_key,
            active_business_profile_uuid,
        );
        Ok(user)
    }

    pub async fn validate_refresh_token(token: String) -> Result<User, BusinessError> {
        log::info!("Validating refresh token");
        let public_key =
            env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(public_key.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        if result.is_err() {
            log::warn!("Refresh token is invalid: {:?}", result);
            return Err(BusinessError::new("Token is invalid".to_string()));
        }

        let authentication = result.unwrap();
        log::info!("Token is valid {:?}", authentication.claims.sub.clone());
        let email = authentication.claims.sub;

        let uuid = authentication.claims.uuid;
        let person_id = authentication.claims.person_id;
        let name = authentication.claims.name;
        let person_uuid = authentication.claims.person_uuid;
        let person_object_key = authentication.claims.person_object_key;
        let active_business_profile_uuid = authentication.claims.active_business_profile_uuid;

        let user = User::new(
            name,
            email,
            uuid,
            person_id,
            person_uuid,
            person_object_key,
            active_business_profile_uuid,
        );
        Ok(user)
    }
}
