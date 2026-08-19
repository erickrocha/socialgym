use entity::prelude::CountryEntity as CountryQuery;
use sea_orm::{DbConn, DbErr, EntityTrait};
pub struct CountryGateway {}

impl CountryGateway {
	pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<entity::country_entity::CountryEntity>, DbErr> {
		CountryQuery::find_by_id(id).one(db).await
	}

	pub async fn find_all(db: &DbConn) -> Result<Vec<entity::country_entity::CountryEntity>, DbErr> {
		CountryQuery::find().all(db).await
	}
}
