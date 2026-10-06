use crate::commons::authorization::ensure_owns;
use crate::gateway::consent_gateway::HealthConsentGatewayPort;
use crate::gateway::evolution_check_in_gateway::EvolutionCheckInGatewayPort;
use domain::business_error::BusinessError;
use domain::evolution_check_in::EvolutionCheckIn;
use mongodb::bson::DateTime;

pub struct EvolutionCheckInUseCase<G, C> {
    gateway: G,
    consent_gateway: C,
}

#[cfg(test)]
#[path = "../tests/evolution_check_in_use_case_test.rs"]
mod tests;

impl<G, C> EvolutionCheckInUseCase<G, C>
where
    G: EvolutionCheckInGatewayPort,
    C: HealthConsentGatewayPort,
{
    pub fn new(gateway: G, consent_gateway: C) -> Self {
        Self {
            gateway,
            consent_gateway,
        }
    }

    /// Record a check-in for `acting_person_uuid`. A `personUuid` in the request
    /// body is ignored: the check-in always belongs to the caller.
    pub async fn add(
        &self,
        mut evolution: EvolutionCheckIn,
        acting_person_uuid: &str,
    ) -> Result<EvolutionCheckIn, BusinessError> {
        self.consent_gateway.require_health_consent().await?;
        evolution.person_uuid = acting_person_uuid.to_string();
        // Ids are server-generated so a client cannot overwrite or probe another check-in.
        evolution.uuid = uuid::Uuid::new_v4().to_string();
        log::info!("Adding evolution check-in: {:?}", evolution);
        self.gateway
            .persist_check_in(evolution)
            .await
            .map_err(|error| {
                log::error!("Error adding evolution check-in: {error:?}");
                BusinessError::new("Failed to add evolution check-in".to_string())
            })
    }

    pub async fn find(
        &self,
        id: String,
        acting_person_uuid: &str,
    ) -> Result<EvolutionCheckIn, BusinessError> {
        log::info!("Finding evolution check-in: {:?}", id);
        let check_in = self
            .gateway
            .find_check_in(id.clone())
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
