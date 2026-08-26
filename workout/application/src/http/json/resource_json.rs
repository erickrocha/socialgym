use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::http::json::country_json::CountryJson;
use crate::http::json::province_json::ProvinceJson;
use crate::http::json::settings_json::SettingsJson;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceJson {
	pub countries: Vec<CountryJson>,
	pub settings: Option<SettingsJson>,
	pub provinces: Vec<ProvinceJson>,
}

impl ResourceJson {
	pub fn builder() -> ResourceJsonBuilder {
		ResourceJsonBuilder::default()
	}
}

#[derive(Default)]
pub struct ResourceJsonBuilder {
	countries: Vec<CountryJson>,
	settings: Option<SettingsJson>,
	provinces: Vec<ProvinceJson>,
}

impl ResourceJsonBuilder {
	pub fn countries(mut self, countries: Vec<CountryJson>) -> Self {
		self.countries = countries;
		self
	}

	pub fn settings(mut self, settings: Option<SettingsJson>) -> Self {
		self.settings = settings;
		self
	}

	pub fn provinces(mut self, provinces: Vec<ProvinceJson>) -> Self {
		self.provinces = provinces;
		self
	}

	pub fn build(self) -> ResourceJson {
		ResourceJson {
			countries: self.countries,
			settings: self.settings,
			provinces: self.provinces,
		}
	}
}