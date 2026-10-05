use crate::domain::business_error::BusinessError;
use crate::domain::country::Country;
use crate::gateway::country_gateway::CountryGatewayPort;

pub struct ResourceUseCase<G> {
    gateway: G,
}

impl<G: CountryGatewayPort> ResourceUseCase<G> {
    pub fn new(gateway: G) -> Self {
        Self { gateway }
    }

    pub async fn get_countries(&self) -> Result<Vec<Country>, BusinessError> {
        log::info!("Getting countries");
        self.gateway.find_all().await
    }
}
