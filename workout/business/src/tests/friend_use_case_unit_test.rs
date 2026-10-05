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
    let result = FriendUseCase::normalize_accepted_friendships(vec![make_model(10, 1, 2)], 1);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].person_id, 1);
    assert_eq!(result[0].friend_id, 2);
}

#[test]
fn normalize_flips_inverse_friendship_counterpart() {
    let result = FriendUseCase::normalize_accepted_friendships(vec![make_model(10, 2, 1)], 1);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].person_id, 1);
    assert_eq!(result[0].friend_id, 2);
}

#[test]
fn normalize_dedups_direct_and_inverse_duplicates() {
    let result = FriendUseCase::normalize_accepted_friendships(
        vec![make_model(10, 1, 2), make_model(11, 2, 1)],
        1,
    );

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].friend_id, 2);
}

#[test]
fn normalize_filters_out_self_friendship() {
    let result = FriendUseCase::normalize_accepted_friendships(vec![make_model(10, 1, 1)], 1);

    assert!(result.is_empty());
}
