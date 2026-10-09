use super::*;

fn domain_friend(id: Option<i32>, uuid: Option<&str>, status: InviteStatus) -> DomainFriend {
    DomainFriend {
        id,
        uuid: uuid.map(str::to_string),
        person_id: 1,
        person_uuid: "person-uuid".to_string(),
        friend_id: 2,
        friend_uuid: "friend-uuid".to_string(),
        created_at: None,
        updated_at: None,
        status,
    }
}

#[test]
fn friend_response_carries_ids_and_status() {
    let response = FriendMapper::response(domain_friend(
        Some(5),
        Some("f-uuid"),
        InviteStatus::Accepted,
    ));

    assert_eq!(response.id, 5);
    assert_eq!(response.uuid, "f-uuid");
    assert_eq!(response.status, InviteStatus::Accepted.as_str());
    assert_eq!((response.person_id, response.friend_id), (1, 2));
}

#[test]
fn friend_response_defaults_a_missing_id_to_zero() {
    let response =
        FriendMapper::response(domain_friend(None, Some("f-uuid"), InviteStatus::Pending));

    assert_eq!(response.id, 0);
}

#[test]
fn friend_domain_treats_zero_id_and_empty_uuid_as_absent() {
    let friend = FriendMapper::domain(proto::friend::Friend {
        id: 0,
        uuid: String::new(),
        person_id: 1,
        person_uuid: "p".to_string(),
        friend_id: 2,
        friend_uuid: "f".to_string(),
        status: String::new(),
    });

    assert_eq!(friend.id, None);
    assert_eq!(friend.uuid, None);
    // An empty status is a freshly created request.
    assert!(matches!(friend.status, InviteStatus::Pending));
}

#[test]
fn friend_domain_keeps_a_present_id_uuid_and_status() {
    let friend = FriendMapper::domain(proto::friend::Friend {
        id: 9,
        uuid: "u".to_string(),
        person_id: 1,
        person_uuid: "p".to_string(),
        friend_id: 2,
        friend_uuid: "f".to_string(),
        status: InviteStatus::Accepted.as_str().to_string(),
    });

    assert_eq!(friend.id, Some(9));
    assert_eq!(friend.uuid.as_deref(), Some("u"));
    assert!(matches!(friend.status, InviteStatus::Accepted));
}

#[test]
fn country_round_trips() {
    let response = CountryMapper::response(DomainCountry {
        id: Some(3),
        ddi: "+55".to_string(),
        name: "Brazil".to_string(),
        acronym: "BR".to_string(),
        currency: "BRL".to_string(),
    });
    assert_eq!(response.id, 3);

    let back = CountryMapper::domain(response);
    assert_eq!(back.id, Some(3));
    assert_eq!(back.acronym, "BR");
}

#[test]
fn team_member_response_and_domain_handle_absent_ids() {
    let response = TeamMemberMapper::response(DomainTeamMember {
        id: None,
        uuid: None,
        business_profile_id: 4,
        business_profile_uuid: "bp".to_string(),
        person_id: 2,
        person_uuid: "p".to_string(),
        created_at: None,
        updated_at: None,
        status: InviteStatus::Pending,
    });
    assert_eq!(response.id, 0);
    assert_eq!(response.uuid, "");

    let back = TeamMemberMapper::domain(response);
    assert_eq!(back.id, None);
    assert_eq!(back.uuid, None);
    assert!(matches!(back.status, InviteStatus::Pending));
}

fn domain_settings(
    weight_unit: Option<WeightUnit>,
    created_at: Option<NaiveDateTime>,
) -> DomainSettings {
    DomainSettings {
        id: Some(1),
        uuid: Some("s-uuid".to_string()),
        person_id: 3,
        person_uuid: "p-uuid".to_string(),
        language: "en".to_string(),
        theme: "dark".to_string(),
        notifications_enabled: true,
        context_menu_position: Position::Top,
        home_page: "feed".to_string(),
        weight_unit,
        created_at,
        updated_at: None,
    }
}

#[test]
fn settings_round_trip_preserves_weight_unit_and_timestamps() {
    let created =
        NaiveDateTime::parse_from_str("2026-10-05 12:30:45.123", "%Y-%m-%d %H:%M:%S%.f").unwrap();

    let response =
        SettingsMapper::response(domain_settings(Some(WeightUnit::Pounds), Some(created)));
    assert_eq!(response.owner_id, 3);
    assert_eq!(response.weight_unit, WeightUnit::Pounds.to_string());
    assert_eq!(response.updated_at, "");

    let back = SettingsMapper::domain(response);
    assert_eq!(back.created_at, Some(created));
    assert_eq!(back.updated_at, None);
    assert!(matches!(back.weight_unit, Some(WeightUnit::Pounds)));
    assert!(matches!(back.context_menu_position, Position::Top));
}

#[test]
fn settings_without_weight_unit_or_timestamps_map_to_empty_and_back_to_none() {
    let response = SettingsMapper::response(domain_settings(None, None));
    assert_eq!(response.weight_unit, "");
    assert_eq!(response.created_at, "");

    let back = SettingsMapper::domain(response);
    assert!(back.weight_unit.is_none());
    assert_eq!(back.created_at, None);
}

#[test]
fn settings_with_zero_id_map_to_no_id() {
    let mut response = SettingsMapper::response(domain_settings(None, None));
    response.id = 0;

    assert_eq!(SettingsMapper::domain(response).id, None);
}

#[test]
fn response_and_domain_helpers_map_collections_and_options() {
    let countries = vec![
        DomainCountry {
            id: Some(1),
            ddi: "1".into(),
            name: "A".into(),
            acronym: "A".into(),
            currency: "X".into(),
        },
        DomainCountry {
            id: Some(2),
            ddi: "2".into(),
            name: "B".into(),
            acronym: "B".into(),
            currency: "Y".into(),
        },
    ];

    let responses = CountryMapper::response_vec(countries);
    assert_eq!(
        responses.iter().map(|c| c.id).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(CountryMapper::domain_vec(responses.clone()).len(), 2);
    assert!(CountryMapper::response_option(None).is_none());
    assert!(CountryMapper::domain_option(responses.into_iter().next()).is_some());
}
