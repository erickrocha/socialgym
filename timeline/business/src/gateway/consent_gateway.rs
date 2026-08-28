use crate::commons::grpc_config::GrpcConfig;
use crate::proto::proto::person::ConsentStatusRequest;
use crate::proto::proto::person::person_service_client::PersonServiceClient;
use domain::business_error::BusinessError;

pub struct ConsentGateway;

impl ConsentGateway {
    pub async fn require(document: &str) -> Result<(), BusinessError> {
        let endpoint = GrpcConfig::build_endpoint();
        let channel = GrpcConfig::create_channel(&endpoint).await?;
        let mut client =
            PersonServiceClient::with_interceptor(channel, GrpcConfig::auth_interceptor);
        let status = client
            .has_active_consent(ConsentStatusRequest {
                document: document.to_string(),
            })
            .await
            .map_err(|e| {
                if matches!(
                    e.code(),
                    tonic::Code::PermissionDenied | tonic::Code::Unauthenticated
                ) {
                    BusinessError::forbidden(format!("{document} consent is required"))
                } else {
                    BusinessError::infrastructure(format!("failed to verify {document} consent"))
                }
            })?
            .into_inner();
        if status.active {
            Ok(())
        } else {
            Err(BusinessError::forbidden(format!(
                "{document} consent is required"
            )))
        }
    }

    pub async fn require_health_consent() -> Result<(), BusinessError> {
        Self::require("health_data").await
    }
}
