use crate::commons::entity_mapper::EntityMapper;
use crate::domain::workout::{Workout, WorkoutEntityMapper};
use entity::prelude::Workout as WorkoutQuery;
use entity::workout;
use entity::workout::Entity;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter,
};
use uuid::Uuid;

pub struct WorkoutGateway {}

impl WorkoutGateway {
    pub async fn persist(db: &DbConn, entity: Workout) -> Result<workout::ActiveModel, DbErr> {
        let active_model = WorkoutEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<workout::Model>, DbErr> {
        WorkoutQuery::find()
            .filter(workout::Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: String) -> Result<Option<workout::Model>, DbErr> {
        WorkoutQuery::find()
            .filter(workout::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }

    pub async fn find_by_owner_id(db: &DbConn, id: i32) -> Result<Vec<workout::Model>, DbErr> {
        WorkoutQuery::find()
            .filter(workout::Column::OwnerId.eq(id))
            .all(db)
            .await
    }

    pub async fn find_by_owner_uuid(db: &DbConn, uuid: Uuid) -> Result<Vec<workout::Model>, DbErr> {
        WorkoutQuery::find()
            .filter(workout::Column::OwnerUuid.eq(uuid))
            .all(db)
            .await
    }

    pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Entity::delete_by_id(id).exec(db).await
    }

    pub async fn delete_by_uuid(db: &DbConn, uuid: String) -> Result<DeleteResult, DbErr> {
        Entity::delete_many()
            .filter(workout::Column::Uuid.eq(uuid))
            .exec(db)
            .await
    }
}
