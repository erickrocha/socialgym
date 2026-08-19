use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::parse_uuid;
use crate::domain::workout::{Workout, WorkoutEntityMapper};
use entity::prelude::WorkoutEntity as WorkoutQuery;
use entity::workout_entity as workout;
use entity::workout_entity::Entity;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter,
};

pub struct WorkoutGateway {}

impl WorkoutGateway {
    pub async fn persist(db: &DbConn, entity: Workout) -> Result<workout::ActiveModel, DbErr> {
        let active_model = WorkoutEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<workout::WorkoutEntity>, DbErr> {
        WorkoutQuery::find()
            .filter(workout::Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: String) -> Result<Option<workout::WorkoutEntity>, DbErr> {
        let uuid = parse_uuid(&uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        WorkoutQuery::find()
            .filter(workout::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }

    pub async fn find_by_owner_id(db: &DbConn, id: i32) -> Result<Vec<workout::WorkoutEntity>, DbErr> {
        WorkoutQuery::find()
            .filter(workout::Column::OwnerId.eq(id))
            .all(db)
            .await
    }

    pub async fn find_by_owner_uuid(db: &DbConn, uuid: String) -> Result<Vec<workout::WorkoutEntity>, DbErr> {
        let uuid = parse_uuid(&uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        WorkoutQuery::find()
            .filter(workout::Column::OwnerUuid.eq(uuid))
            .all(db)
            .await
    }

    pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Entity::delete_by_id(id).exec(db).await
    }

    pub async fn delete_by_uuid(db: &DbConn, uuid: String) -> Result<DeleteResult, DbErr> {
        let uuid = parse_uuid(&uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        Entity::delete_many()
            .filter(workout::Column::Uuid.eq(uuid))
            .exec(db)
            .await
    }
}
