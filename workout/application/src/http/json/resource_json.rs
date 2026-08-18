use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::http::json::country_json::CountryJson;
use crate::http::json::settings_json::SettingsJson;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceJson {
	pub countries: Vec<CountryJson>,
	pub settings: Option<SettingsJson>,
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

	pub fn build(self) -> ResourceJson {
		ResourceJson {
			countries: self.countries,
			settings: self.settings,
		}
	}
}