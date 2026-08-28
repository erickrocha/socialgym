use crate::commons::authorization::ensure_owns;
use crate::commons::entity_mapper::EntityMapper;
use crate::commons::gateway::Gateway;
use crate::domain::business_error::BusinessError;
use crate::domain::settings::{Settings, SettingsEntityMapper};
use crate::domain::user::User;
use crate::gateway::settings_gateway::SettingsGateway;

pub struct SettingsUseCase {
    gateway: SettingsGateway,
}

impl SettingsUseCase {
    pub fn new(gateway: SettingsGateway) -> Self {
        Self { gateway }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Settings, BusinessError> {
        let result = self.gateway.find_by_id(id).await;
        if result.is_err() {
            log::error!(
                "Error fetching settings for Settings ID {}: {:?}",
                id,
                result.err()
            );
            return Err(BusinessError::new("Settings not found".to_string()));
        }

        let opt_settings = result.unwrap();
        match opt_settings {
            Some(settings) => Ok(SettingsEntityMapper::from_model(settings)),
            None => Err(BusinessError::new("Settings not found".to_string())),
        }
    }

    /// Signup-only path: creates the default settings row for a person that has
    /// just been created, before any user exists to authenticate as. Every other
    /// caller must go through [`Self::persist`], which enforces ownership.
    pub async fn bootstrap_for_person(&self, domain: Settings) -> Result<Settings, BusinessError> {
        log::info!("Bootstrapping settings for person_id={}", domain.person_id);
        let settings = self.gateway.persist(domain).await.map_err(|error| {
            log::error!("Error bootstrapping settings: {:?}", error);
            BusinessError::infrastructure("Error persisting settings")
        })?;
        Ok(SettingsEntityMapper::from_active_model(settings))
    }

    /// Create or update settings on behalf of `actor`. Settings belong to exactly
    /// one person, so the owner is always taken from `actor`, never the payload.
    pub async fn persist(&self, mut domain: Settings, actor: &User) -> Result<Settings, BusinessError> {
        log::info!("Persisting settings for person_id={}", actor.person_id);

        if let Some(id) = domain.id {
            let existing = self.get_by_id(id).await?;
            ensure_owns(existing.person_id, actor.person_id)?;
        }

        domain.person_id = actor.person_id;
        domain.person_uuid = actor.person_uuid.clone();

        let result = self.gateway.persist(domain).await;
        if result.is_err() {
            log::error!("Error persisting settings: {:?}", result.err());
            return Err(BusinessError::new("Error persisting settings".to_string()));
        }
        let settings = result.unwrap();
        log::info!("Settings persisted: {:?}", settings);
        Ok(SettingsEntityMapper::from_active_model(settings))
    }

    pub async fn get_by_uuid(&self, uuid: String) -> Result<Settings, BusinessError> {
        let result = self.gateway.find_by_uuid(uuid.clone()).await;
        if result.is_err() {
            log::error!(
                "Error fetching settings for UUID {}: {:?}",
                uuid,
                result.err()
            );
            return Err(BusinessError::new("Settings not found".to_string()));
        }

        let opt_settings = result.unwrap();
        match opt_settings {
            Some(settings) => Ok(SettingsEntityMapper::from_model(settings)),
            None => Err(BusinessError::new("Settings not found".to_string())),
        }
    }

    pub async fn get_by_owner_id(&self, owner_id: i32) -> Result<Settings, BusinessError> {
        log::info!("Getting settings for Owner ID: {}", owner_id);
        let result = self.gateway.find_by_owner_id(owner_id).await;
        if result.is_err() {
            log::error!(
                "Error fetching settings for Owner ID {}: {:?}",
                owner_id,
                result.err()
            );
            return Err(BusinessError::new("Settings not found".to_string()));
        }

        let opt_settings = result.unwrap();
        match opt_settings {
            Some(settings) => Ok(SettingsEntityMapper::from_model(settings)),
            None => Err(BusinessError::new("Settings not found".to_string())),
        }
    }

    pub async fn get_by_owner_uuid(&self, owner_uuid: String) -> Result<Settings, BusinessError> {
        log::info!(
            "[SettingsUseCase::get_by_owner_uuid] Executing for owner_uuid={}",
            owner_uuid
        );
        let settings = self
            .gateway
            .find_by_owner_ids(None, Some(owner_uuid.clone()))
            .await
            .map_err(|error| {
                log::error!(
                    "[SettingsUseCase::get_by_owner_uuid] Failed for owner_uuid={}: {}",
                    owner_uuid,
                    error
                );
                BusinessError::infrastructure("Error fetching settings")
            })?
            .ok_or_else(|| {
                let error = BusinessError::not_found("Settings not found");
                log::error!(
                    "[SettingsUseCase::get_by_owner_uuid] Failed for owner_uuid={}: {}",
                    owner_uuid,
                    error
                );
                error
            })?;
        Ok(SettingsEntityMapper::from_model(settings))
    }
}
