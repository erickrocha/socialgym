use entity::country::{ActiveModel, Model};
use crate::commons::entity_mapper::EntityMapper;

#[derive(Clone, Debug)]
pub struct Country {
	pub id: i32,
	pub name: String,
	pub acronym: String,
	pub currency: String,
}

pub struct CountryEntityMapper;
impl EntityMapper<Country,Model, ActiveModel> for CountryEntityMapper {
	fn build_active_model(d: Country) -> ActiveModel {
		ActiveModel {
			id: sea_orm::Set(d.id),
			name: sea_orm::Set(d.name),
			acronym: sea_orm::Set(d.acronym),
			currency: sea_orm::Set(d.currency),
		}
	}

	fn from_model(e: Model) -> Country {
		Country {
			id: e.id,
			name: e.name,
			acronym: e.acronym,
			currency: e.currency,
		}
	}

	fn from_active_model(e: ActiveModel) -> Country {
		Country {
			id: e.id.unwrap(),
			name: e.name.unwrap(),
			acronym: e.acronym.unwrap(),
			currency: e.currency.unwrap(),
		}
	}
}