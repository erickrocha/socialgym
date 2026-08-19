use sea_orm::entity::prelude::*;

pub type CountryEntity = Model;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "country")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub name: String,
	pub acronym: String,
	pub currency: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {}
