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
            status: String::new(),
        },
        Friend {
            id: 2,
            uuid: "uuid-2".to_string(),
            person_id: 30,
            friend_id: 10,
            person_uuid: "friend-2".to_string(),
            friend_uuid: "me".to_string(),
            status: String::new(),
        },
        Friend {
            id: 3,
            uuid: "uuid-3".to_string(),
            person_id: 10,
            friend_id: 20,
            person_uuid: "me".to_string(),
            friend_uuid: "friend-1".to_string(),
            status: String::new(),
        },
    ];

    let result = friend_uuids_from_response("me", friends);

    assert_eq!(result, vec!["friend-1".to_string(), "friend-2".to_string()]);
}
