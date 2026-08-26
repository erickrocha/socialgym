use crate::domain::business_error::BusinessError;
use crate::domain::country::{Country, CountryEntityMapper};
use crate::domain::province::{Province, ProvinceEntityMapper};
use crate::gateway::country_gateway::CountryGateway;
use crate::gateway::province_gateway::ProvinceGateway;
use sea_orm::DbConn;
use crate::commons::entity_mapper::EntityMapper;

pub struct ResourceUseCase {}

impl ResourceUseCase {

	pub async fn get_countries(db: &DbConn) -> Result<Vec<Country>, BusinessError> {
		log::info!("Getting countries");
		let countries_result = CountryGateway::find_all(db).await;
		if countries_result.is_err() {
			log::error!("Error: {}", countries_result.err().unwrap());
			return Err(BusinessError::new("Failed to get countries".to_string()));
		}
		let countries_domain = CountryEntityMapper::from_models(countries_result.unwrap());
		Ok(countries_domain)
	}

	pub async fn get_provinces(db: &DbConn) -> Result<Vec<Province>, BusinessError> {
		log::info!("Getting provinces");
		let provinces_result = ProvinceGateway::find_all(db).await;
		if provinces_result.is_err() {
			log::error!("Error: {}", provinces_result.err().unwrap());
			return Err(BusinessError::new("Failed to get provinces".to_string()));
		}
		let provinces_domain = ProvinceEntityMapper::from_models(provinces_result.unwrap());
		Ok(provinces_domain)
	}

}


