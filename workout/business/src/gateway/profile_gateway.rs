use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};
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
}