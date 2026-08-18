use crate::commons::entity_mapper::EntityMapper;
use crate::commons::gateway::Gateway;
use crate::domain::business_error::BusinessError;
use crate::domain::settings::{Settings, SettingsEntityMapper};
use crate::gateway::settings_gateway::SettingsGateway;

pub struct SettingsUseCase{
	gateway: SettingsGateway
}

impl SettingsUseCase {
	pub fn new(gateway: SettingsGateway) -> Self {
		Self { gateway }
	}

	pub async fn get_by_id(&self, id: i32) -> Result<Settings,BusinessError> {
		let result = self.gateway.find_by_id(id).await;
		if result.is_err() {
			log::error!("Error fetching settings for Settings ID {}: {:?}", id, result.err());
			return Err(BusinessError::new("Settings not found".to_string()));
		}

		let opt_settings = result.unwrap();
		match opt_settings {
			Some(settings) => Ok(SettingsEntityMapper::from_model(settings)),
			None => Err(BusinessError::new("Settings not found".to_string())),
		}
	}

	pub async fn persist(&self, domain: Settings) -> Result<Settings, BusinessError> {
		log::info!("Persisting settings: {:?}", domain);
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
			log::error!("Error fetching settings for UUID {}: {:?}", uuid, result.err());
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
			log::error!("Error fetching settings for Owner ID {}: {:?}", owner_id, result.err());
			return Err(BusinessError::new("Settings not found".to_string()));
		}

		let opt_settings = result.unwrap();
		match opt_settings {
			Some(settings) => Ok(SettingsEntityMapper::from_model(settings)),
			None => Err(BusinessError::new("Settings not found".to_string())),
		}
	}
}