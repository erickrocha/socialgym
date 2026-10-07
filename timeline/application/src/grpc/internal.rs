//! InternalService: the calls `workout` makes for account deletion and data export.
use super::services::Services;
use super::status::status_from_business;
use business::proto::proto::timeline::internal_service_server::InternalService;
use business::proto::proto::timeline::*;
use business::use_cases::account_data_deletion_use_case::AccountDataDeletionUseCase;
use business::use_cases::person_data_export_use_case::PersonDataExportUseCase;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl InternalService for Services {
    async fn delete_person_data(&self, request: Request<PersonDataRequest>) -> Result<Response<DeletePersonDataResponse>, Status> {
        AccountDataDeletionUseCase::delete_all_for_person(&self.db(), &request.get_ref().person_uuid)
            .await
            .map_err(|e| status_from_business(&e))?;
        Ok(Response::new(DeletePersonDataResponse {}))
    }

    async fn export_person_data(&self, request: Request<PersonDataRequest>) -> Result<Response<ExportPersonDataResponse>, Status> {
        let export = PersonDataExportUseCase::export(&self.db(), &request.get_ref().person_uuid)
            .await
            .map_err(|e| status_from_business(&e))?;
        Ok(Response::new(ExportPersonDataResponse { export_json: export.to_string() }))
    }
}
