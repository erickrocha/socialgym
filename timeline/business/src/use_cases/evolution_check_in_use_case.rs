use crate::gateway::evolution_check_in_gateway::EvolutionCheckInGateway;
use crate::repositories::repository::Repository;
use domain::business_error::BusinessError;
use domain::evolution_check_in::EvolutionCheckIn;
use mongodb::bson::DateTime;

pub struct EvolutionCheckInUseCase {
    pub gateway: EvolutionCheckInGateway,
}

impl EvolutionCheckInUseCase {
    pub fn new(gateway: EvolutionCheckInGateway) -> Self {
        Self { gateway }
    }

    pub async fn add(
        &self,
        evolution: EvolutionCheckIn,
    ) -> Result<EvolutionCheckIn, BusinessError> {
        log::info!("Adding evolution check-in: {:?}", evolution);
        let persisted = self.gateway.persist(evolution).await;
        if persisted.is_err() {
            log::error!(
                "Error adding evolution check-in: {:?}",
                persisted.err().unwrap()
            );
            return Err(BusinessError::new(
                "Failed to add evolution check-in".to_string(),
            ));
        }
        persisted
    }

    pub async fn find(&self, id: String) -> Option<EvolutionCheckIn> {
        log::info!("Finding evolution check-in: {:?}", id);
        let result = self.gateway.find_by_id(id.clone()).await;
        if result.is_none() {
            log::warn!("Evolution check-in not found: {:?}", id);
        }
        result
    }

    pub async fn find_all_by_owner(&self,person_uuid: String,start: DateTime, end: DateTime) -> Vec<EvolutionCheckIn> {
        log::info!("Finding all evolution check-ins by person uuid: {:?} start: {:?} end: {:?}",person_uuid, start, end);
        self.gateway.find_all_by_person_uuid(person_uuid, start, end).await
    }
}
