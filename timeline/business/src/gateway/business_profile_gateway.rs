use crate::commons::grpc_config::GrpcConfig;
use crate::proto::proto::business_profile::BusinessProfileRequestId;
use crate::proto::proto::business_profile::business_profile_service_client::BusinessProfileServiceClient;
use domain::business_error::BusinessError;

pub struct BusinessProfileGateway;

impl BusinessProfileGateway {
    /// True when `uuid` is a Business Profile. NOT_FOUND means "it is a person", any other
    /// failure is a dependency outage and must not be read as "not a business profile".
    // ponytail: one lookup per call, add a short TTL cache if the call volume shows up.
    pub async fn exists(uuid: &str) -> Result<bool, BusinessError> {
        let channel = GrpcConfig::create_channel(&GrpcConfig::build_endpoint()).await?;
        let mut client =
            BusinessProfileServiceClient::with_interceptor(channel, GrpcConfig::auth_interceptor);
        match client
            .get_business_profile_by_id(BusinessProfileRequestId { id: 0, uuid: uuid.to_string() })
            .await
        {
            Ok(_) => Ok(true),
            Err(status) if status.code() == tonic::Code::NotFound => Ok(false),
            Err(status) => {
                log::error!("Business profile lookup failed: {}", status.code());
                Err(BusinessError::infrastructure("Business profile lookup failed"))
            }
        }
    }
}
