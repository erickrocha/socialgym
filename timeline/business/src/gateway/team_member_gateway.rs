use domain::business_error::BusinessError;

use crate::commons::grpc_config::GrpcConfig;
use crate::proto::proto::team_member::team_member_service_client::TeamMemberServiceClient;
use crate::proto::proto::team_member::TeamRosterRequest;

/// Everyone allowed inside a business profile's team chat, plus the business
/// identity used to attribute the owner's messages.
#[derive(Debug, Clone)]
pub struct TeamRoster {
    pub business_profile_id: i32,
    pub business_profile_uuid: String,
    pub business_profile_name: String,
    pub business_profile_logo_object_key: Option<String>,
    pub owner_person_uuid: String,
    pub accepted_member_person_uuids: Vec<String>,
}

impl TeamRoster {
    /// True when `person_uuid` may participate in this business profile's chat:
    /// the owner or any Accepted team member.
    pub fn allows(&self, person_uuid: &str) -> bool {
        self.owner_person_uuid == person_uuid
            || self
                .accepted_member_person_uuids
                .iter()
                .any(|u| u == person_uuid)
    }
}

pub struct TeamMemberGateway {
    endpoint: String,
}

impl TeamMemberGateway {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    pub async fn get_team_roster(
        &self,
        business_profile_uuid: &str,
    ) -> Result<TeamRoster, BusinessError> {
        let channel = GrpcConfig::create_channel(&self.endpoint).await?;
        let mut client =
            TeamMemberServiceClient::with_interceptor(channel, GrpcConfig::auth_interceptor);

        let response = client
            .get_team_roster(TeamRosterRequest {
                business_profile_uuid: business_profile_uuid.to_string(),
            })
            .await
            .map_err(|e| {
                log::error!(
                    "Error fetching team roster for '{}': {:?}",
                    business_profile_uuid,
                    e
                );
                match e.code() {
                    tonic::Code::Unauthenticated => {
                        BusinessError::unauthorized("Failed to fetch team roster")
                    }
                    tonic::Code::NotFound => BusinessError::not_found("Business profile not found"),
                    _ => BusinessError::new("Failed to fetch team roster".to_string()),
                }
            })?
            .into_inner();

        let logo = response.business_profile_logo_object_key.trim().to_string();

        Ok(TeamRoster {
            business_profile_id: response.business_profile_id,
            business_profile_uuid: response.business_profile_uuid,
            business_profile_name: response.business_profile_name,
            business_profile_logo_object_key: if logo.is_empty() { None } else { Some(logo) },
            owner_person_uuid: response.owner_person_uuid,
            accepted_member_person_uuids: response.accepted_member_person_uuids,
        })
    }
}
