use sea_orm::{NotSet, Set};
use entity::country_entity::{ActiveModel, CountryEntity};
use crate::commons::entity_mapper::EntityMapper;

#[derive(Clone, Debug)]
pub struct Country {
	pub id: Option<i32>,
	pub ddi: String,
	pub name: String,
	pub acronym: String,
	pub currency: String,
}

pub struct CountryEntityMapper;
impl EntityMapper<Country,CountryEntity, ActiveModel> for CountryEntityMapper {
	fn build_active_model(d: Country) -> ActiveModel {
		ActiveModel {
			id: match d.id {
				Some(id) => Set(id),
				None => NotSet
			},
			ddi: Set(d.ddi),
			name: Set(d.name),
			acronym: Set(d.acronym),
			currency: Set(d.currency),
		}
	}

	fn from_model(e: CountryEntity) -> Country {
		Country {
			id: Some(e.id),
			ddi: e.ddi,
			name: e.name,
			acronym: e.acronym,
			currency: e.currency,
		}
	}

	fn from_active_model(e: ActiveModel) -> Country {
		Country {
			id: Some(e.id.unwrap()),
			name: e.name.unwrap(),
			ddi: e.ddi.unwrap(),
			acronym: e.acronym.unwrap(),
			currency: e.currency.unwrap(),
		}
	}
}