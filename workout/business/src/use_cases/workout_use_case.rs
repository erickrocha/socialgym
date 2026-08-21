use crate::commons::authorization::ensure_owns;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::exercise::ExerciseEntityMapper;
use crate::domain::workout::{Workout, WorkoutEntityMapper};
use crate::gateway::exercise_gateway::ExerciseGateway;
use crate::gateway::workout_exercise_gateway::WorkoutExerciseGateway;
use crate::gateway::workout_gateway::WorkoutGateway;
use crate::domain::user::User;
use crate::use_cases::exercise_use_case::ExerciseUseCase;
use sea_orm::DbConn;

pub struct WorkoutUseCase {}

impl WorkoutUseCase {
    /// Create or update a workout on behalf of `actor`.
    ///
    /// The owner is always taken from `actor` — a client-supplied owner id is
    /// ignored — and updating an existing workout requires owning it.
    pub async fn persist(
        db: &DbConn,
        mut workout: Workout,
        actor: &User,
    ) -> Result<Workout, BusinessError> {
        log::info!(
            "[WorkoutUseCase::persist] Executing for actor person_id={}",
            actor.person_id
        );

        if let Some(id) = workout.id {
            let existing = Self::find_entity_by_id(db, id).await?;
            ensure_owns(existing.owner_id, actor.person_id)?;
        }

        // `description` is a `text` column (migration m20260129_000007), so
        // Postgres won't reject an oversized value on its own.
        const MAX_DESCRIPTION_LEN: usize = 5000;
        if workout.description.as_deref().is_some_and(|d| d.len() > MAX_DESCRIPTION_LEN) {
            return Err(BusinessError::validation(format!(
                "description must be at most {MAX_DESCRIPTION_LEN} characters"
            )));
        }

        workout.owner_id = actor.person_id;
        workout.owner_uuid = actor.person_uuid.clone();

        let exercises = workout.exercises.clone();
        let domain = WorkoutGateway::persist(db, workout).await;

        if domain.is_err() {
            log::error!("Error adding workout: {}", domain.as_ref().err().unwrap());
            return Err(BusinessError::infrastructure("Error adding workout"));
        }

        let model = domain.unwrap();
        let workout_id = model.id.clone().unwrap();

        let exercise_result =
            ExerciseUseCase::add_all_to_workout(db, workout_id, exercises, actor).await;

        if exercise_result.is_err() {
            log::error!("Error adding exercise: {}", exercise_result.err().unwrap());
            return Err(BusinessError::infrastructure("Error adding exercise"));
        }

        Ok(WorkoutEntityMapper::from_active_model(model))
    }

    pub async fn get(db: &DbConn, id: i32) -> Result<Workout, BusinessError> {
        log::info!("[WorkoutUseCase::get] Executing for id={}", id);
        let model = WorkoutGateway::find_by_id(db, id)
            .await
            .map_err(|error| {
                log::error!("[WorkoutUseCase::get] Failed for id={}: {}", id, error);
                BusinessError::infrastructure("Error getting workout")
            })?
            .ok_or_else(|| {
                let error = BusinessError::not_found("Workout not found");
                log::error!("[WorkoutUseCase::get] Failed for id={}: {}", id, error);
                error
            })?;
        let mut workout = WorkoutEntityMapper::from_model(model);
        Self::fill_exercise(db, &mut workout).await?;
        Ok(workout)
    }

    pub async fn get_by_uuid(db: &DbConn, uuid: String) -> Result<Workout, BusinessError> {
        log::info!("[WorkoutUseCase::get_by_uuid] Executing for uuid={}", uuid);
        let model = WorkoutGateway::find_by_uuid(db, uuid.clone())
            .await
            .map_err(|error| {
                log::error!(
                    "[WorkoutUseCase::get_by_uuid] Failed for uuid={}: {}",
                    uuid,
                    error
                );
                BusinessError::infrastructure("Error getting workout")
            })?
            .ok_or_else(|| {
                let error = BusinessError::not_found("Workout not found");
                log::error!(
                    "[WorkoutUseCase::get_by_uuid] Failed for uuid={}: {}",
                    uuid,
                    error
                );
                error
            })?;
        let mut workout = WorkoutEntityMapper::from_model(model);
        Self::fill_exercise(db, &mut workout).await?;
        Ok(workout)
    }

    pub async fn find_all_by_owner_id(
        db: &DbConn,
        owner_id: i32,
    ) -> Result<Vec<Workout>, BusinessError> {
        log::info!(
            "[WorkoutUseCase::find_all_by_owner_id] Executing for owner_id={}",
            owner_id
        );

        let domain = WorkoutGateway::find_by_owner_id(db, owner_id).await;

        if domain.is_err() {
            log::error!("Error finding workouts: {}", domain.as_ref().err().unwrap());
            return Err(BusinessError::infrastructure("Error finding workouts"));
        }

        let workouts = WorkoutEntityMapper::from_models(domain.unwrap());
        Self::fill_exercises(db, workouts).await
    }

