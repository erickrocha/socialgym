use crate::domain::business_error::BusinessError;
use crate::domain::image_storage::ImageStorage;
use crate::gateway::s3_gateway::S3Gateway;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use cloudfront_sign::{get_signed_url, SignedOptions};
use std::borrow::Cow;
use std::time::Duration;
use std::{env};
use uuid::Uuid;

const AWS_WORKOUT_BUCKET: &str = "AWS_WORKOUT_BUCKET";
const PRESIGNED_URL_EXPIRATION_SECONDS: &str = "PRESIGNED_URL_EXPIRATION_SECONDS"; // 15 minutes
const CLOUDFRONT_KEY_PAIR_ID: &str = "CLOUDFRONT_KEY_PAIR_ID";
const CLOUDFRONT_DOMAIN: &str = "CLOUDFRONT_DOMAIN";
const PRIVATE_KEY_RAW: &str = "PRIVATE_KEY_RAW";

pub struct ImageStorageUseCase {}

impl ImageStorageUseCase {
    pub async fn update_presigned_url(person_id: i32,object_key: String,format: String) -> Result<ImageStorage, BusinessError> {
        let extension = format.split("/").last().unwrap_or("jpg");
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS WORKOUT_BUCKET must be set");
        let expiration_seconds = env::var(PRESIGNED_URL_EXPIRATION_SECONDS)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(900); // Default to 15 minutes if not set or invalid

        let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&shared_config);
        let expires_in = Duration::from_secs(expiration_seconds);
        let content_type = format!("image/{}", extension);

        let s3_response = S3Gateway::put_object(&client, &bucket, &object_key, &content_type, expires_in).await;
        if s3_response.is_err() {
            log::error!("Error generating pre-signed URL: {:?}", s3_response.err());
            return Err(BusinessError::new(
                "Failed to generate pre-signed URL".to_string(),
            ));
        }

        let uri = s3_response?;
        println!("Presigned PUT URI: {}", uri);
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
    pub async fn generate_presigned_url(entity_type: String, owner_id: i32, owner_uuid: &str, album: &str, content_type: &str,
    ) -> Result<ImageStorage, BusinessError> {
        let object_key = Self::generate_album_object_key(&entity_type, owner_uuid, album);
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS_WORKOUT_BUCKET must be set");
        let expiration_seconds = env::var(PRESIGNED_URL_EXPIRATION_SECONDS)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(900);

        let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&shared_config);
        let expires_in = Duration::from_secs(expiration_seconds);

        let s3_response = S3Gateway::put_object(&client, &bucket, &object_key, content_type, expires_in).await;

