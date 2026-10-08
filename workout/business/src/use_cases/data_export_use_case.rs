use crate::domain::business_error::BusinessError;
use crate::gateway::data_export_gateway::DataExportGateway;
use crate::use_cases::image_storage_use_case::ImageStorageUseCase;
use chrono::Utc;
use entity::data_export_entity::Model as DataExport;
use sea_orm::DbConn;

/// How long the pre-signed download link of a ready export stays valid.
pub const DOWNLOAD_LINK_SECONDS: u32 = 900;

/// A link to a ready export file.
pub struct ExportDownload {
    pub url: String,
    pub expires_in_seconds: u32,
}

/// A person's data exports: asking for one, listing them, and getting the link to a ready file. Only the
/// owner sees an export; to anyone else it does not exist. Shared by the REST and gRPC forms.
pub struct DataExportUseCase;

impl DataExportUseCase {
    pub async fn create(db: &DbConn, person_id: i32) -> Result<DataExport, BusinessError> {
        DataExportGateway::create(db, person_id)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))
    }

    pub async fn list(db: &DbConn, person_id: i32) -> Result<Vec<DataExport>, BusinessError> {
        DataExportGateway::list_for_person(db, person_id)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))
    }

    /// The export `id` (a UUID) of `person_id`: validation error for a malformed id, not found for an
    /// export that is missing or belongs to someone else.
    pub async fn get(db: &DbConn, person_id: i32, id: &str) -> Result<DataExport, BusinessError> {
        let uuid = uuid::Uuid::parse_str(id).map_err(|_| BusinessError::validation("Invalid export id"))?;
        DataExportGateway::find_owned(db, uuid, person_id)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))?
            .ok_or_else(|| BusinessError::not_found("Export not found"))
    }

    /// The download link of a ready, unexpired export; a conflict while the export is not ready.
    pub async fn download(db: &DbConn, person_id: i32, id: &str) -> Result<ExportDownload, BusinessError> {
        let export = Self::get(db, person_id, id).await?;
        let ready = export.status == "ready" && export.expires_at.is_some_and(|at| at > Utc::now());
        let key = match export.object_key {
            Some(key) if ready => key,
            _ => return Err(BusinessError::conflict("Export is not ready")),
        };
        let url = ImageStorageUseCase::export_download_url(&key).await?;
        Ok(ExportDownload { url, expires_in_seconds: DOWNLOAD_LINK_SECONDS })
    }
}
