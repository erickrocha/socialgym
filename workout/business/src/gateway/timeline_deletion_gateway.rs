use crate::domain::business_error::BusinessError;
use std::env;
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::Request;

mod proto {
    tonic::include_proto!("grpc.timeline");
}
use proto::internal_service_client::InternalServiceClient;
use proto::PersonDataRequest;

const TIMELINE_GRPC_URL: &str = "TIMELINE_GRPC_URL";
const TIMELINE_GRPC_CERT_PATH: &str = "TIMELINE_GRPC_CERT_PATH";
const TIMELINE_GRPC_DOMAIN: &str = "TIMELINE_GRPC_DOMAIN";
const INTERNAL_SERVICE_SECRET: &str = "INTERNAL_SERVICE_SECRET";

/// Calls `timeline`'s internal gRPC service, authenticated by the shared service secret. An
/// `https` URL verifies the server with the CA in `TIMELINE_GRPC_CERT_PATH` (and the name in
/// `TIMELINE_GRPC_DOMAIN`); an `http` URL is plaintext, for tests only.
pub struct TimelineDeletionGateway {}

impl TimelineDeletionGateway {
    async fn client() -> Result<InternalServiceClient<Channel>, BusinessError> {
        let url =
            env::var(TIMELINE_GRPC_URL).unwrap_or_else(|_| "https://127.0.0.1:50052".to_string());
        let mut endpoint = Channel::from_shared(url.clone()).map_err(|e| {
            BusinessError::infrastructure(format!("Invalid {TIMELINE_GRPC_URL}: {e}"))
        })?;
        if url.starts_with("https://") {
            let mut tls = ClientTlsConfig::new();
            if let Ok(path) = env::var(TIMELINE_GRPC_CERT_PATH) {
                let pem = std::fs::read_to_string(&path).map_err(|e| {
                    BusinessError::infrastructure(format!("Cannot read {path}: {e}"))
                })?;
                tls = tls.ca_certificate(Certificate::from_pem(pem));
            }
            if let Ok(domain) = env::var(TIMELINE_GRPC_DOMAIN) {
                tls = tls.domain_name(domain);
            }
            endpoint = endpoint.tls_config(tls).map_err(|e| {
                BusinessError::infrastructure(format!("Invalid timeline TLS config: {e}"))
            })?;
        }
        let channel = endpoint
            .connect()
            .await
            .map_err(|e| BusinessError::infrastructure(format!("Timeline is unreachable: {e}")))?;
        Ok(InternalServiceClient::new(channel))
    }

    fn request(person_uuid: &str) -> Result<Request<PersonDataRequest>, BusinessError> {
        let secret = env::var(INTERNAL_SERVICE_SECRET).map_err(|_| {
            BusinessError::infrastructure("INTERNAL_SERVICE_SECRET is not configured")
        })?;
        let mut request = Request::new(PersonDataRequest {
            person_uuid: person_uuid.to_string(),
        });
        request.metadata_mut().insert(
            "x-internal-secret",
            MetadataValue::try_from(secret).map_err(|_| {
                BusinessError::infrastructure("INTERNAL_SERVICE_SECRET is not a valid header value")
            })?,
        );
        Ok(request)
    }

    /// Everything `timeline` holds for the person, for their data export.
    pub async fn export_person_data(person_uuid: &str) -> Result<serde_json::Value, BusinessError> {
        let request = Self::request(person_uuid)?;
        let response = Self::client()
            .await?
            .export_person_data(request)
            .await
            .map_err(|status| {
                log::error!("Timeline export failed: {}", status.code());
                BusinessError::infrastructure(format!("Timeline export failed: {}", status.code()))
            })?;
        serde_json::from_str(&response.into_inner().export_json)
            .map_err(|e| BusinessError::infrastructure(format!("Invalid timeline export: {e}")))
    }

    /// Cascade-deletes every document `timeline` holds for the person (posts, comments, reactions,
    /// check-ins, sessions, notifications, chat). Idempotent, so a failed purge can retry it.
    pub async fn delete_person_data(person_uuid: &str) -> Result<(), BusinessError> {
        let request = Self::request(person_uuid)?;
        Self::client()
            .await?
            .delete_person_data(request)
            .await
            .map_err(|status| {
                log::error!("Timeline account deletion failed: {}", status.code());
                BusinessError::infrastructure(
                    "Failed to delete timeline data for account".to_string(),
                )
            })?;
        Ok(())
    }
}
