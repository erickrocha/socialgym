use crate::domain::access_token::AccessToken;
use crate::domain::business_error::BusinessError;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::person::{PersonEntityMapper};
use crate::gateway::person_gateway::PersonGateway;
use crate::use_cases::authentication::Authentication;
use sea_orm::DbConn;

pub struct RefreshToken {}

impl RefreshToken {
    pub async fn execute(db: &DbConn, refresh_token: String) -> Result<AccessToken, BusinessError> {
        log::info!("Refreshing access token");
        let user = Authentication::validate_refresh_token(db, refresh_token).await?;
        let person_found = PersonGateway::find_by_id(db, user.person_id).await;

        if person_found.is_none() {
            log::warn!("Person not found for person_id={}", user.person_id);
            return Err(BusinessError::new("Invalid credentials".to_string()));
        }
        let person = PersonEntityMapper::from_model(person_found.unwrap());

        log::info!("Access token refreshed for person_id={}", user.person_id);
        Ok(Authentication::generate_access_token(&user, &person, None, true))
    }
}
