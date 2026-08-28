use crate::domain::business_error::BusinessError;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use std::time::Duration;

pub struct S3Gateway {}

impl S3Gateway {
    pub async fn upload_bytes(
        client: &Client,
        bucket: &str,
        object: &str,
        bytes: Vec<u8>,
        content_type: &str,
    ) -> Result<(), BusinessError> {
        client
            .put_object()
            .bucket(bucket)
            .key(object)
            .content_type(content_type)
            .server_side_encryption(aws_sdk_s3::types::ServerSideEncryption::Aes256)
            .body(ByteStream::from(bytes))
            .send()
            .await
            .map(|_| ())
            .map_err(|e| BusinessError::infrastructure(format!("Failed to upload export: {e:?}")))
    }

    pub async fn download_bytes(
        client: &Client,
        bucket: &str,
        object: &str,
    ) -> Result<Vec<u8>, BusinessError> {
        let output = client
            .get_object()
            .bucket(bucket)
            .key(object)
            .send()
            .await
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to download object: {e:?}"))
            })?;
        output
            .body
            .collect()
            .await
            .map(|data| data.into_bytes().to_vec())
            .map_err(|e| BusinessError::infrastructure(format!("Failed to read object: {e:?}")))
    }
    pub async fn put_object(
        client: &Client,
        bucket: &str,
        object: &str,
        content_type: &str,
        expires_in: Duration,
    ) -> Result<String, BusinessError> {
        let expires_in: PresigningConfig =
            PresigningConfig::expires_in(expires_in).map_err(|err| {
                BusinessError::new(format!(
                    "Failed to convert expiration to PresigningConfig: {err:?}"
                ))
            })?;
        let presigned_request = client
            .put_object()
            .bucket(bucket)
            .key(object)
            .content_type(content_type)
            .presigned(expires_in)
            .await;

        if presigned_request.is_err() {
            return Err(BusinessError::new(format!(
                "Failed to generate pre-signed URL: {presigned_request:?}"
            )));
        }
        let presigned_request = presigned_request.unwrap();
        Ok(presigned_request.uri().into())
    }

    pub async fn get_object(
        client: &Client,
        bucket: &str,
        object: &str,
        expires_in: Duration,
    ) -> Result<String, BusinessError> {
        let expires_in: PresigningConfig =
            PresigningConfig::expires_in(expires_in).map_err(|err| {
                BusinessError::new(format!(
                    "Failed to convert expiration to PresigningConfig: {err:?}"
                ))
            })?;
        let presigned_request = client
            .get_object()
            .bucket(bucket)
            .key(object)
            .presigned(expires_in)
            .await;

        if presigned_request.is_err() {
            return Err(BusinessError::new(format!(
                "Failed to generate pre-signed URL: {presigned_request:?}"
            )));
        }
        let presigned_request = presigned_request.unwrap();
        Ok(presigned_request.uri().into())
    }

    pub async fn delete_object(
        client: &Client,
        bucket: &str,
        object: &str,
    ) -> Result<(), BusinessError> {
        let result = client
            .delete_object()
            .bucket(bucket)
            .key(object)
            .send()
            .await;
        if let Err(e) = result {
            return Err(BusinessError::new(format!(
                "Failed to delete object from S3: {e:?}"
            )));
        }
        Ok(())
    }

    /// Performs a HeadObject call and returns the stored `Content-Type` metadata.
    /// Falls back to `"application/octet-stream"` when the header is absent.
    pub async fn head_object(
        client: &Client,
        bucket: &str,
        object: &str,
    ) -> Result<String, BusinessError> {
        let result = client
            .head_object()
            .bucket(bucket)
            .key(object)
            .send()
            .await
            .map_err(|e| {
                BusinessError::new(format!("HeadObject failed for key '{object}': {e:?}"))
            })?;
        Ok(result
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string())
    }
}
