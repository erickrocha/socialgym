use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::enums::Visibility;
use crate::domain::exercise::{Exercise, ExerciseEntityMapper};
use crate::domain::workout_exercise::{WorkoutExercise, WorkoutExerciseEntityMapper};
use crate::gateway::exercise_gateway::ExerciseGateway;
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::workout_exercise_gateway::WorkoutExerciseGateway;
use sea_orm::DbConn;

pub struct ExerciseUseCase {}

impl ExerciseUseCase {
    pub async fn add_all(
        db: &DbConn,
        exercises: Vec<Exercise>,
    ) -> Result<Vec<Exercise>, BusinessError> {
        log::info!("Adding exercises: {:?}", exercises);

        let mut added_exercises = Vec::new();

        for exercise in exercises {
            let domain = ExerciseGateway::persist(db, exercise.clone()).await;
            if domain.is_err() {
                log::error!("Error adding exercise: {}", domain.as_ref().err().unwrap());
                return Err(BusinessError::new("Error adding exercises".to_string()));
            }

            let model = domain.unwrap();
            added_exercises.push(ExerciseEntityMapper::from_active_model(model));
        }

        Ok(added_exercises)
    }

    pub async fn add_all_to_workout(
        db: &DbConn,
        workout_id: i32,
        exercises: Vec<Exercise>,
    ) -> Result<Vec<Exercise>, BusinessError> {
        log::info!(
            "Adding {} exercises to workout_id {}",
            exercises.len(),
            workout_id
        );

        if exercises.is_empty() {
            return Ok(Vec::new());
        }

        let mut resolved: Vec<Exercise> = Vec::with_capacity(exercises.len());
        for exercise in exercises {
            if exercise.id.is_some() {
                resolved.push(exercise);
            } else {
                let model = ExerciseGateway::persist(db, exercise).await.map_err(|e| {
                    log::error!("Error persisting new exercise: {}", e);
                    BusinessError::new("Error adding exercises".to_string())
                })?;
                resolved.push(ExerciseEntityMapper::from_active_model(model));
            }
        }

        let associations: Vec<WorkoutExercise> = resolved
            .iter()
            .enumerate()
            .map(|(order_index, exercise)| {
                WorkoutExercise::new(workout_id, exercise.id.unwrap(), order_index as i32)
            })
            .collect();

        WorkoutExerciseGateway::persist_all(db, associations)
            .await
            .map_err(|e| {
                log::error!("Error creating workout_exercise associations: {}", e);
                BusinessError::new("Error associating exercises with workout".to_string())
            })?;

        Ok(resolved)
    }

    pub async fn add_exercise_to_workout(
        db: &DbConn,
        workout_id: i32,
        exercise_id: i32,
        order_index: i32,
    ) -> Result<WorkoutExercise, BusinessError> {
        log::info!("Adding exercise {} to workout {}", exercise_id, workout_id);
        let workout_exercise = WorkoutExercise::new(workout_id, exercise_id, order_index);
        let result = WorkoutExerciseGateway::persist(db, workout_exercise).await;
        if result.is_err() {
            log::error!(
                "Error associating exercise with workout: {}",
                result.as_ref().err().unwrap()
            );
            return Err(BusinessError::new(
                "Error associating exercise with workout".to_string(),
            ));
        }
        Ok(WorkoutExerciseEntityMapper::from_active_model(
            result.unwrap(),
        ))
    }

