use crate::commons::authorization::ensure_owns;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::BusinessProfile;
use crate::domain::enums::Visibility;
use crate::domain::exercise::{Exercise, ExerciseEntityMapper};
use crate::domain::user::User;
use crate::domain::workout_exercise::{WorkoutExercise, WorkoutExerciseEntityMapper};
use crate::gateway::exercise_gateway::ExerciseGateway;
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::workout_exercise_gateway::WorkoutExerciseGateway;
use sea_orm::DbConn;

pub struct ExerciseUseCase {}

impl ExerciseUseCase {
    /// Attach exercises to a workout on behalf of `actor` (acting as
    /// `active_profile` when a business profile is active).
    ///
    /// New exercises are created owned by the acting identity; an entry that
    /// references an existing exercise must be one the acting identity is
    /// allowed to read, so a workout cannot be used to pull somebody else's
    /// private exercise into view.
    pub async fn add_all_to_workout(
        db: &DbConn,
        workout_id: i32,
        exercises: Vec<Exercise>,
        actor: &User,
        active_profile: Option<&BusinessProfile>,
    ) -> Result<Vec<Exercise>, BusinessError> {
        log::info!(
            "Adding {} exercises to workout_id {}",
            exercises.len(),
            workout_id
        );

        if exercises.is_empty() {
            return Ok(Vec::new());
        }

        let acting_owner_id = active_profile.and_then(|p| p.id).unwrap_or(actor.person_id);
        let acting_owner_uuid = active_profile
            .and_then(|p| p.uuid.clone())
            .unwrap_or_else(|| actor.person_uuid.clone());

        let mut resolved: Vec<Exercise> = Vec::with_capacity(exercises.len());
        for mut exercise in exercises {
            if let Some(id) = exercise.id {
                let existing = Self::get(db, id).await?;
                Self::ensure_readable(&existing, acting_owner_id)?;
                resolved.push(existing);
            } else {
                exercise.owner_id = acting_owner_id;
                exercise.owner_uuid = acting_owner_uuid.clone();
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

    /// Create or update an exercise on behalf of `actor` (acting as
    /// `active_profile` when a business profile is active).
    ///
    /// The owner is always taken from the acting identity — a client-supplied
    /// owner id is ignored — and updating an existing exercise requires
    /// owning it.
    pub async fn persist(
        db: &DbConn,
        mut exercise: Exercise,
        actor: &User,
        active_profile: Option<&BusinessProfile>,
    ) -> Result<Exercise, BusinessError> {
        log::info!(
            "[ExerciseUseCase::persist] Executing for actor person_id={}",
            actor.person_id
        );

        let acting_owner_id = active_profile.and_then(|p| p.id).unwrap_or(actor.person_id);
        let acting_owner_uuid = active_profile
            .and_then(|p| p.uuid.clone())
            .unwrap_or_else(|| actor.person_uuid.clone());

        if let Some(id) = exercise.id {
            let existing = Self::get(db, id).await?;
            ensure_owns(existing.owner_id, acting_owner_id)?;
        }

        // Matches the varchar(255) column (migration m20260129_000008); Postgres
        // would reject this anyway, but a validation error is friendlier than a
        // raw DB error surfacing to the client.
        const MAX_DESCRIPTION_LEN: usize = 255;
        if exercise.description.as_deref().is_some_and(|d| d.len() > MAX_DESCRIPTION_LEN) {
            return Err(BusinessError::validation(format!(
                "description must be at most {MAX_DESCRIPTION_LEN} characters"
            )));
        }

        exercise.owner_id = acting_owner_id;
        exercise.owner_uuid = acting_owner_uuid;

        let model = ExerciseGateway::persist(db, exercise)
            .await
            .map_err(|error| {
                log::error!("[ExerciseUseCase::persist] Failed: {}", error);
                BusinessError::infrastructure("Error adding exercise")
            })?;
        Ok(ExerciseEntityMapper::from_active_model(model))
    }

    /// Read guard: an exercise is readable by its owner, or by anyone when
    /// public. `Visibility::Friends`/`Professional` stay owner-only until the
    /// permission model that can resolve their audience exists.
    pub fn ensure_readable(exercise: &Exercise, acting_person_id: i32) -> Result<(), BusinessError> {
        if matches!(exercise.visibility, Visibility::Public) {
            return Ok(());
        }
        ensure_owns(exercise.owner_id, acting_person_id)
    }

    pub async fn get(db: &DbConn, exercise_id: i32) -> Result<Exercise, BusinessError> {
        log::info!(
            "[ExerciseUseCase::get] Executing for exercise_id={}",
            exercise_id
        );
        let model = ExerciseGateway::find_by_id(db, exercise_id)
            .await
            .map_err(|error| {
                log::error!(
                    "[ExerciseUseCase::get] Failed for exercise_id={}: {}",
                    exercise_id,
                    error
                );
                BusinessError::infrastructure("Error getting exercise")
            })?
            .ok_or_else(|| {
                let error = BusinessError::not_found("Exercise not found");
                log::error!(
                    "[ExerciseUseCase::get] Failed for exercise_id={}: {}",
                    exercise_id,
                    error
                );
                error
            })?;
        Ok(ExerciseEntityMapper::from_model(model))
    }

    pub async fn get_by_uuid(db: &DbConn, uuid: String) -> Result<Exercise, BusinessError> {
        log::info!("[ExerciseUseCase::get_by_uuid] Executing for uuid={}", uuid);
        let model = ExerciseGateway::find_by_uuid(db, uuid.clone())
            .await
            .map_err(|error| {
                log::error!(
                    "[ExerciseUseCase::get_by_uuid] Failed for uuid={}: {}",
                    uuid,
                    error
                );
                BusinessError::infrastructure("Error getting exercise")
            })?
            .ok_or_else(|| {
                let error = BusinessError::not_found("Exercise not found");
                log::error!(
                    "[ExerciseUseCase::get_by_uuid] Failed for uuid={}: {}",
                    uuid,
                    error
                );
                error
            })?;
        Ok(ExerciseEntityMapper::from_model(model))
    }

    pub async fn find_all_by_workout_id(
        db: &DbConn,
        workout_id: i32,
    ) -> Result<Vec<Exercise>, BusinessError> {
        log::info!("Finding exercises for workout_id: {}", workout_id);

        let workout_exercises = WorkoutExerciseGateway::find_by_workout_id(db, workout_id)
            .await
            .map_err(|error| {
                log::error!(
                    "[ExerciseUseCase::find_all_by_workout_id] Failed: {}",
                    error
                );
                BusinessError::infrastructure("Error finding workout exercises")
            })?;
        let exercise_ids: Vec<i32> = workout_exercises.iter().map(|we| we.exercise_id).collect();

        if exercise_ids.is_empty() {
            return Ok(Vec::new());
        }

        let domain = ExerciseGateway::find_by_ids(db, exercise_ids)
            .await
            .map_err(|error| {
                log::error!(
                    "[ExerciseUseCase::find_all_by_workout_id] Failed: {}",
                    error
                );
                BusinessError::infrastructure("Error finding exercises")
            })?;
        let exercises: Vec<Exercise> = ExerciseEntityMapper::from_models(domain);
        Ok(exercises)
    }

    pub async fn delete_by_id(
        db: &DbConn,
        exercise_id: i32,
        acting_person_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!("Deleting exercise for exercise_id: {}", exercise_id);

        let existing = Self::get(db, exercise_id).await?;
        ensure_owns(existing.owner_id, acting_person_id)?;

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

    pub async fn delete_by_uuid(
        db: &DbConn,
        uuid: String,
        acting_person_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!("Deleting exercise for uuid: {}", uuid);

        let existing = Self::get_by_uuid(db, uuid.clone()).await?;
        ensure_owns(existing.owner_id, acting_person_id)?;

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
    ) -> Result<(Vec<Exercise>, i64, bool), BusinessError> {
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
            .map_err(|error| {
                log::error!("[ExerciseUseCase::find_by_complex_filters_paginated] Failed to find friends: {}", error);
                BusinessError::infrastructure("Error finding friends")
            })?;

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
            return Err(BusinessError::infrastructure("Error finding exercises"));
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
        Ok((exercises, total_count_i64, has_next_page))
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
