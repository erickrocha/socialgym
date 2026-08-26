use entity::prelude::ProvinceEntity as ProvinceQuery;
use sea_orm::{DbConn, DbErr, EntityTrait};
pub struct ProvinceGateway {}

impl ProvinceGateway {
	pub async fn find_all(db: &DbConn) -> Result<Vec<entity::province_entity::ProvinceEntity>, DbErr> {
		ProvinceQuery::find().all(db).await
	}
}
