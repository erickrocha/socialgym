//! ContentReportService and WorkoutSessionService. Moderation and ownership rules stay in the
//! use cases the REST controllers call.
use super::auth::with_caller;
use super::convert::*;
use super::rate_limit::enforce;
use super::services::Services;
use super::status::status_from_business;
use crate::authentication::rate_limit::content_limiter;
use crate::infrastructure::data_tools::opt_naive_to_bson_datetime;
use crate::infrastructure::mapper::{Mapper, WorkoutMapper};
use business::proto::proto::timeline::content_report_service_server::ContentReportService;
use business::proto::proto::timeline::workout_session_service_server::WorkoutSessionService;
use business::proto::proto::timeline::*;
use business::use_cases::content_report_use_case::ContentReportUseCase;
use business::use_cases::workout_use_case::WorkoutSessionUseCase;
use chrono::{Duration, Utc};
use domain::business_error::BusinessError;
use tonic::{Request, Response, Status};

fn failed(error: BusinessError) -> Status {
    status_from_business(&error)
}

#[tonic::async_trait]
impl ContentReportService for Services {
    async fn create_report(&self, request: Request<CreateReportRequest>) -> Result<Response<ContentReport>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let report = ContentReportUseCase::create(&db, &user, body.target_type, body.target_id, body.post_id, body.reason, body.details)
                .await
                .map_err(failed)?;
            Ok(Response::new(report_to(report)))
        })
        .await
    }

    async fn list_reports(&self, request: Request<ListReportsRequest>) -> Result<Response<ListReportsResponse>, Status> {
        let (db, status) = (self.db(), request.get_ref().status.clone());
        with_caller(&request, |_| async move {
            let reports = ContentReportUseCase::list(&db, status.as_deref()).await.map_err(failed)?;
            Ok(Response::new(ListReportsResponse { reports: reports.into_iter().map(report_to).collect() }))
        })
        .await
    }

    async fn decide_report(&self, request: Request<DecideReportRequest>) -> Result<Response<ContentReport>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let report = ContentReportUseCase::decide(&db, &body.report_id, &user.person_uuid, &body.decision, &body.reason)
                .await
                .map_err(failed)?;
            Ok(Response::new(report_to(report)))
        })
        .await
    }
}

#[tonic::async_trait]
impl WorkoutSessionService for Services {
    async fn create_workout_session(&self, request: Request<CreateWorkoutSessionRequest>) -> Result<Response<WorkoutSession>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let json = session_from(body.session.ok_or_else(|| Status::invalid_argument("session is required"))?)?;
            if !WorkoutMapper::is_complete(&json) {
                return Err(Status::invalid_argument("session dates and set owner names are required"));
            }
            let saved = WorkoutSessionUseCase::add(&db, WorkoutMapper::domain(json), &user.person_uuid).await.map_err(failed)?;
            Ok(Response::new(session_to(WorkoutMapper::json(saved))))
        })
        .await
    }

    async fn get_workout_session(&self, request: Request<GetWorkoutSessionRequest>) -> Result<Response<WorkoutSession>, Status> {
        let (db, id) = (self.db(), request.get_ref().session_uuid.clone());
        with_caller(&request, |user| async move {
            let session = WorkoutSessionUseCase::find_by_id(&db, id, &user.person_uuid).await.map_err(failed)?;
            Ok(Response::new(session_to(WorkoutMapper::json(session))))
        })
        .await
    }

    async fn list_workout_sessions(&self, request: Request<ListWorkoutSessionsRequest>) -> Result<Response<ListWorkoutSessionsResponse>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
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
                sessions: found.into_iter().map(|s| session_to(WorkoutMapper::json(s))).collect(),
            }))
        })
        .await
    }
}
