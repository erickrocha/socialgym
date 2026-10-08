use crate::domain::business_error::BusinessError;
use crate::domain::image_storage::ImageStorage;
use crate::gateway::aws_clients::{s3_client, s3_presign_client};
use crate::gateway::s3_gateway::S3Gateway;
use cloudfront_sign::{get_signed_url, SignedOptions};
use std::borrow::Cow;
use std::env;
use std::time::Duration;
use uuid::Uuid;

const AWS_WORKOUT_BUCKET: &str = "AWS_WORKOUT_BUCKET";
const PRESIGNED_URL_EXPIRATION_SECONDS: &str = "PRESIGNED_URL_EXPIRATION_SECONDS"; // 15 minutes
const CLOUDFRONT_KEY_PAIR_ID: &str = "CLOUDFRONT_KEY_PAIR_ID";
const CLOUDFRONT_DOMAIN: &str = "CLOUDFRONT_DOMAIN";
const PRIVATE_KEY_RAW: &str = "PRIVATE_KEY_RAW";

pub struct ImageStorageUseCase {}

impl ImageStorageUseCase {
    pub async fn upload_export(object_key: &str, bytes: Vec<u8>) -> Result<(), BusinessError> {
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS_WORKOUT_BUCKET must be set");
        S3Gateway::upload_bytes(
            &s3_client().await,
            &bucket,
            object_key,
            bytes,
            "application/zip",
        )
        .await
    }

    pub async fn download_object(object_key: &str) -> Result<Vec<u8>, BusinessError> {
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS_WORKOUT_BUCKET must be set");
        S3Gateway::download_bytes(&s3_client().await, &bucket, object_key).await
    }

    pub async fn export_download_url(object_key: &str) -> Result<String, BusinessError> {
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS_WORKOUT_BUCKET must be set");
        S3Gateway::get_object(
            &s3_presign_client().await,
            &bucket,
            object_key,
            Duration::from_secs(900),
        )
        .await
    }
    pub async fn update_presigned_url(
        person_id: i32,
        object_key: String,
        format: String,
    ) -> Result<ImageStorage, BusinessError> {
        let extension = format.split("/").last().unwrap_or("jpg");
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS WORKOUT_BUCKET must be set");
        let expiration_seconds = env::var(PRESIGNED_URL_EXPIRATION_SECONDS)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(900); // Default to 15 minutes if not set or invalid

        let client = s3_presign_client().await;
        let expires_in = Duration::from_secs(expiration_seconds);
        let content_type = format!("image/{}", extension);

        let s3_response =
            S3Gateway::put_object(&client, &bucket, &object_key, &content_type, expires_in).await;
        if s3_response.is_err() {
            log::error!("Error generating pre-signed URL: {:?}", s3_response.err());
            return Err(BusinessError::new(
                "Failed to generate pre-signed URL".to_string(),
            ));
        }

        let uri = s3_response?;
        Ok(ImageStorage::new(uri, object_key, person_id))
    }

    pub fn generate_object_key(owner_id: i32, extension: &str) -> String {
        // Gera um UUID v4 (ex: 67e55044-10b1-426f-9247-bb680e5fe0c8)
        let uuid = Uuid::new_v4();
        // generate te: uploads/owner-id/UUID.extension
        format!("uploads/owner-{}/{}{}", owner_id, uuid, extension)
    }

    /// Generates an S3 object key using the album-based path:
    /// `{person_uuid}/{album}/{new-uuid}`
    pub fn generate_album_object_key(entity_type: &str, person_uuid: &str, album: &str) -> String {
        let uuid = Uuid::new_v4();
        format!("{}/{}/{}/{}", entity_type, person_uuid, album, uuid)
    }

    /// Generates a pre-signed PUT URL using an album-based S3 key.
    /// Also accepts a raw content_type like `image/jpeg` or `video/mp4`.
    pub async fn generate_presigned_url(
        entity_type: String,
        owner_id: i32,
        owner_uuid: &str,
        album: &str,
        content_type: &str,
    ) -> Result<ImageStorage, BusinessError> {
        let object_key = Self::generate_album_object_key(&entity_type, owner_uuid, album);
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS_WORKOUT_BUCKET must be set");
        let expiration_seconds = env::var(PRESIGNED_URL_EXPIRATION_SECONDS)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(900);

        let client = s3_presign_client().await;
        let expires_in = Duration::from_secs(expiration_seconds);

        let s3_response =
            S3Gateway::put_object(&client, &bucket, &object_key, content_type, expires_in).await;

        match s3_response {
            Ok(uri) => {
                log::info!("Pre-signed upload link generated for owner_id={}", owner_id);
                Ok(ImageStorage::new(uri, object_key, owner_id))
            }
            Err(e) => {
                log::error!("Error generating album pre-signed URL: {:?}", e);
                Err(BusinessError::new(
                    "Failed to generate pre-signed URL".to_string(),
                ))
            }
        }
    }

    pub async fn generate_cloud_front_signed_url(
        object_key: &str,
    ) -> Result<String, BusinessError> {
        // 1. Load configs from environment variables
        let key_id = env::var(CLOUDFRONT_KEY_PAIR_ID).expect("CLOUDFRONT_KEY_PAIR_ID not defined");
        let domain = env::var(CLOUDFRONT_DOMAIN).expect("CLOUDFRONT_DOMAIN not defined");

        let raw_value = env::var(PRIVATE_KEY_RAW).expect("Private key not found");

        // 2. read the private key from the file system
        // Note: the private key should be in PEM format, and should be the one associated with the key pair ID used in CloudFront
        let private_key_pem = raw_value.replace("\\n", "\n");

        // 3. Monta a URL base
        let resource_url = format!("{}/{}", domain, object_key);

        // 4. Generate the signed URL using the cloudfront_sign crate
        let options = SignedOptions {
            key_pair_id: Cow::from(key_id),
            private_key: Cow::from(private_key_pem.trim().to_string()),
            ..Default::default()
        };

        let signed_url = get_signed_url(&resource_url, &options);
        if signed_url.is_err() {
            log::error!(
                "Error generating CloudFront signed URL: {:?}",
                signed_url.err()
            );
            return Err(BusinessError::new(
                "Failed to generate CloudFront signed URL".to_string(),
            ));
        }
        Ok(signed_url.unwrap())
    }

    pub async fn delete_presigned_url(object_key: String) -> Result<(), BusinessError> {
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS WORKOUT_BUCKET must be set");
        let client = s3_client().await;
        let result = S3Gateway::delete_object(&client, &bucket, &object_key).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Error deleting object from S3: {:?}", e);
                Err(BusinessError::new(
                    "Failed to delete object from S3".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
#[path = "../tests/image_storage_use_case_unit_test.rs"]
mod tests;
