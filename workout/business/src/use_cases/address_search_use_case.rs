use crate::domain::address_candidate::AddressCandidate;
use crate::domain::business_error::BusinessError;
use crate::gateway::google_places_gateway;
use crate::gateway::google_places_gateway::GooglePlacesGateway;

pub struct AddressSearchUseCase {}

impl AddressSearchUseCase {
	pub async fn search(
		text: &str,
		latitude: Option<f64>,
		longitude: Option<f64>,
	) -> Result<Vec<AddressCandidate>, BusinessError> {
		if !google_places_gateway::is_enabled() {
			return Err(BusinessError::forbidden("Address search feature is disabled".to_string()));
		}

		if text.trim().is_empty() {
			return Err(BusinessError::validation("Search text must not be empty".to_string()));
		}

		let bias = match (latitude, longitude) {
			(Some(lat), Some(lng)) => Some((lat, lng)),
			_ => None,
		};

		// The typed text is part of a person's address: it is not logged.
		log::info!("Searching address");
		GooglePlacesGateway::search_text(text, bias).await
	}
}
