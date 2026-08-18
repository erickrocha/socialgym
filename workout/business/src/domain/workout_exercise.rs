use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::uuid_to_string;
use chrono::NaiveDateTime;
use entity::workout_exercise::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct WorkoutExercise {
    pub id: Option<i32>,
    pub workout_id: i32,
    pub exercise_id: i32,
    pub order_index: i32,
    pub uuid: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct WorkoutExerciseEntityMapper {}

impl EntityMapper<WorkoutExercise, Model, ActiveModel> for WorkoutExerciseEntityMapper {
    fn build_active_model(d: WorkoutExercise) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            uuid: NotSet,
            workout_id: Set(d.workout_id),
            exercise_id: Set(d.exercise_id),
            order_index: Set(d.order_index),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: Model) -> WorkoutExercise {
        WorkoutExercise {
            id: Some(e.id),
            workout_id: e.workout_id,
            exercise_id: e.exercise_id,
            order_index: e.order_index,
            uuid: Some(uuid_to_string(e.uuid)),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> WorkoutExercise {
        WorkoutExercise {
            id: Some(e.id.unwrap()),
            workout_id: e.workout_id.unwrap(),
            exercise_id: e.exercise_id.unwrap(),
            order_index: e.order_index.unwrap(),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl WorkoutExercise {
    pub fn new(workout_id: i32, exercise_id: i32, order_index: i32) -> WorkoutExercise {
        Self::update(None, workout_id, exercise_id, order_index)
    }

    pub fn update(
        id: Option<i32>,
        workout_id: i32,
        exercise_id: i32,
        order_index: i32,
    ) -> WorkoutExercise {
        WorkoutExercise {
            id,
            workout_id,
            exercise_id,
            order_index,
            uuid: None,
            created_at: None,
            updated_at: None,
        }
    }
}
