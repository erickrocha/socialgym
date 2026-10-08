use crate::infrastructure::utils::{locale_of, localized_status, require_person_id};
use crate::proto::media::media_service_server::MediaService;
use crate::proto::media::{MediaUploadRequest, MediaUploadResponse};
use business::commons::i18n::ErrorKey;
use business::use_cases::person_media_use_case::PersonMediaUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

/// The gRPC twin of the REST `/media/upload` route: a pre-signed link to upload post media.
pub struct GrpcMediaService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcMediaService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[tonic::async_trait]
impl MediaService for GrpcMediaService {
    async fn get_post_media_upload_url(&self, request: Request<MediaUploadRequest>) -> Result<Response<MediaUploadResponse>, Status> {
        let locale = locale_of(&request);
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();
        if payload.album.is_empty() {
            return Err(localized_status(Code::InvalidArgument, ErrorKey::RequiredParameterMissing, locale));
        }
        let format = if payload.format.is_empty() { "image/jpeg".to_string() } else { payload.format };
        PersonMediaUseCase::generate_upload_url(&self.conn, person_id, payload.album, format)
            .await
            .map(|image_storage| {
                Response::new(MediaUploadResponse { url: image_storage.url, object_key: image_storage.object_key, person_id })
            })
            .map_err(|_| localized_status(Code::InvalidArgument, ErrorKey::PreSignedUrlNotGenerated, locale))
    }
}