        match s3_response {
            Ok(uri) => {
                log::info!("Presigned PUT URI (album): {}", uri);
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

    pub async fn generate_cloud_front_signed_url(object_key: &str) -> Result<String, BusinessError> {
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
            log::error!("Error generating CloudFront signed URL: {:?}",signed_url.err());
            return Err(BusinessError::new("Failed to generate CloudFront signed URL".to_string()));
        }
        Ok(signed_url.unwrap())
    }
    
    pub async fn delete_presigned_url(object_key: String) -> Result<(), BusinessError> {
        let bucket = env::var(AWS_WORKOUT_BUCKET).expect("AWS WORKOUT_BUCKET must be set");
        let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&shared_config);
        let result = S3Gateway::delete_object(&client, &bucket, &object_key).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Error deleting object from S3: {:?}", e);
                Err(BusinessError::new("Failed to delete object from S3".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    // ========================
    // generate_object_key Tests
    // ========================

    use crate::domain::enums::ImageType;
    use crate::domain::image_storage::ImageStorage;
    use crate::use_cases::image_storage_use_case::ImageStorageUseCase;

    #[test]
    fn test_generate_object_key_format_is_correct() {
        let owner_id = 123;
        let extension = "-profile.jpg";

        let object_key = ImageStorageUseCase::generate_object_key(owner_id, extension);

        // Verify the structure: uploads/owner-{id}/{uuid}{extension}
        assert!(object_key.starts_with("uploads/owner-123/"));
        assert!(object_key.ends_with(extension));
    }

    #[test]
    fn test_generate_object_key_contains_valid_uuid() {
        let owner_id = 456;
        let extension = "-cover.png";

        let object_key = ImageStorageUseCase::generate_object_key(owner_id, extension);
        let parts: Vec<&str> = object_key.split('/').collect();

        // Should have 3 parts: uploads, owner-{id}, {uuid}{extension}
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "uploads");
        assert_eq!(parts[1], "owner-456");

        // UUID part should be 36 characters + extension length - 1 (the hyphen in extension)
        let uuid_with_ext = parts[2];
        // UUID v4 is 36 characters (with hyphens), extension is like "-cover.png"
        // So total should be around 36 + extension.len()
        assert!(uuid_with_ext.len() > 40);
    }

    #[test]
    fn test_generate_object_key_different_calls_produce_different_uuids() {
        let owner_id = 789;
        let extension = "-avatar.jpeg";

        let key1 = ImageStorageUseCase::generate_object_key(owner_id, extension);
        let key2 = ImageStorageUseCase::generate_object_key(owner_id, extension);

        // Different UUIDs should result in different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_object_key_with_different_owner_ids() {
        let extension = "-profile.jpg";

        let key1 = ImageStorageUseCase::generate_object_key(100, extension);
        let key2 = ImageStorageUseCase::generate_object_key(200, extension);

        // Keys should have different owner IDs in the path
        assert!(key1.contains("owner-100"));
        assert!(key2.contains("owner-200"));
    }

    #[test]
    fn test_generate_object_key_preserves_extension() {
        let owner_id = 1;

        let formats = vec![
            "-profile.jpg",
            "-cover.png",
            "-avatar.jpeg",
            "-thumbnail.gif",
        ];

        for format in formats {
            let object_key = ImageStorageUseCase::generate_object_key(owner_id, format);
            assert!(object_key.ends_with(format));
        }
    }

    // ========================
    // ImageType Enum Tests
    // ========================

    #[test]
    fn test_image_type_profile_variant() {
        let _profile = ImageType::Avatar;
        // Verify enum can be instantiated
        match _profile {
            ImageType::Avatar => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_image_type_cover_variant() {
        let _cover = ImageType::Cover;
        // Verify enum can be instantiated
        match _cover {
            ImageType::Cover => assert!(true),
            _ => assert!(false),
        }
    }

    // ========================
    // Utility Function Tests
    // ========================

    #[test]
    fn test_generate_object_key_large_owner_id() {
        let large_id = i32::MAX;
        let extension = "-profile.jpg";

        let object_key = ImageStorageUseCase::generate_object_key(large_id, extension);

        assert!(object_key.contains(&format!("owner-{}", large_id)));
        assert!(object_key.starts_with("uploads/"));
    }

    #[test]
    fn test_generate_object_key_small_owner_id() {
        let small_id = 1;
        let extension = "-profile.jpg";

        let object_key = ImageStorageUseCase::generate_object_key(small_id, extension);

        assert!(object_key.contains("owner-1"));
        assert!(object_key.starts_with("uploads/"));
    }

    #[test]
    fn test_generate_object_key_zero_owner_id() {
        let zero_id = 0;
        let extension = "-profile.jpg";

        let object_key = ImageStorageUseCase::generate_object_key(zero_id, extension);

        assert!(object_key.contains("owner-0"));
        assert!(object_key.ends_with(".jpg"));
    }

    #[test]
    fn test_generate_object_key_complex_extension() {
        let owner_id = 42;
        let complex_ext = "-profile-v2.jpg";

        let object_key = ImageStorageUseCase::generate_object_key(owner_id, complex_ext);

        assert!(object_key.ends_with(complex_ext));
        assert!(object_key.contains("owner-42"));
    }

    // ========================
    // Integration Tests (Mocking)
    // ========================

    #[test]
    fn test_image_storage_new() {
        let url = "https://example.com/image.jpg".to_string();
        let object_key = "uploads/owner-1/uuid.jpg".to_string();
        let owner_id = 1;

        let storage = ImageStorage::new(url.clone(), object_key.clone(), owner_id);

        assert_eq!(storage.url, url);
        assert_eq!(storage.object_key, object_key);
        assert_eq!(storage.owner_id, owner_id);
    }

    #[test]
    fn test_image_storage_debug_trait() {
        let storage = ImageStorage::new(
            "https://example.com/image.jpg".to_string(),
            "uploads/owner-1/uuid.jpg".to_string(),
            1,
        );

        let debug_str = format!("{:?}", storage);
        assert!(debug_str.contains("ImageStorage"));
    }
}
