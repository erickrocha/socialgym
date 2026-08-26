use sea_orm::{NotSet, Set};
use entity::province_entity::{ActiveModel, ProvinceEntity};
use crate::commons::entity_mapper::EntityMapper;

#[derive(Clone, Debug)]
pub struct Province {
	pub id: Option<i32>,
	pub name: String,
	pub acronym: String,
	pub country_id: i32,
}

pub struct ProvinceEntityMapper;
impl EntityMapper<Province,ProvinceEntity, ActiveModel> for ProvinceEntityMapper {
	fn build_active_model(d: Province) -> ActiveModel {
		ActiveModel {
			id: match d.id {
				Some(id) => Set(id),
				None => NotSet
			},
			name: Set(d.name),
			acronym: Set(d.acronym),
			country_id: Set(d.country_id),
		}
	}

	fn from_model(e: ProvinceEntity) -> Province {
		Province {
			id: Some(e.id),
			name: e.name,
			acronym: e.acronym,
			country_id: e.country_id,
		}
	}

	fn from_active_model(e: ActiveModel) -> Province {
		Province {
			id: Some(e.id.unwrap()),
			name: e.name.unwrap(),
			acronym: e.acronym.unwrap(),
			country_id: e.country_id.unwrap(),
		}
	}
}
