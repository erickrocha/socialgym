use crate::domain::business_error::BusinessError;
use crate::domain::image_storage::ImageStorage;
use crate::gateway::person_gateway::PersonGateway;
use crate::gateway::person_media_gateway::PersonMediaGateway;
use crate::use_cases::common_use_case::{handle_option};
use crate::use_cases::image_storage_use_case::ImageStorageUseCase;
use sea_orm::{DbConn};
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::person::PersonEntityMapper;

pub struct PersonMediaUseCase {}

impl PersonMediaUseCase {
    /// Generates a pre-signed PUT URL for uploading media to S3 using the album-based key format:
    /// `{person_uuid}/{album}/{new-uuid}`.
    ///
    /// A corresponding row is saved in the `person_media` table so the uploaded file can be
    /// tracked and later retrieved via the CloudFront CDN.
    ///
    /// # Arguments
    /// * `person_id`    – ID of the logged-in person
    /// * `album`        – Album name (e.g. `"avatar"`, `"cover"`, `"gallery"`)
    /// * `content_type` – Full MIME type string (e.g. `"image/jpeg"`, `"video/mp4"`)
    pub async fn generate_upload_url(db: &DbConn,person_id: i32,album: String,content_type: String) -> Result<ImageStorage, BusinessError> {
        log::info!("Generating upload URL for person_id={} album={} content_type={}",person_id,album,content_type);

        // Resolve person_uuid needed for the S3 key hierarchy
        let person_option = PersonGateway::find_by_id(db, person_id).await;
        let person_model = handle_option(person_option, "Person not found")?;
        let person = PersonEntityMapper::from_model(person_model);

        let person_uuid = person.uuid.unwrap_or_else(|| "default".to_string());

        // Generate presigned URL and build the S3 key
        let image_storage = ImageStorageUseCase::generate_presigned_url("person".to_string(),person_id,&person_uuid,&album,&content_type).await?;

        Ok(image_storage)
    }

    /// Deletes a media file from S3 and removes the matching row from `person_media`.
    pub async fn delete_media(db: &DbConn, s3_key: &str) -> Result<(), BusinessError> {
        log::info!("Deleting media with s3_key={}", s3_key);

        // Delete from S3 first
        ImageStorageUseCase::delete_presigned_url(s3_key.to_string()).await?;

        // Remove the tracking record
        if let Err(e) = PersonMediaGateway::delete_by_s3_key(db, s3_key).await {
            log::error!("Error deleting person_media record for key {}: {:?}", s3_key, e);
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------
}

