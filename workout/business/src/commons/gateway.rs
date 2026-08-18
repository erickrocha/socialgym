use sea_orm::DbErr;
use sea_orm::prelude::async_trait::async_trait;

#[async_trait]
pub trait Gateway<D, M, AM> {
	async fn persist(&self, domain: D) -> Result<AM, DbErr>;
	async fn delete(&self, domain: D) -> Result<(), DbErr>;
	async fn find_by_id(&self, id: i32) -> Result<Option<M>, DbErr>;
	async fn find_by_uuid(&self, uuid: String) -> Result<Option<M>, DbErr>;
	async fn find_all(&self) -> Result<Vec<M>, DbErr>;
}