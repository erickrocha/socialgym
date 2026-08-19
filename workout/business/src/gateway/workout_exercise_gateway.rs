use crate::commons::entity_mapper::EntityMapper;
use crate::domain::workout_exercise::{WorkoutExercise, WorkoutExerciseEntityMapper};
use entity::workout_exercise_entity as workout_exercise;
use entity::prelude::WorkoutExerciseEntity as WorkoutExerciseQuery;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter,
    QueryOrder,
};

pub struct WorkoutExerciseGateway {}

impl WorkoutExerciseGateway {
    pub async fn persist(
        db: &DbConn,
        entity: WorkoutExercise,
    ) -> Result<workout_exercise::ActiveModel, DbErr> {
        let active_model = WorkoutExerciseEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn persist_all(
        db: &DbConn,
        entities: Vec<WorkoutExercise>,
    ) -> Result<Vec<workout_exercise::ActiveModel>, DbErr> {
        let mut result = Vec::new();
        for entity in entities {
            let saved = Self::persist(db, entity).await?;
            result.push(saved);
        }
        Ok(result)
    }

    pub async fn find_by_workout_id(db: &DbConn,workout_id: i32) -> Vec<workout_exercise::WorkoutExerciseEntity> {
        WorkoutExerciseQuery::find()
            .filter(workout_exercise::Column::WorkoutId.eq(workout_id))
            .order_by_asc(workout_exercise::Column::OrderIndex)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn find_by_exercise_id(
        db: &DbConn,
        exercise_id: i32,
    ) -> Vec<workout_exercise::WorkoutExerciseEntity> {
        WorkoutExerciseQuery::find()
            .filter(workout_exercise::Column::ExerciseId.eq(exercise_id))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn delete_by_workout_id(
        db: &DbConn,
        workout_id: i32,
    ) -> Result<DeleteResult, DbErr> {
        workout_exercise::Entity::delete_many()
            .filter(workout_exercise::Column::WorkoutId.eq(workout_id))
            .exec(db)
            .await
    }

    pub async fn delete_by_workout_and_exercise(
        db: &DbConn,
        workout_id: i32,
        exercise_id: i32,
    ) -> Result<DeleteResult, DbErr> {
        workout_exercise::Entity::delete_many()
            .filter(workout_exercise::Column::WorkoutId.eq(workout_id))
            .filter(workout_exercise::Column::ExerciseId.eq(exercise_id))
            .exec(db)
            .await
    }
}
