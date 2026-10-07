//! WorkoutSessionService: the workout sessions of the caller (`/timeline/api/workout-sessions`).
//! Ownership rules stay in the use case the REST controllers call.
use crate::infrastructure::mapper::{WorkoutSessionMapper, optional_text_to_date};
use crate::infrastructure::utils::{business_status, enforce, with_caller};
use crate::proto::timeline::workout_session_service_server::WorkoutSessionService;
use crate::proto::timeline::*;
use business::commons::data_tools::opt_naive_to_bson_datetime;
use business::commons::rate_limit::content_limiter;
use business::use_cases::workout_use_case::WorkoutSessionUseCase;
use chrono::{Duration, Utc};
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcWorkoutSessionService {
    database: Arc<Database>,
}

impl GrpcWorkoutSessionService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl WorkoutSessionService for GrpcWorkoutSessionService {
    async fn create_workout_session(&self, request: Request<CreateWorkoutSessionRequest>) -> Result<Response<WorkoutSession>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let session = WorkoutSessionMapper::domain(body.session.ok_or_else(|| Status::invalid_argument("session is required"))?)?;
            let saved = WorkoutSessionUseCase::add(&db, session, &user.person_uuid)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(WorkoutSessionMapper::proto(saved)))
        })
        .await
    }

    async fn get_workout_session(&self, request: Request<GetWorkoutSessionRequest>) -> Result<Response<WorkoutSession>, Status> {
        let (db, id) = (self.database.clone(), request.get_ref().session_uuid.clone());
        with_caller(&request, |user| async move {
            let session = WorkoutSessionUseCase::find_by_id(&db, id, &user.person_uuid)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(WorkoutSessionMapper::proto(session)))
        })
        .await
    }

    async fn list_workout_sessions(&self, request: Request<ListWorkoutSessionsRequest>) -> Result<Response<ListWorkoutSessionsResponse>, Status> {
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let end = optional_text_to_date(body.end_date.as_deref())?.unwrap_or_else(|| Utc::now().naive_utc());
            let start = optional_text_to_date(body.start_date.as_deref())?.unwrap_or_else(|| end - Duration::days(7));
            let found = WorkoutSessionUseCase::find_all_by_person(
                &db,
                user.person_uuid,
                opt_naive_to_bson_datetime(start).unwrap(),
                opt_naive_to_bson_datetime(end).unwrap(),
            )
            .await;
            Ok(Response::new(ListWorkoutSessionsResponse {
                sessions: found.into_iter().map(WorkoutSessionMapper::proto).collect(),
            }))
        })
        .await
    }
}
