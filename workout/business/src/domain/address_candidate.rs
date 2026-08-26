#[derive(Clone, Debug)]
pub struct AddressCandidate {
	pub place_id: String,
	pub formatted_address: String,
	pub address_line1: String,
	pub address_line2: Option<String>,
	pub locality: String,
	pub administrative_area: String,
	pub administrative_area_code: String,
	pub postal_code: Option<String>,
	pub country_code: String,
	pub latitude: f64,
	pub longitude: f64,
}
