use crate::commons::grpc_config::GrpcConfig;
use crate::proto::proto::person::RoleStatusRequest;
use crate::proto::proto::person::person_service_client::PersonServiceClient;
use domain::business_error::BusinessError;

pub struct RoleGateway;
impl RoleGateway {
    pub async fn require(role: &str) -> Result<(), BusinessError> {
        let channel = GrpcConfig::create_channel(&GrpcConfig::build_endpoint()).await?;
        let mut client =
            PersonServiceClient::with_interceptor(channel, GrpcConfig::auth_interceptor);
        let status = client
            .has_role(RoleStatusRequest {
                role: role.to_string(),
            })
            .await
            .map_err(|_| BusinessError::forbidden("moderator role is required"))?
            .into_inner();
        if status.active {
            Ok(())
        } else {
            Err(BusinessError::forbidden("moderator role is required"))
        }
    }
}
