use super::PersonUseCase;
use crate::commons::functions::string_to_uuid;
use crate::domain::enums::InviteStatus;
use crate::domain::friend::Friend;
use chrono::Utc;
use entity::person_address_entity as person_address;

fn make_address(id: i32, person_id: i32) -> person_address::PersonAddressEntity {
    person_address::PersonAddressEntity {
        id,
        uuid: string_to_uuid(format!("address-{id}").as_str()),
        person_id,
        address_line1: "".to_string(),
        address_line2: None,
        locality: "".to_string(),
        administrative_area: "Florianópolis".to_string(),
        postal_code: Some("88058573".to_string()),
        country_code: "BR".to_string(),
        current: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn make_friendship(id: i32, person_id: i32, friend_id: i32) -> Friend {
    Friend {
        id: Some(id),
        uuid: Some(format!("friendship-{id}")),
        person_id,
        friend_id,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
        status: InviteStatus::Accepted,
        person_uuid: format!("person-{person_id}"),
        friend_uuid: format!("person-{friend_id}"),
    }
}

#[test]
fn extract_neighbor_ids_excludes_current_person_by_person_id() {
    let addresses = vec![make_address(99, 1), make_address(100, 2)];

    let ids = PersonUseCase::extract_neighbor_ids(addresses, 1, &[]);

    assert_eq!(ids, vec![2]);
}

#[test]
fn extract_neighbor_ids_excludes_related_people() {
    let addresses = vec![make_address(1, 2), make_address(2, 3), make_address(3, 4)];

    let ids = PersonUseCase::extract_neighbor_ids(addresses, 1, &[3]);

    assert_eq!(ids, vec![2, 4]);
}

#[test]
fn extract_neighbor_ids_dedups_multiple_addresses_same_person() {
    let addresses = vec![make_address(1, 2), make_address(2, 2), make_address(3, 3)];

    let ids = PersonUseCase::extract_neighbor_ids(addresses, 1, &[]);

    assert_eq!(ids, vec![2, 3]);
}

#[test]
fn extract_accepted_friend_ids_keeps_both_directions_and_dedups() {
    let friendships = vec![
        make_friendship(1, 1, 2),
        make_friendship(2, 3, 1),
        make_friendship(3, 1, 2),
        make_friendship(4, 1, 1),
    ];

    let ids = PersonUseCase::extract_accepted_friend_ids(friendships, 1);

    assert_eq!(ids, vec![2, 3]);
}
