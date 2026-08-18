use domain::business_error::BusinessError;
use std::collections::HashSet;

pub mod proto {

    pub mod business_profile {
        tonic::include_proto!("grpc.business_profile");
    }

    pub mod business_profile_address {
        tonic::include_proto!("grpc.business_profile_address");
    }

    pub mod country {
        tonic::include_proto!("grpc.country");
    }

    pub mod exercise {
        tonic::include_proto!("grpc.exercise");
    }

    pub mod friend {
        tonic::include_proto!("grpc.friend");
    }

    pub mod person {
        tonic::include_proto!("grpc.person");
    }

    pub mod person_address {
        tonic::include_proto!("grpc.person_address");
    }

    pub mod person_info {
        tonic::include_proto!("grpc.person_info");
    }

    pub mod settings {
        tonic::include_proto!("grpc.settings");
    }
    pub mod user {
        tonic::include_proto!("grpc.user");
    }

    pub mod workout {
        tonic::include_proto!("grpc.workout");
    }
}

use proto::friend::friend_service_client::FriendServiceClient;
use proto::friend::{Friend, FriendsRequest};
use crate::commons::grpc_config::GrpcConfig;

pub struct FriendGateway{
    endpoint: String
}

impl FriendGateway {

    pub fn new(endpoint: String) -> Self {
        Self{endpoint}
    }
    pub async fn find_friend_uuids(&self,person_id: i32,person_uuid: &str) -> Result<Vec<String>, BusinessError> {
        let channel = GrpcConfig::create_channel(&self.endpoint).await?;

        let mut client = FriendServiceClient::with_interceptor(channel, GrpcConfig::auth_interceptor);
        let response = client
            .get_friends(FriendsRequest {
                id: person_id,
                uuid: person_uuid.to_string(),
            })
            .await
            .map_err(|e| {
                log::error!("Error fetching friends for '{}': {:?}", person_uuid, e);
                BusinessError::new("Failed to fetch friends".to_string())
            })?;

        Ok(friend_uuids_from_response(
            person_uuid,
            response.into_inner().friends,
        ))
    }
}

fn friend_uuids_from_response(current_person_uuid: &str, friends: Vec<Friend>) -> Vec<String> {
    let mut seen = HashSet::new();

    friends
        .into_iter()
        .filter_map(|friend| {
            let candidate = if friend.person_uuid == current_person_uuid {
                friend.friend_uuid
            } else {
                friend.person_uuid
            };

            let normalized = candidate.trim();
            if normalized.is_empty() || normalized == current_person_uuid {
                return None;
            }

            let normalized = normalized.to_string();
            if seen.insert(normalized.clone()) {
                Some(normalized)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_only_other_side_uuid_and_deduplicates() {
        let friends = vec![
            Friend {
                id: 1,
                uuid: "uuid-1".to_string(),
                person_id: 10,
                friend_id: 20,
                person_uuid: "me".to_string(),
                friend_uuid: "friend-1".to_string(),
            },
            Friend {
                id: 2,
                uuid: "uuid-2".to_string(),
                person_id: 30,
                friend_id: 10,
                person_uuid: "friend-2".to_string(),
                friend_uuid: "me".to_string(),
            },
            Friend {
                id: 3,
                uuid: "uuid-3".to_string(),
                person_id: 10,
                friend_id: 20,
                person_uuid: "me".to_string(),
                friend_uuid: "friend-1".to_string(),
            },
        ];

        let result = friend_uuids_from_response("me", friends);

        assert_eq!(result, vec!["friend-1".to_string(), "friend-2".to_string()]);
    }
}

