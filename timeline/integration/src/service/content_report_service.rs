//! ContentReportService: reports and moderation (`/timeline/api/reports`, `/moderation`). The
//! moderation rules stay in the use case the REST controllers call.
use crate::infrastructure::mapper::ContentReportMapper;
use crate::infrastructure::utils::{business_status, with_caller};
use crate::proto::timeline::content_report_service_server::ContentReportService;
use crate::proto::timeline::*;
use business::use_cases::content_report_use_case::ContentReportUseCase;
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcContentReportService {
    database: Arc<Database>,
}

impl GrpcContentReportService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl ContentReportService for GrpcContentReportService {
    async fn create_report(
        &self,
        request: Request<CreateReportRequest>,
    ) -> Result<Response<ContentReport>, Status> {
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let report = ContentReportUseCase::create(
                &db,
                &user,
                body.target_type,
                body.target_id,
                body.post_id,
                body.reason,
                body.details,
            )
            .await
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(ContentReportMapper::proto(report)))
        })
        .await
    }

    async fn list_reports(
        &self,
        request: Request<ListReportsRequest>,
    ) -> Result<Response<ListReportsResponse>, Status> {
        let (db, status) = (self.database.clone(), request.get_ref().status.clone());
        with_caller(&request, |_| async move {
            let reports = ContentReportUseCase::list(&db, status.as_deref())
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(ListReportsResponse {
                reports: reports
                    .into_iter()
                    .map(ContentReportMapper::proto)
                    .collect(),
            }))
        })
        .await
    }

    async fn decide_report(
        &self,
        request: Request<DecideReportRequest>,
    ) -> Result<Response<ContentReport>, Status> {
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let report = ContentReportUseCase::decide(
                &db,
                &body.report_id,
                &user.person_uuid,
                &body.decision,
                &body.reason,
            )
            .await
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(ContentReportMapper::proto(report)))
        })
        .await
    }
}
