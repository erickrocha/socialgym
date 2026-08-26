use sea_orm::entity::prelude::*;

pub type ProvinceEntity = Model;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "province")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub name: String,
	pub acronym: String,
	pub country_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::country_entity::Entity",
		from = "Column::CountryId",
		to = "super::country_entity::Column::Id"
	)]
	Country,
}

impl Related<super::country_entity::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Country.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
