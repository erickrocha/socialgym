use crate::commons::authorization::{ensure_owns_as, ActingOwner};
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::BusinessProfile;
use crate::domain::enums::Visibility;
use crate::domain::exercise::{Exercise, ExerciseEntityMapper};
use crate::domain::user::User;
use crate::domain::workout_exercise::{WorkoutExercise, WorkoutExerciseEntityMapper};
use crate::gateway::exercise_gateway::ExerciseGateway;
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::person_gateway::PersonGateway;
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

        let acting = ActingOwner::new(actor, active_profile);

        let mut resolved: Vec<Exercise> = Vec::with_capacity(exercises.len());
        for mut exercise in exercises {
            if let Some(id) = exercise.id {
                let existing = Self::get(db, id).await?;
                Self::ensure_readable(db, &existing, &acting).await?;
                resolved.push(existing);
            } else {
                exercise.owner_id = acting.id;
                exercise.owner_uuid = acting.uuid.clone();
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

        let acting = ActingOwner::new(actor, active_profile);

        if let Some(id) = exercise.id {
            let existing = Self::get(db, id).await?;
            ensure_owns_as(existing.owner_id, &existing.owner_uuid, &acting)?;
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

        exercise.owner_id = acting.id;
        exercise.owner_uuid = acting.uuid;

        let model = ExerciseGateway::persist(db, exercise)
            .await
            .map_err(|error| {
                log::error!("[ExerciseUseCase::persist] Failed: {}", error);
                BusinessError::infrastructure("Error adding exercise")
            })?;
        Ok(ExerciseEntityMapper::from_active_model(model))
    }

    /// Read guard: an exercise is readable by its owner, by anyone when public,
    /// and by an accepted friend of the owner when its visibility is `Friends`.
    /// `Professional` stays owner-only until its audience can be resolved.
    pub async fn is_readable(
        db: &DbConn,
        exercise: &Exercise,
        acting: &ActingOwner,
    ) -> Result<bool, BusinessError> {
        audience_allows(db, &exercise.visibility, exercise.owner_id, &exercise.owner_uuid, acting)
            .await
    }

    /// An unreadable exercise is reported as not found, so a caller cannot tell a
    /// private exercise from a missing one.
    pub async fn ensure_readable(
        db: &DbConn,
        exercise: &Exercise,
        acting: &ActingOwner,
    ) -> Result<(), BusinessError> {
        if Self::is_readable(db, exercise, acting).await? {
            return Ok(());
        }
        Err(BusinessError::not_found("Exercise not found"))
    }

    pub async fn ensure_all_readable(
        db: &DbConn,
        exercises: &[Exercise],
        acting: &ActingOwner,
    ) -> Result<(), BusinessError> {
        for exercise in exercises {
            Self::ensure_readable(db, exercise, acting).await?;
        }
        Ok(())
    }

    /// Drops the exercises `acting` may not read.
    pub async fn retain_readable(
        db: &DbConn,
        exercises: Vec<Exercise>,
        acting: &ActingOwner,
    ) -> Result<Vec<Exercise>, BusinessError> {
        let mut readable = Vec::with_capacity(exercises.len());
        for exercise in exercises {
            if Self::is_readable(db, &exercise, acting).await? {
                readable.push(exercise);
            }
        }
        Ok(readable)
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
        acting: &ActingOwner,
    ) -> Result<(), BusinessError> {
        log::info!("Deleting exercise for exercise_id: {}", exercise_id);

        let existing = Self::get(db, exercise_id).await?;
        ensure_owns_as(existing.owner_id, &existing.owner_uuid, acting)?;

        let delete_result = ExerciseGateway::delete_by_id(db, exercise_id)
            .await
            .map_err(|e| {
                log::error!("Error deleting exercise: {}", e);
                BusinessError::new("Error deleting exercise".to_string())
            })?;

        if delete_result.rows_affected == 0 {
            return Err(BusinessError::not_found("Exercise not found"));
        }
        Ok(())
    }

    pub async fn delete_by_uuid(
        db: &DbConn,
        uuid: String,
        acting: &ActingOwner,
    ) -> Result<(), BusinessError> {
        log::info!("Deleting exercise for uuid: {}", uuid);

        let existing = Self::get_by_uuid(db, uuid.clone()).await?;
        ensure_owns_as(existing.owner_id, &existing.owner_uuid, acting)?;

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
            return Err(BusinessError::not_found("Exercise not found"));
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

    /// Search by Person ids. Runs as the acting identity and matches owners by uuid
    /// (see [`Self::find_by_complex_filters_paginated_uuid`]), so a Person id and a
    /// Business Profile id with the same number are never mixed up. Owner ids that
    /// match no Person are ignored.
    #[allow(clippy::too_many_arguments)]
    pub async fn find_by_complex_filters_paginated(
        db: &DbConn,
        acting: &ActingOwner,
        public_owner_ids: Vec<i32>,
        category: Option<String>,
        visibility: Option<String>,
        page_number: u64,
        page_size: u64,
        sort_by: Option<String>,
    ) -> Result<(Vec<Exercise>, i64, bool), BusinessError> {
        let mut public_owner_uuids = Vec::with_capacity(public_owner_ids.len());
        for id in public_owner_ids {
            if let Some(person) = PersonGateway::find_by_id(db, id).await {
                public_owner_uuids.push(person.uuid.to_string());
            }
        }
        Self::find_by_complex_filters_paginated_uuid(
            db,
            acting.uuid.clone(),
            public_owner_uuids,
            category,
            visibility,
            page_number,
            page_size,
            sort_by,
        )
        .await
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
    ) -> Result<(Vec<Exercise>, i64, bool), BusinessError> {
        let page_index = page_number - 1;

        let friends = FriendGateway::find_all_accepted_friends_by_uuid(db, person_uuid.clone())
            .await
            .map_err(|error| {
                log::error!("[ExerciseUseCase::find_by_complex_filters_paginated_uuid] Failed to find friends: {}", error);
                BusinessError::infrastructure("Error finding friends")
            })?;

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
}

/// Shared audience rule for exercises and workouts: owner, public, or (for
/// `Friends`) an accepted friend of the owner. Ownership matches on id and uuid.
pub(crate) async fn audience_allows(
    db: &DbConn,
    visibility: &Visibility,
    owner_id: i32,
    owner_uuid: &str,
    acting: &ActingOwner,
) -> Result<bool, BusinessError> {
    if matches!(visibility, Visibility::Public) || acting.owns(owner_id, owner_uuid) {
        return Ok(true);
    }
    if matches!(visibility, Visibility::Friends) {
        return FriendGateway::are_accepted_friends_by_uuid(db, &acting.uuid, owner_uuid)
            .await
            .map_err(|e| {
                log::error!("[audience_allows] Failed to check friendship: {}", e);
                BusinessError::infrastructure("Error checking friendship")
            });
    }
    Ok(false)
}
