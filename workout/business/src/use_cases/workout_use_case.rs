use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::exercise::ExerciseEntityMapper;
use crate::domain::workout::{Workout, WorkoutEntityMapper};
use crate::gateway::exercise_gateway::ExerciseGateway;
use crate::gateway::workout_exercise_gateway::WorkoutExerciseGateway;
use crate::gateway::workout_gateway::WorkoutGateway;
use crate::use_cases::exercise_use_case::ExerciseUseCase;
use sea_orm::DbConn;

pub struct WorkoutUseCase {}

impl WorkoutUseCase {
    pub async fn persist(db: &DbConn, workout: Workout) -> Result<Workout, BusinessError> {
        log::info!("Adding workout: {:?}", workout);

        let exercises = workout.exercises.clone();
        let domain = WorkoutGateway::persist(db, workout).await;

        if domain.is_err() {
            log::error!("Error adding workout: {}", domain.as_ref().err().unwrap());
            return Err(BusinessError::new("Error adding workout".to_string()));
        }

        let model = domain.unwrap();
        let workout_id = model.id.clone().unwrap();

        let exercise_result = ExerciseUseCase::add_all_to_workout(db, workout_id, exercises).await;

        if exercise_result.is_err() {
            log::error!("Error adding exercise: {}", exercise_result.err().unwrap());
            return Err(BusinessError::new("Error adding exercise".to_string()));
        }

        Ok(WorkoutEntityMapper::from_active_model(model))
    }

    pub async fn get(db: &DbConn, id: i32) -> Option<Workout> {
        log::info!("Getting workout for id: {}", id);

        let domain = WorkoutGateway::find_by_id(db, id).await;

        if domain.is_err() {
            log::error!("Error getting workout: {}", domain.as_ref().err().unwrap());
            return None;
        }

        let model_option = domain.unwrap();

        if let Some(model) = model_option {
            let mut workout = WorkoutEntityMapper::from_model(model);
            Self::fill_exercise(db, &mut workout).await;
            Some(workout)
        } else {
            None
        }
    }

    pub async fn get_by_uuid(db: &DbConn, uuid: String) -> Option<Workout> {
        log::info!("Getting workout for uuid: {}", uuid);

        let domain = WorkoutGateway::find_by_uuid(db, uuid).await;

        if domain.is_err() {
            log::error!("Error getting workout: {}", domain.as_ref().err().unwrap());
            return None;
        }

        let model_option = domain.unwrap();

        if let Some(model) = model_option {
            let mut workout = WorkoutEntityMapper::from_model(model);
            Self::fill_exercise(db, &mut workout).await;
            Some(workout)
        } else {
            None
        }
    }

    pub async fn find_all_by_owner_id(db: &DbConn, owner_id: i32) -> Vec<Workout> {
        log::info!("Finding workouts for owner_id: {}", owner_id);

        let domain = WorkoutGateway::find_by_owner_id(db, owner_id).await;

        if domain.is_err() {
            log::error!("Error finding workouts: {}", domain.as_ref().err().unwrap());
            return Vec::new();
        }

        let workouts = WorkoutEntityMapper::from_models(domain.unwrap());
        Self::fill_exercises(db, workouts).await
    }

    pub async fn find_all_by_owner_uuid(db: &DbConn, owner_uuid: String) -> Vec<Workout> {
        log::info!("Finding workouts for owner_uuid: {}", owner_uuid);

        let domain = WorkoutGateway::find_by_owner_uuid(db, owner_uuid).await;

        if domain.is_err() {
            log::error!("Error finding workouts: {}", domain.as_ref().err().unwrap());
            return Vec::new();
        }

        let workouts = WorkoutEntityMapper::from_models(domain.unwrap());
        Self::fill_exercises(db, workouts).await
    }

    pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<(), BusinessError> {
        let result = WorkoutGateway::delete_by_id(db, id).await;
        if result.is_err() {
            return Err(BusinessError::new("Failed to delete workout".into()));
        }
        Ok(())
    }

    pub async fn delete_by_uuid(db: &DbConn, uuid: String) -> Result<(), BusinessError> {
        let result = WorkoutGateway::delete_by_uuid(db, uuid).await;
        if result.is_err() {
            return Err(BusinessError::new("Failed to delete workout".into()));
        }
        Ok(())
    }

    async fn fill_exercises(db: &DbConn, mut workouts: Vec<Workout>) -> Vec<Workout> {
        for workout in workouts.iter_mut() {
            Self::fill_exercise(db, workout).await;
        }
        workouts
    }

    async fn fill_exercise(db: &DbConn, workout: &mut Workout) {
        let workout_exercises =
            WorkoutExerciseGateway::find_by_workout_id(db, workout.id.unwrap()).await;
        let exercise_ids: Vec<i32> = workout_exercises.iter().map(|we| we.exercise_id).collect();
        let exercises = if exercise_ids.is_empty() {
            Vec::new()
        } else {
            let exercises = ExerciseGateway::find_by_ids(db, exercise_ids).await;
            ExerciseEntityMapper::from_models(exercises)
        };
        workout.exercises = exercises;
    }
}
