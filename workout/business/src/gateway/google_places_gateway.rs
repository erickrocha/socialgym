use crate::domain::address_candidate::AddressCandidate;
use crate::domain::business_error::BusinessError;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

const GOOGLE_MAPS_API_KEY: &str = "GOOGLE_MAPS_API_KEY";
const GOOGLE_MAPS_ENABLED: &str = "GOOGLE_MAPS_ENABLED";
const SEARCH_TEXT_URL: &str = "https://places.googleapis.com/v1/places:searchText";
const FIELD_MASK: &str = "places.id,places.formattedAddress,places.location,places.addressComponents";
const BIAS_RADIUS_METERS: f64 = 20000.0;
const PAGE_SIZE: i32 = 5;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
	HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

/// Whether the Google Maps-backed address search feature is turned on.
/// Defaults to disabled: unset, empty, or any value other than "true" means
/// the feature stays off and `GOOGLE_MAPS_API_KEY` is never read.
pub fn is_enabled() -> bool {
	env::var(GOOGLE_MAPS_ENABLED).ok().and_then(|s| s.parse().ok()).unwrap_or(false)
}

#[derive(Serialize)]
struct LatLng {
	latitude: f64,
	longitude: f64,
}

#[derive(Serialize)]
struct Circle {
	center: LatLng,
	radius: f64,
}

#[derive(Serialize)]
struct LocationBias {
	circle: Circle,
}

#[derive(Serialize)]
struct SearchTextRequest {
	#[serde(rename = "textQuery")]
	text_query: String,
	#[serde(rename = "locationBias", skip_serializing_if = "Option::is_none")]
	location_bias: Option<LocationBias>,
	#[serde(rename = "pageSize")]
	page_size: i32,
}

#[derive(Deserialize)]
struct SearchTextResponse {
	#[serde(default)]
	places: Vec<GooglePlace>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GooglePlace {
	id: String,
	#[serde(default)]
	formatted_address: String,
	#[serde(default)]
	location: Option<GoogleLatLng>,
	#[serde(default)]
	address_components: Vec<GoogleAddressComponent>,
}

#[derive(Deserialize)]
struct GoogleLatLng {
	latitude: f64,
	longitude: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleAddressComponent {
	long_text: String,
	short_text: String,
	types: Vec<String>,
}

fn component<'a>(components: &'a [GoogleAddressComponent], component_type: &str) -> Option<&'a GoogleAddressComponent> {
	components.iter().find(|c| c.types.iter().any(|t| t == component_type))
}

fn map_place(place: GooglePlace) -> AddressCandidate {
	let street_number = component(&place.address_components, "street_number").map(|c| c.long_text.as_str());
	let route = component(&place.address_components, "route").map(|c| c.long_text.as_str());
	let address_line1 = match (street_number, route) {
		(Some(number), Some(street)) => format!("{} {}", street, number),
		(None, Some(street)) => street.to_string(),
		(Some(number), None) => number.to_string(),
		(None, None) => place.formatted_address.clone(),
	};
	let address_line2 = component(&place.address_components, "subpremise").map(|c| c.long_text.clone());
	let locality = component(&place.address_components, "locality")
		.map(|c| c.long_text.clone())
		.unwrap_or_default();
	let (administrative_area, administrative_area_code) =
		match component(&place.address_components, "administrative_area_level_1") {
			Some(c) => (c.long_text.clone(), c.short_text.clone()),
			None => (String::new(), String::new()),
		};
	let postal_code = component(&place.address_components, "postal_code").map(|c| c.long_text.clone());
	let country_code = component(&place.address_components, "country")
		.map(|c| c.short_text.clone())
		.unwrap_or_default();
	let location = place.location.unwrap_or(GoogleLatLng { latitude: 0.0, longitude: 0.0 });

	AddressCandidate {
		place_id: place.id,
		formatted_address: place.formatted_address,
		address_line1,
		address_line2,
		locality,
		administrative_area,
		administrative_area_code,
		postal_code,
		country_code,
		latitude: location.latitude,
		longitude: location.longitude,
	}
}

pub struct GooglePlacesGateway {}

