use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::uuid_to_string;
use crate::domain::business_error::BusinessError;
use crate::domain::friend::{Friend, FriendEntityMapper, FriendStatus};
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::person_gateway::PersonGateway;
use entity::friends_entity as friends;
use sea_orm::DbConn;

pub struct FriendUseCase {}

impl FriendUseCase {
    pub async fn send_friend_request(
        db: &DbConn,
        sender_id: i32,
        receiver_id: i32,
    ) -> Result<Friend, BusinessError> {
        log::info!(
            "Attempting to send friend request from sender_id: {:?} to receiver_id: {:?}",
            sender_id,
            receiver_id
        );

        if sender_id == receiver_id {
            log::error!(
                "Cannot send friend request to yourself sender_id: {:?}, receiver_id: {:?}",
                sender_id,
                receiver_id
            );
            return Err(BusinessError::new(
                "Cannot send friend request to yourself".to_string(),
            ));
        }

        let existing = FriendGateway::find_friend_request(db, sender_id, receiver_id)
            .await
            .map_err(|e| {
                BusinessError::new(format!("Error checking existing friend request: {:?}", e))
            })?;

        if let Some(existing_request) = existing {
            let status = FriendStatus::from_string(&existing_request.status);
            return match status {
                FriendStatus::Pending => {
                    if existing_request.person_id == receiver_id
                        && existing_request.friend_id == sender_id
                    {
                        log::info!(
                            "Receiver {:?} already sent a pending request to sender {:?}. Auto-accepting.",
                            receiver_id,
                            sender_id
                        );
                        let mut req = existing_request;
                        req.status = FriendStatus::Accepted.as_str().to_string();
                        let result = FriendGateway::update(db, FriendEntityMapper::from_model(req))
                            .await
                            .map_err(|e| {
                                BusinessError::new(format!(
                                    "Error auto-accepting friend request: {:?}",
                                    e
                                ))
                            })?;
                        Ok(FriendEntityMapper::from_model(result))
                    } else {
                        log::warn!(
                            "A pending friend request already exists from sender {:?} to receiver {:?}",
                            sender_id,
                            receiver_id
                        );
                        Err(BusinessError::new(
                            "A pending friend request already exists".to_string(),
                        ))
                    }
                }
                FriendStatus::Accepted => {
                    log::warn!(
                        "Users {:?} and {:?} are already friends",
                        sender_id,
                        receiver_id
                    );
                    Err(BusinessError::new("You are already friends".to_string()))
                }
                FriendStatus::Rejected | FriendStatus::Cancelled => {
                    log::info!(
                        "Previous request was '{}'. Allowing new friend request from {:?} to {:?}.",
                        existing_request.status,
                        sender_id,
                        receiver_id
                    );
                    Self::create_new_request(db, sender_id, receiver_id).await
                }
            };
        }

        Self::create_new_request(db, sender_id, receiver_id).await
    }

    async fn create_new_request(
        db: &DbConn,
        sender_id: i32,
        receiver_id: i32,
    ) -> Result<Friend, BusinessError> {
        let sender = PersonGateway::find_by_id(db, sender_id)
            .await
            .ok_or_else(|| BusinessError::new("Sender person not found".to_string()))?;

        let receiver = PersonGateway::find_by_id(db, receiver_id)
            .await
            .ok_or_else(|| BusinessError::new("Receiver person not found".to_string()))?;

        let friend_request = Friend::new(
            sender_id,
            receiver_id,
            uuid_to_string(sender.uuid),
            uuid_to_string(receiver.uuid),
            FriendStatus::Pending,
        );
        let result = FriendGateway::persist(db, friend_request).await;
        if result.is_err() {
            log::error!("Error sending friend request: {:?}", result.err());
            return Err(BusinessError::new(
                "Error sending friend request".to_string(),
            ));
        }
        log::info!("Friend request sent successfully: {:?}", result);
        Ok(FriendEntityMapper::from_active_model(result.unwrap()))
    }

    pub async fn accept_friend_request(
        db: &DbConn,
        person_id: i32,
        friend_id: i32,
    ) -> Result<Friend, BusinessError> {
        log::info!(
            "Attempting to accept friend request with person id: {:?} and friend id {:?}",
            person_id,
            friend_id
        );
        let result =
            Self::update_friend_request(db, person_id, friend_id, FriendStatus::Accepted).await;
        if result.is_err() {
            log::error!("Error accepting friend request: {:?}", result.err());
            return Err(BusinessError::new(
                "Error accepting friend request".to_string(),
            ));
        }
        log::info!("Friend request accepted successfully: {:?}", result);
        result
    }

    async fn update_friend_request(
        db: &DbConn,
        person_id: i32,
        friend_id: i32,
        status: FriendStatus,
    ) -> Result<Friend, BusinessError> {
        let friend_request = FriendGateway::find_friend_request(db, person_id, friend_id).await;
        if friend_request.is_err() {
            log::error!("Error finding friend request: {:?}", friend_request.err());
            return Err(BusinessError::new(
                "Error finding friend request".to_string(),
            ));
        }
        let friend_request = friend_request.unwrap();
        if friend_request.is_none() {
            log::error!(
                "Friend request not found with person id: {:?} and friend id {:?}",
                person_id,
                friend_id
            );
            return Err(BusinessError::new("Friend request not found".to_string()));
        }
        let mut friend_request = friend_request.unwrap();
        friend_request.status = status.as_str().to_string();
        let result =
            FriendGateway::update(db, FriendEntityMapper::from_model(friend_request)).await;
        if result.is_err() {
            log::error!("Error updating friend request: {:?}", result.err());
            return Err(BusinessError::new(
                "Error updating friend request".to_string(),
            ));
        }
        log::info!("Friend request updated successfully: {:?}", result);
        Ok(FriendEntityMapper::from_model(result.unwrap()))
    }

