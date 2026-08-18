use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::person_info::{PersonInfo, PersonInfoEntityMapper};
use crate::gateway::person_info_gateway::PersonInfoGateway;
use sea_orm::DbConn;

pub struct PersonInfoUseCase {}

impl PersonInfoUseCase {
    pub async fn get(db: &DbConn, id: i32) -> Result<PersonInfo, BusinessError> {
        let result = PersonInfoGateway::find_by_id(db, id).await;
        if result.is_err() {
            log::error!("Error: {}", result.err().unwrap());
            return Err(BusinessError::new("Failed to get PersonInfo".to_string()));
        }

        let person_info_domain = result.unwrap();
        match person_info_domain {
            Some(person_info) => Ok(PersonInfoEntityMapper::from_model(person_info)),
            None => {
                log::error!("PersonInfo not found for person id: {}", id);
                Err(BusinessError::new("PersonInfo not found".to_string()))
            }
        }
    }

    pub async fn update(
        db: &DbConn,
        id: i32,
        payload: PersonInfo,
    ) -> Result<PersonInfo, BusinessError> {
        log::info!("Updating PersonInfo for id: {}", id);
        let entity_found = Self::get(db, id).await;

        if entity_found.is_err() {
            log::error!("Error to find person info id: {}", id);
            return Err(BusinessError::new("PersonInfo not found".to_string()));
        };

        let domain = entity_found?;

        let biography = match payload.biography {
            Some(biography) => Some(biography),
            None => domain.biography,
        };

        let relationship = match payload.relationship {
            Some(relationship) => Some(relationship),
            None => domain.relationship,
        };

        let job = match payload.job {
            Some(job) => Some(job),
            None => domain.job,
        };

        let home_town = match payload.home_town {
            Some(home_town) => Some(home_town),
            None => domain.home_town,
        };

        let current_city = match payload.current_city {
            Some(current_city) => Some(current_city),
            None => domain.current_city,
        };

        let weight = match payload.weight {
            Some(weight) => Some(weight),
            None => domain.weight,
        };

        let height = match payload.height {
            Some(height) => Some(height),
            None => domain.height,
        };

        let person_info = PersonInfo {
            id: Some(id),
            person_id: payload.person_id,
            biography,
            relationship,
            job,
            home_town,
            current_city,
            weight,
            height,
            uuid: None,
            created_at: None,
            updated_at: None,
        };
        let entity_persisted = PersonInfoGateway::update(db, person_info).await;

        if entity_persisted.is_err() {
            let error = entity_persisted.err().unwrap();
            log::info!("Error updating person info: {:?}", error.to_string());
            return Err(BusinessError::new(error.to_string()));
        };
        let person_info_persisted = entity_persisted.unwrap();
        log::info!("Person info updated successfully");
        Ok(PersonInfoEntityMapper::from_model(person_info_persisted))
    }

    pub async fn add(
        db: &sea_orm::DbConn,
        person_info: PersonInfo,
    ) -> Result<PersonInfo, BusinessError> {
        log::info!("Adding person info for person id: {:?}", person_info.id);
        let result = PersonInfoGateway::persist(db, person_info).await;

        if result.is_err() {
            return Err(BusinessError::new("Failed to add person info".to_string()));
        }

        Ok(PersonInfoEntityMapper::from_active_model(result.unwrap()))
    }
}