    pub async fn remove_exercise_from_workout(
        db: &DbConn,
        workout_id: i32,
        exercise_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!(
            "Removing exercise {} from workout {}",
            exercise_id,
            workout_id
        );
        let result =
            WorkoutExerciseGateway::delete_by_workout_and_exercise(db, workout_id, exercise_id)
                .await;
        if result.is_err() {
            log::error!(
                "Error removing exercise from workout: {}",
                result.as_ref().err().unwrap()
            );
            return Err(BusinessError::new(
                "Error removing exercise from workout".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn persist(db: &DbConn, exercise: Exercise) -> Option<Exercise> {
        log::info!("Adding exercise: {:?}", exercise);
        let domain = ExerciseGateway::persist(db, exercise).await;
        if domain.is_err() {
            log::error!("Error adding exercise: {}", domain.as_ref().err().unwrap());
            return None;
        }
        let model = domain.unwrap();
        Some(ExerciseEntityMapper::from_active_model(model))
    }

    pub async fn get(db: &DbConn, exercise_id: i32) -> Option<Exercise> {
        log::info!("Getting exercise for exercise_id: {}", exercise_id);
        let domain = ExerciseGateway::find_by_id(db, exercise_id).await;
        if domain.is_err() {
            log::error!("Error getting exercise: {}", domain.as_ref().err().unwrap());
            return None;
        }
        domain
            .unwrap()
            .map(ExerciseEntityMapper::from_model)
    }

    pub async fn get_by_uuid(db: &DbConn, uuid: String) -> Option<Exercise> {
        log::info!("Getting exercise for uuid: {}", uuid);
        let domain = ExerciseGateway::find_by_uuid(db, uuid).await;
        if domain.is_err() {
            log::error!("Error getting exercise: {}", domain.as_ref().err().unwrap());
            return None;
        }
        domain
            .unwrap()
            .map(ExerciseEntityMapper::from_model)
    }

    pub async fn find_all_by_workout_id(
        db: &DbConn,
        workout_id: i32,
    ) -> Result<Vec<Exercise>, BusinessError> {
        log::info!("Finding exercises for workout_id: {}", workout_id);

        let workout_exercises = WorkoutExerciseGateway::find_by_workout_id(db, workout_id).await;
        let exercise_ids: Vec<i32> = workout_exercises.iter().map(|we| we.exercise_id).collect();

        if exercise_ids.is_empty() {
            return Ok(Vec::new());
        }

        let domain = ExerciseGateway::find_by_ids(db, exercise_ids).await;
        let exercises: Vec<Exercise> = ExerciseEntityMapper::from_models(domain);
        Ok(exercises)
    }

    pub async fn delete_by_id(db: &DbConn, exercise_id: i32) -> Result<(), BusinessError> {
        log::info!("Deleting exercise for exercise_id: {}", exercise_id);

        let delete_result = ExerciseGateway::delete_by_id(db, exercise_id)
            .await
            .map_err(|e| {
                log::error!("Error deleting exercise: {}", e);
                BusinessError::new("Error deleting exercise".to_string())
            })?;

        if delete_result.rows_affected == 0 {
            return Err(BusinessError::new("Exercise not found".to_string()));
        }
        Ok(())
    }

    pub async fn delete_by_uuid(db: &DbConn, uuid: String) -> Result<(), BusinessError> {
        log::info!("Deleting exercise for uuid: {}", uuid);

        let delete_result = ExerciseGateway::delete_by_uuid(db, uuid.clone())
            .await
            .map_err(|e| {
                log::error!("Error deleting exercise: {}", e);
                BusinessError::new("Error deleting exercise".to_string())
            })?;

        if delete_result.rows_affected == 0 {
            log::error!(
                "Error deleting exercise with uuid: {} Exercise not found",
                uuid
            );
            return Err(BusinessError::new("Exercise not found".to_string()));
        }

        Ok(())
    }

    pub async fn find_by_visibility(
        db: &DbConn,
        visibility: Visibility,
        current_user_person_id: i32,
    ) -> Result<Vec<Exercise>, BusinessError> {
        log::info!(
            "Finding exercises with visibility {:?} for person_id: {}",
            visibility,
            current_user_person_id
        );

        let models = match visibility {
            Visibility::Private => {
                let result = ExerciseGateway::find_by_owner_id(db, current_user_person_id).await;
                if result.is_err() {
                    log::error!(
                        "Error finding private exercises: {}",
                        result.as_ref().err().unwrap()
                    );
                    return Err(BusinessError::new("Error finding exercises".to_string()));
                }
                result.unwrap()
            }
            Visibility::Friends => {
                let friends_result =
                    FriendGateway::find_all_accepted_friends(db, current_user_person_id).await;
                if friends_result.is_err() {
                    log::error!(
                        "Error finding friends: {}",
                        friends_result.as_ref().err().unwrap()
                    );
                    return Err(BusinessError::new("Error finding friends".to_string()));
                }

                let friends = friends_result.unwrap();
                if friends.is_empty() {
                    return Ok(Vec::new());
                }

                let friend_ids: Vec<i32> = friends
                    .into_iter()
                    .map(|f| {
                        if f.person_id == current_user_person_id {
                            f.friend_id
                        } else {
                            f.person_id
                        }
                    })
                    .collect();

                let result = ExerciseGateway::find_by_owner_ids_and_visibility(
                    db,
                    friend_ids,
                    Visibility::Friends.to_string(),
                )
                .await;
                if result.is_err() {
                    log::error!(
                        "Error finding friends' exercises: {}",
                        result.as_ref().err().unwrap()
                    );
                    return Err(BusinessError::new("Error finding exercises".to_string()));
                }
                result.unwrap()
            }
            Visibility::Public => {
                let result =
                    ExerciseGateway::find_by_visibility(db, Visibility::Public.to_string()).await;
                if result.is_err() {
                    log::error!(
                        "Error finding public exercises: {}",
                        result.as_ref().err().unwrap()
                    );
                    return Err(BusinessError::new("Error finding exercises".to_string()));
                }
                result.unwrap()
            }
            _ => Vec::new(),
        };

        let exercises: Vec<Exercise> = ExerciseEntityMapper::from_models(models);
        log::info!("Found {} exercises", exercises.len());
        Ok(exercises)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_by_complex_filters_paginated(
        db: &DbConn,
        current_user_person_id: i32,
        public_owner_ids: Vec<i32>,
        category: Option<String>,
        visibility: Option<String>,
        page_number: u64,
        page_size: u64,
        sort_by: Option<String>,
    ) -> (Vec<Exercise>, i64, bool) {
        log::info!(
            "Finding exercises with complex filters: current_user_id={}, public_owner_ids={:?}, category={:?}, page_number={}, page_size={}",
            current_user_person_id,
            public_owner_ids,
            category,
            page_number,
            page_size
        );

        let page_index = page_number - 1;

        let friends = FriendGateway::find_all_accepted_friends(db, current_user_person_id)
            .await
            .unwrap_or_else(|_| Vec::new());

        let friend_ids: Vec<i32> = friends
            .into_iter()
            .map(|f| {
                if f.person_id == current_user_person_id {
                    f.friend_id
                } else {
                    f.person_id
                }
            })
            .collect();

        let result = ExerciseGateway::find_by_complex_filters_paginated(
            db,
            current_user_person_id,
            friend_ids,
            public_owner_ids,
            category,
            visibility,
            page_index,
            page_size,
            sort_by,
        )
        .await;

        if result.is_err() {
            log::error!(
                "Error finding exercises with complex filters: {}",
                result.as_ref().err().unwrap()
            );
            return (vec![], 0, false);
        }

        let (models, total_count) = result.unwrap();
        let exercises: Vec<Exercise> = ExerciseEntityMapper::from_models(models);
        let has_next_page = ((page_index + 1) * page_size) < total_count;
        let total_count_i64 = total_count as i64;

        log::info!(
            "Found {} exercises (total: {})",
            exercises.len(),
            total_count_i64
        );
        (exercises, total_count_i64, has_next_page)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_by_complex_filters_paginated_uuid(
        db: &DbConn,
        person_uuid: String,
        public_owner_uuids: Vec<String>,
        category: Option<String>,
        visibility: Option<String>,
        page_number: u64,
        page_size: u64,
        sort_by: Option<String>,
    ) -> (Vec<Exercise>, i64, bool) {
        let page_index = page_number - 1;

        let friends = FriendGateway::find_all_accepted_friends_by_uuid(db, person_uuid.clone())
            .await
            .unwrap_or_else(|_| Vec::new());

        let friend_uuids: Vec<String> = friends
            .into_iter()
            .map(|f| {
                if f.person_uuid.to_string() == person_uuid {
                    f.friend_uuid.to_string()
                } else {
                    f.person_uuid.to_string()
                }
            })
            .collect();

        let result = ExerciseGateway::find_by_complex_filters_paginated_uuid(
            db,
            person_uuid,
            friend_uuids,
            public_owner_uuids,
            category,
            visibility,
            page_index,
            page_size,
            sort_by,
        )
        .await;

        if result.is_err() {
            log::error!(
                "Error finding exercises with complex filters: {}",
                result.as_ref().err().unwrap()
            );
            return (vec![], 0, false);
        }

        let (models, total_count) = result.unwrap();
        let exercises: Vec<Exercise> = ExerciseEntityMapper::from_models(models);
        let has_next_page = ((page_index + 1) * page_size) < total_count;
        let total_count_i64 = total_count as i64;

        log::info!(
            "Found {} exercises (total: {})",
            exercises.len(),
            total_count_i64
        );
        (exercises, total_count_i64, has_next_page)
    }
}