    pub async fn deny_friend_request(
        db: &DbConn,
        person_id: i32,
        friend_id: i32,
    ) -> Result<Friend, BusinessError> {
        log::info!(
            "Attempting to deny friend request with person id: {:?} and friend id {:?}",
            person_id,
            friend_id
        );
        let result =
            Self::update_friend_request(db, person_id, friend_id, FriendStatus::Rejected).await;
        if result.is_err() {
            log::error!("Error denying friend request: {:?}", result.err());
            return Err(BusinessError::new(
                "Error denying friend request".to_string(),
            ));
        }
        log::info!("Friend request denied successfully: {:?}", result);
        result
    }

    pub async fn cancel_friend_request(
        db: &DbConn,
        person_id: i32,
        friend_id: i32,
    ) -> Result<Friend, BusinessError> {
        log::info!(
            "Attempting to cancel friend request with person id: {:?}",
            person_id
        );
        let result =
            Self::update_friend_request(db, person_id, friend_id, FriendStatus::Cancelled).await;
        if result.is_err() {
            log::error!("Error cancelling friend request: {:?}", result.err());
            return Err(BusinessError::new(
                "Error cancelling friend request".to_string(),
            ));
        }
        log::info!("Friend request cancelled successfully: {:?}", result);
        result
    }

    pub async fn find_all_friend(
        db: &DbConn,
        person_id: i32,
    ) -> Result<Vec<Friend>, BusinessError> {
        log::info!(
            "Attempting to find all friends for person_id: {:?}",
            person_id
        );
        let friends = FriendGateway::find_all_accepted_friends(db, person_id).await;
        if friends.is_err() {
            log::error!("Error finding friends: {:?}", friends.err());
            return Err(BusinessError::new("Error finding friends".to_string()));
        }

        let friend_list = Self::normalize_accepted_friendships(friends.unwrap(), person_id);

        log::info!("Friends found successfully: {:?}", friend_list);
        Ok(friend_list)
    }

    fn normalize_accepted_friendships(records: Vec<friends::FriendsEntity>, person_id: i32) -> Vec<Friend> {
        let mut friend_list: Vec<Friend> = records
            .into_iter()
            .filter_map(|f| {
                let friend_id = if f.person_id == person_id {
                    f.friend_id
                } else if f.friend_id == person_id {
                    f.person_id
                } else {
                    return None;
                };
                if friend_id == person_id {
                    return None;
                }

                let mut friend = FriendEntityMapper::from_model(f);
                if friend.person_id != person_id {
                    std::mem::swap(&mut friend.person_id, &mut friend.friend_id);
                    std::mem::swap(&mut friend.person_uuid, &mut friend.friend_uuid);
                }
                Some(friend)
            })
            .collect();

        friend_list.sort_by_key(|f| f.friend_id);
        friend_list.dedup_by_key(|f| f.friend_id);
        friend_list
    }
}

#[cfg(test)]
mod tests {
    use super::FriendUseCase;
    use crate::commons::functions::string_to_uuid;
    use chrono::Utc;
    use entity::friends_entity as friends;

    fn make_model(id: i32, person_id: i32, friend_id: i32) -> friends::FriendsEntity {
        friends::FriendsEntity {
            id,
            uuid: string_to_uuid(format!("uuid-{id}").as_str()),
            person_id,
            friend_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: "Accepted".to_string(),
            person_uuid: string_to_uuid(format!("person-{person_id}").as_str()),
            friend_uuid: string_to_uuid(format!("person-{friend_id}").as_str()),
        }
    }

    #[test]
    fn normalize_keeps_direct_friendship_counterpart() {
        let person_id = 1;
        let result =
            FriendUseCase::normalize_accepted_friendships(vec![make_model(10, 1, 2)], person_id);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].person_id, 1);
        assert_eq!(result[0].friend_id, 2);
    }

    #[test]
    fn normalize_flips_inverse_friendship_counterpart() {
        let person_id = 1;
        let result =
            FriendUseCase::normalize_accepted_friendships(vec![make_model(10, 2, 1)], person_id);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].person_id, 1);
        assert_eq!(result[0].friend_id, 2);
    }

    #[test]
    fn normalize_dedups_direct_and_inverse_duplicates() {
        let person_id = 1;
        let result = FriendUseCase::normalize_accepted_friendships(
            vec![make_model(10, 1, 2), make_model(11, 2, 1)],
            person_id,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].friend_id, 2);
    }

    #[test]
    fn normalize_filters_out_self_friendship() {
        let person_id = 1;
        let result =
            FriendUseCase::normalize_accepted_friendships(vec![make_model(10, 1, 1)], person_id);

        assert!(result.is_empty());
    }
}
