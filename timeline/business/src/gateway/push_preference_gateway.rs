use crate::commons::grpc_config::GrpcConfig;
use crate::proto::proto::settings::OwnerUuidRequest;
use crate::proto::proto::settings::settings_service_client::SettingsServiceClient;
use domain::business_error::BusinessError;
use tonic::metadata::MetadataValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushPreferenceError {
    NotFound,
    Transient,
    Permanent,
}

pub struct PushPreferenceGateway;

impl PushPreferenceGateway {
    pub async fn notifications_enabled(owner_uuid: &str) -> Result<bool, PushPreferenceError> {
        let secret = std::env::var("INTERNAL_SERVICE_SECRET")
            .ok()
            .filter(|value| !value.is_empty())
            .ok_or(PushPreferenceError::Permanent)?;
        let endpoint = GrpcConfig::build_endpoint();
        let channel = GrpcConfig::create_channel(&endpoint)
            .await
            .map_err(|_| PushPreferenceError::Transient)?;
        let secret = MetadataValue::try_from(secret).map_err(|_| PushPreferenceError::Permanent)?;
        let mut client = SettingsServiceClient::with_interceptor(
            channel,
            move |mut request: tonic::Request<()>| -> Result<_, tonic::Status> {
                request
                    .metadata_mut()
                    .insert("x-internal-secret", secret.clone());
                Ok(request)
            },
        );

        client
            .get_push_preference_by_owner_uuid(OwnerUuidRequest {
                owner_uuid: owner_uuid.to_string(),
            })
            .await
            .map(|response| response.into_inner().notifications_enabled)
            .map_err(|status| Self::classify_status(status.code()))
    }

    fn classify_status(code: tonic::Code) -> PushPreferenceError {
        match code {
            tonic::Code::NotFound => PushPreferenceError::NotFound,
            tonic::Code::Unavailable
            | tonic::Code::DeadlineExceeded
            | tonic::Code::ResourceExhausted
            | tonic::Code::Internal
            | tonic::Code::Unknown => PushPreferenceError::Transient,
            _ => PushPreferenceError::Permanent,
        }
    }

    pub fn into_business_error(error: PushPreferenceError) -> BusinessError {
        match error {
            PushPreferenceError::NotFound => BusinessError::not_found("Settings not found"),
            PushPreferenceError::Transient => {
                BusinessError::infrastructure("Settings service temporarily unavailable")
            }
            PushPreferenceError::Permanent => {
                BusinessError::unauthorized("Internal settings service authorization failed")
            }
        }
    }
}

#[cfg(test)]
#[path = "../tests/push_preference_gateway_unit_test.rs"]
mod tests;
