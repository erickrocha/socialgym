use crate::domain::business_error::BusinessError;
use crate::domain::country::{Country, CountryEntityMapper};
use crate::gateway::country_gateway::CountryGateway;
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

}