    pub async fn find_all_by_owner_uuid(
        db: &DbConn,
        owner_uuid: String,
    ) -> Result<Vec<Workout>, BusinessError> {
        log::info!(
            "[WorkoutUseCase::find_all_by_owner_uuid] Executing for owner_uuid={}",
            owner_uuid
        );

        let domain = WorkoutGateway::find_by_owner_uuid(db, owner_uuid).await;

        if domain.is_err() {
            log::error!("Error finding workouts: {}", domain.as_ref().err().unwrap());
            return Err(BusinessError::infrastructure("Error finding workouts"));
        }

        let workouts = WorkoutEntityMapper::from_models(domain.unwrap());
        Self::fill_exercises(db, workouts).await
    }

    pub async fn delete_by_id(
        db: &DbConn,
        id: i32,
        acting_person_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!("[WorkoutUseCase::delete_by_id] Executing for id={}", id);
        let existing = Self::find_entity_by_id(db, id).await?;
        ensure_owns(existing.owner_id, acting_person_id)?;
        let result = WorkoutGateway::delete_by_id(db, id).await;
        if result.is_err() {
            log::error!("[WorkoutUseCase::delete_by_id] Failed for id={}", id);
            return Err(BusinessError::infrastructure("Failed to delete workout"));
        }
        Ok(())
    }

    pub async fn delete_by_uuid(
        db: &DbConn,
        uuid: String,
        acting_person_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!(
            "[WorkoutUseCase::delete_by_uuid] Executing for uuid={}",
            uuid
        );
        let existing = Self::find_entity_by_uuid(db, uuid.clone()).await?;
        ensure_owns(existing.owner_id, acting_person_id)?;
        let result = WorkoutGateway::delete_by_uuid(db, uuid.clone()).await;
        if result.is_err() {
            log::error!("[WorkoutUseCase::delete_by_uuid] Failed for uuid={}", uuid);
            return Err(BusinessError::infrastructure("Failed to delete workout"));
        }
        Ok(())
    }

    /// Owner id of a workout, without loading its exercises.
    async fn find_entity_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<entity::workout_entity::WorkoutEntity, BusinessError> {
        WorkoutGateway::find_by_id(db, id)
            .await
            .map_err(|error| {
                log::error!("[WorkoutUseCase] Failed to load workout id={}: {}", id, error);
                BusinessError::infrastructure("Error getting workout")
            })?
            .ok_or_else(|| BusinessError::not_found("Workout not found"))
    }

    async fn find_entity_by_uuid(
        db: &DbConn,
        uuid: String,
    ) -> Result<entity::workout_entity::WorkoutEntity, BusinessError> {
        WorkoutGateway::find_by_uuid(db, uuid.clone())
            .await
            .map_err(|error| {
                log::error!(
                    "[WorkoutUseCase] Failed to load workout uuid={}: {}",
                    uuid,
                    error
                );
                BusinessError::infrastructure("Error getting workout")
            })?
            .ok_or_else(|| BusinessError::not_found("Workout not found"))
    }

    /// Read guard: a workout is readable by its owner, or by anyone when public.
    /// `Visibility::Friends`/`Professional` stay owner-only until the permission
    /// model that can resolve their audience exists.
    pub fn ensure_readable(workout: &Workout, acting_person_id: i32) -> Result<(), BusinessError> {
        if matches!(workout.visibility, crate::domain::enums::Visibility::Public) {
            return Ok(());
        }
        ensure_owns(workout.owner_id, acting_person_id)
    }

    async fn fill_exercises(
        db: &DbConn,
        mut workouts: Vec<Workout>,
    ) -> Result<Vec<Workout>, BusinessError> {
        for workout in workouts.iter_mut() {
            Self::fill_exercise(db, workout).await?;
        }
        Ok(workouts)
    }

    async fn fill_exercise(db: &DbConn, workout: &mut Workout) -> Result<(), BusinessError> {
        let workout_exercises = WorkoutExerciseGateway::find_by_workout_id(db, workout.id.unwrap())
            .await
            .map_err(|error| {
                log::error!(
                    "[WorkoutUseCase::fill_exercise] Failed to load workout exercises: {}",
                    error
                );
                BusinessError::infrastructure("Error finding workout exercises")
            })?;
        let exercise_ids: Vec<i32> = workout_exercises.iter().map(|we| we.exercise_id).collect();
        let exercises = if exercise_ids.is_empty() {
            Vec::new()
        } else {
            let exercises = ExerciseGateway::find_by_ids(db, exercise_ids)
                .await
                .map_err(|error| {
                    log::error!(
                        "[WorkoutUseCase::fill_exercise] Failed to load exercises: {}",
                        error
                    );
                    BusinessError::infrastructure("Error finding exercises")
                })?;
            ExerciseEntityMapper::from_models(exercises)
        };
        workout.exercises = exercises;
        Ok(())
    }
}
