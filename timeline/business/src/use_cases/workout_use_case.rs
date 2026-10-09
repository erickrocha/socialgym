use crate::gateway::workout_session_gateway::WorkoutSessionGateway;
use crate::repositories::repository::Repository;
use domain::business_error::BusinessError;
use domain::workout_session::WorkoutSession;
use mongodb::Database;
use mongodb::bson::DateTime;

pub struct WorkoutSessionUseCase {}

impl WorkoutSessionUseCase {
    /// Record a workout session for `acting_person_uuid`. A `personUuid` in the
    /// request body is ignored: the session always belongs to the caller.
    pub async fn add(
        db: &Database,
        mut domain: WorkoutSession,
        acting_person_uuid: &str,
    ) -> Result<WorkoutSession, BusinessError> {
        log::info!(
            "Adding workout session for person_uuid={}",
            acting_person_uuid
        );
        domain.person_uuid = Some(acting_person_uuid.to_string());
        let workout_gateway = WorkoutSessionGateway::new(db);
        let response = workout_gateway.persist(domain).await;
        if response.is_err() {
            log::error!("Error adding workout: {:?}", response.err().unwrap());
            return Err(BusinessError::new("Failed to add workout".to_string()));
        }
        let workout = response?;
        Ok(workout)
    }

    /// Sessions are private to the person who recorded them.
    pub async fn find_by_id(
        db: &Database,
        id: String,
        acting_person_uuid: &str,
    ) -> Result<WorkoutSession, BusinessError> {
        log::info!("Finding workout by id: {:?}", id);
        let workout_gateway = WorkoutSessionGateway::new(db);
        let session = workout_gateway
            .find_by_id(id)
            .await
            .ok_or_else(|| BusinessError::not_found("Workout session not found"))?;
        // Someone else's session is indistinguishable from a missing one.
        if session.person_uuid.as_deref() != Some(acting_person_uuid) {
            return Err(BusinessError::not_found("Workout session not found"));
        }
        Ok(session)
    }

    pub async fn find_all_by_person(
        db: &Database,
        person_uuid: String,
        start: DateTime,
        end: DateTime,
    ) -> Vec<WorkoutSession> {
        log::info!("Finding all workouts by person uuid: {:?}", person_uuid);
        let workout_gateway = WorkoutSessionGateway::new(db);
        workout_gateway
            .find_all_by_person(person_uuid.as_str(), start, end)
            .await
    }
}
