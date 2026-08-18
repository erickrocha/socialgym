use crate::gateway::workout_session_gateway::WorkoutSessionGateway;
use crate::repositories::repository::Repository;
use domain::business_error::BusinessError;
use domain::workout_session::WorkoutSession;
use mongodb::bson::DateTime;
use mongodb::Database;

pub struct WorkoutSessionUseCase {}

impl WorkoutSessionUseCase {
    pub async fn add(db: &Database, domain: WorkoutSession) -> Result<WorkoutSession, BusinessError> {
        log::info!("Adding workout: {:?}", domain);
        let workout_gateway = WorkoutSessionGateway::new(db);
        let response = workout_gateway.persist(domain).await;
        if response.is_err() {
            log::error!("Error adding workout: {:?}", response.err().unwrap());
            return Err(BusinessError::new("Failed to add workout".to_string()));
        }
        let workout = response?;
        Ok(workout)
    }

    pub async fn find_by_id(db: &Database, id: String) -> Option<WorkoutSession> {
        log::info!("Finding workout by id: {:?}", id);
        let workout_gateway = WorkoutSessionGateway::new(db);
        workout_gateway.find_by_id(id).await
    }

    pub async fn find_all_by_person(db: &Database,person_uuid: String,start: DateTime,end: DateTime) -> Vec<WorkoutSession> {
        log::info!("Finding all workouts by person uuid: {:?}", person_uuid);
        let workout_gateway = WorkoutSessionGateway::new(db);
        workout_gateway.find_all_by_person(person_uuid.as_str(), start, end).await
    }
}
