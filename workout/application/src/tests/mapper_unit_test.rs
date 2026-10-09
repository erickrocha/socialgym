use super::*;
use business::domain::enums::{InviteStatus, Position, WeightUnit};

#[test]
fn country_round_trips_through_json() {
    let country = Country {
        id: Some(7),
        ddi: "+55".to_string(),
        name: "Brazil".to_string(),
        acronym: "BR".to_string(),
        currency: "BRL".to_string(),
    };

    let json = CountryMapper::json(country);
    assert_eq!(json.name.as_deref(), Some("Brazil"));

    let back = CountryMapper::domain(json);
    assert_eq!(back.id, Some(7));
    assert_eq!(back.ddi, "+55");
    assert_eq!(back.currency, "BRL");
}

#[test]
fn address_candidate_round_trips_through_json() {
    let candidate = AddressCandidate {
        place_id: "place-1".to_string(),
        formatted_address: "1 Main St".to_string(),
        address_line1: "1 Main St".to_string(),
        address_line2: Some("Apt 2".to_string()),
        locality: "Springfield".to_string(),
        administrative_area: "State".to_string(),
        administrative_area_code: "ST".to_string(),
        postal_code: None,
        country_code: "BR".to_string(),
        latitude: -23.5,
        longitude: -46.6,
    };

    let json = AddressCandidateMapper::json(candidate);
    assert_eq!(json.place_id, "place-1");
    assert_eq!(json.address_line2.as_deref(), Some("Apt 2"));

    let back = AddressCandidateMapper::domain(json);
    assert_eq!(back.locality, "Springfield");
    assert_eq!(back.postal_code, None);
    assert_eq!((back.latitude, back.longitude), (-23.5, -46.6));
}

#[test]
fn settings_round_trip_converts_position_and_weight_unit() {
    let settings = Settings {
        id: Some(1),
        uuid: Some("uuid-1".to_string()),
        person_id: 3,
        person_uuid: "person-uuid".to_string(),
        language: "en".to_string(),
        theme: "dark".to_string(),
        notifications_enabled: true,
        context_menu_position: Position::Top,
        home_page: "feed".to_string(),
        weight_unit: Some(WeightUnit::Kilograms),
        created_at: None,
        updated_at: None,
    };

    let json = SettingsMapper::json(settings);
    assert_eq!(json.context_menu_position, Position::Top.to_string());
    assert_eq!(json.weight_unit, Some(WeightUnit::Kilograms.to_string()));

    let back = SettingsMapper::domain(json);
    assert!(matches!(back.context_menu_position, Position::Top));
    assert!(matches!(back.weight_unit, Some(WeightUnit::Kilograms)));
    assert!(back.notifications_enabled);
}

#[test]
fn settings_without_weight_unit_stays_empty() {
    let settings = Settings {
        id: None,
        uuid: None,
        person_id: 1,
        person_uuid: "p".to_string(),
        language: "pt".to_string(),
        theme: "light".to_string(),
        notifications_enabled: false,
        context_menu_position: Position::Left,
        home_page: "workouts".to_string(),
        weight_unit: None,
        created_at: None,
        updated_at: None,
    };

    let back = SettingsMapper::domain(SettingsMapper::json(settings));

    assert!(back.weight_unit.is_none());
    assert!(!back.notifications_enabled);
}

#[test]
fn team_member_round_trip_preserves_status() {
    let member = TeamMember {
        id: Some(4),
        uuid: Some("tm-uuid".to_string()),
        business_profile_id: 9,
        business_profile_uuid: "bp-uuid".to_string(),
        person_id: 2,
        person_uuid: "person-uuid".to_string(),
        created_at: None,
        updated_at: None,
        status: InviteStatus::Accepted,
    };

    let json = TeamMemberMapper::json(member);
    assert_eq!(json.status, InviteStatus::Accepted.as_str());

    let back = TeamMemberMapper::domain(json);
    assert!(matches!(back.status, InviteStatus::Accepted));
    assert_eq!(back.business_profile_id, 9);
}

#[test]
fn collection_helpers_map_every_element() {
    let countries = vec![
        Country {
            id: Some(1),
            ddi: "+1".into(),
            name: "A".into(),
            acronym: "A".into(),
            currency: "X".into(),
        },
        Country {
            id: Some(2),
            ddi: "+2".into(),
            name: "B".into(),
            acronym: "B".into(),
            currency: "Y".into(),
        },
    ];

    let json = CountryMapper::json_vec(countries);
    assert_eq!(json.len(), 2);
    assert!(CountryMapper::json_opt(None).is_none());

    let back = CountryMapper::domain_vec(json);
    assert_eq!(
        back.iter().map(|c| c.id).collect::<Vec<_>>(),
        vec![Some(1), Some(2)]
    );
}
