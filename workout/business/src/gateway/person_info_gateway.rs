use crate::commons::entity_mapper::EntityMapper;
use crate::domain::person_info::{PersonInfo, PersonInfoEntityMapper};
use entity::person_info_entity as person_info;
use entity::person_info_entity::PersonInfoEntity;
use entity::prelude::PersonInfoEntity as PersonInfoQuery;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

pub struct PersonInfoGateway {}

impl PersonInfoGateway {
    pub async fn persist(
        db: &DbConn,
        entity: PersonInfo,
    ) -> Result<person_info::ActiveModel, DbErr> {
        let active_model = PersonInfoEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn update(db: &DbConn, entity: PersonInfo) -> Result<PersonInfoEntity, DbErr> {
        let active_model = PersonInfoEntityMapper::build_active_model(entity);
        active_model.update(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<PersonInfoEntity>, DbErr> {
        PersonInfoQuery::find()
            .filter(person_info::Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn find_by_person_id(
        db: &DbConn,
        person_id: i32,
    ) -> Result<Option<PersonInfoEntity>, DbErr> {
        PersonInfoQuery::find()
            .filter(person_info::Column::PersonId.eq(person_id))
            .one(db)
            .await
    }
}