impl GooglePlacesGateway {
	pub async fn search_text(query: &str, bias: Option<(f64, f64)>) -> Result<Vec<AddressCandidate>, BusinessError> {
		let api_key = env::var(GOOGLE_MAPS_API_KEY).expect("GOOGLE_MAPS_API_KEY must be set");

		let request_body = SearchTextRequest {
			text_query: query.to_string(),
			location_bias: bias.map(|(latitude, longitude)| LocationBias {
				circle: Circle {
					center: LatLng { latitude, longitude },
					radius: BIAS_RADIUS_METERS,
				},
			}),
			page_size: PAGE_SIZE,
		};

		let response = http_client()
			.post(SEARCH_TEXT_URL)
			.header("X-Goog-Api-Key", api_key)
			.header("X-Goog-FieldMask", FIELD_MASK)
			.json(&request_body)
			.send()
			.await;

		let response = match response {
			Ok(response) => response,
			Err(error) => {
				log::error!("Error calling Google Places API: {}", error);
				return Err(BusinessError::infrastructure("Failed to search address".to_string()));
			}
		};

		if !response.status().is_success() {
			let status = response.status();
			let body = response.text().await.unwrap_or_default();
			log::error!("Google Places API returned {}: {}", status, body);
			return Err(BusinessError::infrastructure("Failed to search address".to_string()));
		}

		match response.json::<SearchTextResponse>().await {
			Ok(parsed) => Ok(parsed.places.into_iter().map(map_place).collect()),
			Err(error) => {
				log::error!("Error parsing Google Places API response: {}", error);
				Err(BusinessError::infrastructure("Failed to search address".to_string()))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::sync::Mutex;

	// Guards GOOGLE_MAPS_ENABLED so the enabled-flag tests (which mutate
	// process-wide env state) don't race with each other under `cargo test`.
	static ENV_LOCK: Mutex<()> = Mutex::new(());

	#[test]
	fn is_enabled_defaults_to_false_when_unset() {
		let _guard = ENV_LOCK.lock().unwrap();
		unsafe { env::remove_var(GOOGLE_MAPS_ENABLED) };
		assert!(!is_enabled());
	}

	#[test]
	fn is_enabled_true_when_set_to_true() {
		let _guard = ENV_LOCK.lock().unwrap();
		unsafe { env::set_var(GOOGLE_MAPS_ENABLED, "true") };
		assert!(is_enabled());
		unsafe { env::remove_var(GOOGLE_MAPS_ENABLED) };
	}

	#[test]
	fn is_enabled_false_for_garbage_value() {
		let _guard = ENV_LOCK.lock().unwrap();
		unsafe { env::set_var(GOOGLE_MAPS_ENABLED, "yes-please") };
		assert!(!is_enabled());
		unsafe { env::remove_var(GOOGLE_MAPS_ENABLED) };
	}

	const SAMPLE_RESPONSE: &str = r#"{
		"places": [
			{
				"id": "ChIJ_abc123",
				"formattedAddress": "1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA",
				"location": { "latitude": 37.4224764, "longitude": -122.0842499 },
				"addressComponents": [
					{ "longText": "1600", "shortText": "1600", "types": ["street_number"] },
					{ "longText": "Amphitheatre Parkway", "shortText": "Amphitheatre Pkwy", "types": ["route"] },
					{ "longText": "Mountain View", "shortText": "Mountain View", "types": ["locality", "political"] },
					{ "longText": "California", "shortText": "CA", "types": ["administrative_area_level_1", "political"] },
					{ "longText": "94043", "shortText": "94043", "types": ["postal_code"] },
					{ "longText": "United States", "shortText": "US", "types": ["country", "political"] }
				]
			}
		]
	}"#;

	#[test]
	fn parses_google_response_and_maps_full_address() {
		let parsed: SearchTextResponse = serde_json::from_str(SAMPLE_RESPONSE).unwrap();
		let candidates: Vec<AddressCandidate> = parsed.places.into_iter().map(map_place).collect();

		assert_eq!(candidates.len(), 1);
		let candidate = &candidates[0];
		assert_eq!(candidate.place_id, "ChIJ_abc123");
		assert_eq!(candidate.address_line1, "Amphitheatre Parkway 1600");
		assert_eq!(candidate.locality, "Mountain View");
		assert_eq!(candidate.administrative_area, "California");
		assert_eq!(candidate.administrative_area_code, "CA");
		assert_eq!(candidate.postal_code, Some("94043".to_string()));
		assert_eq!(candidate.country_code, "US");
		assert!((candidate.latitude - 37.4224764).abs() < f64::EPSILON);
		assert!((candidate.longitude - (-122.0842499)).abs() < f64::EPSILON);
		assert_eq!(candidate.address_line2, None);
	}

	#[test]
	fn falls_back_to_formatted_address_when_no_street_components() {
		let response = r#"{
			"places": [
				{
					"id": "ChIJ_no_street",
					"formattedAddress": "Some Region, Some Country",
					"location": { "latitude": 1.0, "longitude": 2.0 },
					"addressComponents": []
				}
			]
		}"#;
		let parsed: SearchTextResponse = serde_json::from_str(response).unwrap();
		let candidates: Vec<AddressCandidate> = parsed.places.into_iter().map(map_place).collect();

		assert_eq!(candidates[0].address_line1, "Some Region, Some Country");
		assert_eq!(candidates[0].locality, "");
		assert_eq!(candidates[0].country_code, "");
		assert_eq!(candidates[0].postal_code, None);
	}

	#[test]
	fn location_bias_serializes_with_google_field_names() {
		let request = SearchTextRequest {
			text_query: "1600 Amphitheatre".to_string(),
			location_bias: Some(LocationBias {
				circle: Circle {
					center: LatLng { latitude: 37.4, longitude: -122.0 },
					radius: BIAS_RADIUS_METERS,
				},
			}),
			page_size: PAGE_SIZE,
		};
		let json = serde_json::to_value(&request).unwrap();
		assert_eq!(json["textQuery"], "1600 Amphitheatre");
		assert_eq!(json["pageSize"], PAGE_SIZE);
		assert_eq!(json["locationBias"]["circle"]["center"]["latitude"], 37.4);
		assert_eq!(json["locationBias"]["circle"]["radius"], BIAS_RADIUS_METERS);
	}

	#[test]
	fn location_bias_omitted_when_none() {
		let request = SearchTextRequest {
			text_query: "text".to_string(),
			location_bias: None,
			page_size: PAGE_SIZE,
		};
		let json = serde_json::to_value(&request).unwrap();
		assert!(json.get("locationBias").is_none());
	}
}
