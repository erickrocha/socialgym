use sea_orm::{
	ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr, DeleteResult, EntityTrait,
	QueryFilter,
};
use entity::profile_entity::{ActiveModel, Column, ProfileEntity};
use crate::domain::profile::{Profile, ProfileEntityMapper};
use entity::prelude::ProfileEntity as ProfileQuery;
use crate::commons::entity_mapper::EntityMapper;

pub struct ProfileGateway{}

impl ProfileGateway {
	pub async fn persist(
		db: &DbConn,
		entity: Profile,
	) -> Result<ActiveModel, DbErr> {
		let active_model = ProfileEntityMapper::build_active_model(entity);
		active_model.save(db).await
	}

	pub async fn find_by_person_id(db: &DbConn,person_id: i32) -> Vec<ProfileEntity> {
		ProfileQuery::find()
			.filter(Column::PersonId.eq(person_id))
			.all(db)
			.await
			.unwrap_or_else(|_| Vec::new())
	}

	/// Bulk-deletes every profile-mapping row for a person (account-purge cascade).
	pub async fn delete_all_by_person_id<C: ConnectionTrait>(db: &C, person_id: i32) -> Result<DeleteResult, DbErr> {
		ProfileQuery::delete_many()
			.filter(Column::PersonId.eq(person_id))
			.exec(db)
			.await
	}

	/// Bulk-deletes every profile-mapping row pointing at a business profile
	/// (account-purge cascade, when the business profile itself is being deleted).
	pub async fn delete_all_by_business_profile_id<C: ConnectionTrait>(db: &C, business_profile_id: i32) -> Result<DeleteResult, DbErr> {
		ProfileQuery::delete_many()
			.filter(Column::BusinessProfileId.eq(business_profile_id))
			.exec(db)
			.await
	}
}