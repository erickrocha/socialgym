use crate::commons::authorization::ensure_owns;
use crate::gateway::consent_gateway::ConsentGateway;
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

    /// Record a check-in for `acting_person_uuid`. A `personUuid` in the request
    /// body is ignored: the check-in always belongs to the caller.
    pub async fn add(
        &self,
        mut evolution: EvolutionCheckIn,
        acting_person_uuid: &str,
    ) -> Result<EvolutionCheckIn, BusinessError> {
        ConsentGateway::require_health_consent().await?;
        evolution.person_uuid = acting_person_uuid.to_string();
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

    pub async fn find(
        &self,
        id: String,
        acting_person_uuid: &str,
    ) -> Result<EvolutionCheckIn, BusinessError> {
        log::info!("Finding evolution check-in: {:?}", id);
        let check_in = self
            .gateway
            .find_by_id(id.clone())
            .await
            .ok_or_else(|| BusinessError::not_found("Evolution check-in not found"))?;
        ensure_owns(&check_in.person_uuid, acting_person_uuid)?;
        Ok(check_in)
    }

    pub async fn find_all_by_owner(
        &self,
        person_uuid: String,
        start: DateTime,
        end: DateTime,
    ) -> Vec<EvolutionCheckIn> {
        log::info!(
            "Finding all evolution check-ins by person uuid: {:?} start: {:?} end: {:?}",
            person_uuid,
            start,
            end
        );
        self.gateway
            .find_all_by_person_uuid(person_uuid, start, end)
            .await
    }
}
