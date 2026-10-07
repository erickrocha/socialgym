//! InternalService: the calls `workout` makes for account deletion and data export.
use crate::infrastructure::utils::business_status;
use crate::proto::timeline::internal_service_server::InternalService;
use crate::proto::timeline::*;
use business::use_cases::account_data_deletion_use_case::AccountDataDeletionUseCase;
use business::use_cases::person_data_export_use_case::PersonDataExportUseCase;
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcInternalService {
    database: Arc<Database>,
}

impl GrpcInternalService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl InternalService for GrpcInternalService {
    async fn delete_person_data(&self, request: Request<PersonDataRequest>) -> Result<Response<DeletePersonDataResponse>, Status> {
        AccountDataDeletionUseCase::delete_all_for_person(&self.database, &request.get_ref().person_uuid)
            .await
            .map_err(|e| business_status(&e))?;
        Ok(Response::new(DeletePersonDataResponse {}))
    }

    async fn export_person_data(&self, request: Request<PersonDataRequest>) -> Result<Response<ExportPersonDataResponse>, Status> {
        let export = PersonDataExportUseCase::export(&self.database, &request.get_ref().person_uuid)
            .await
            .map_err(|e| business_status(&e))?;
        Ok(Response::new(ExportPersonDataResponse { export_json: export.to_string() }))
    }
}
