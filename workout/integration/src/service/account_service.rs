use crate::infrastructure::mapper::DataExportMapper;
use crate::infrastructure::utils::{locale_of, localized_business_status, localized_status, require_actor};
use crate::proto::account::account_service_server::AccountService;
use crate::proto::account::{
    AccountDeletionStatus, CancelAccountDeletionRequest, CancelAccountDeletionResponse, CreateDataExportRequest, DataExport,
    DataExportDownload, GetDataExportRequest, ListDataExportsRequest, ListDataExportsResponse, RequestAccountDeletionRequest,
};
use business::commons::i18n::{ErrorKey, Locale};
use business::domain::business_error::{BusinessError, BusinessErrorKind};
use business::use_cases::account_deletion_use_case::AccountDeletionUseCase;
use business::use_cases::data_export_use_case::DataExportUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

/// The gRPC twin of the REST account-deletion and data-export routes; the rules live in the use cases.
pub struct GrpcAccountService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcAccountService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

/// How each failure of the export use case reads, as over REST.
fn export_failure(error: BusinessError, locale: Locale) -> Status {
    match error.kind {
        BusinessErrorKind::Validation => localized_status(Code::InvalidArgument, ErrorKey::InvalidParameterValue, locale),
        BusinessErrorKind::NotFound => localized_status(Code::NotFound, ErrorKey::DataExportNotReady, locale),
        BusinessErrorKind::Conflict => localized_status(Code::AlreadyExists, ErrorKey::DataExportNotReady, locale),
        _ => localized_status(Code::Unavailable, ErrorKey::DataExportFailed, locale),
    }
}

#[tonic::async_trait]
impl AccountService for GrpcAccountService {
    async fn request_account_deletion(&self, request: Request<RequestAccountDeletionRequest>) -> Result<Response<AccountDeletionStatus>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        let immediate = request.into_inner().immediate;
        AccountDeletionUseCase::request_deletion(&self.conn, user.id.unwrap_or_default(), immediate)
            .await
            .map(|status| {
                Response::new(AccountDeletionStatus {
                    requested_at: status.requested_at.naive_utc().to_string(),
                    scheduled_at: status.scheduled_at.naive_utc().to_string(),
                })
            })
            .map_err(|_| localized_status(Code::Unavailable, ErrorKey::AccountDeletionRequestFailed, locale))
    }

    async fn cancel_account_deletion(&self, request: Request<CancelAccountDeletionRequest>) -> Result<Response<CancelAccountDeletionResponse>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        AccountDeletionUseCase::cancel_deletion(&self.conn, user.id.unwrap_or_default())
            .await
            .map(|_| Response::new(CancelAccountDeletionResponse {}))
            .map_err(|e| {
                let key = match e.kind {
                    BusinessErrorKind::Validation | BusinessErrorKind::NotFound => ErrorKey::AccountDeletionNotPending,
                    _ => ErrorKey::AccountDeletionCancelFailed,
                };
                localized_business_status(e, key, locale)
            })
    }

    async fn create_data_export(&self, request: Request<CreateDataExportRequest>) -> Result<Response<DataExport>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        DataExportUseCase::create(&self.conn, user.person_id)
            .await
            .map(|row| Response::new(DataExportMapper::response(row)))
            .map_err(|e| export_failure(e, locale))
    }

    async fn list_data_exports(&self, request: Request<ListDataExportsRequest>) -> Result<Response<ListDataExportsResponse>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        DataExportUseCase::list(&self.conn, user.person_id)
            .await
            .map(|rows| Response::new(ListDataExportsResponse { exports: rows.into_iter().map(DataExportMapper::response).collect() }))
            .map_err(|e| export_failure(e, locale))
    }

    async fn get_data_export(&self, request: Request<GetDataExportRequest>) -> Result<Response<DataExport>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        DataExportUseCase::get(&self.conn, user.person_id, &request.into_inner().id)
            .await
            .map(|row| Response::new(DataExportMapper::response(row)))
            .map_err(|e| export_failure(e, locale))
    }

    async fn get_data_export_download_url(&self, request: Request<GetDataExportRequest>) -> Result<Response<DataExportDownload>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        DataExportUseCase::download(&self.conn, user.person_id, &request.into_inner().id)
            .await
            .map(|link| Response::new(DataExportDownload { url: link.url, expires_in_seconds: link.expires_in_seconds }))
            .map_err(|e| export_failure(e, locale))
    }
}
