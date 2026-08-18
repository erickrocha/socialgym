use entity::prelude::Country as CountryQuery;
use sea_orm::{DbConn, DbErr, EntityTrait};
pub struct CountryGateway {}

impl CountryGateway {
	pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<entity::country::Model>, DbErr> {
		CountryQuery::find_by_id(id).one(db).await
	}

	pub async fn find_all(db: &DbConn) -> Result<Vec<entity::country::Model>, DbErr> {
		CountryQuery::find().all(db).await
	}
}
