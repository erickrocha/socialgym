mod support;

use business::domain::business_error::BusinessErrorKind as K;
use business::domain::person::Person;
use business::use_cases::friend_use_case::FriendUseCase;
use business::use_cases::person_use_case::PersonUseCase;
use chrono::NaiveDate;
use sea_orm::{ConnectionTrait, DatabaseConnection};
use support::{business_profile, fresh_db, kind, register};

async fn address(db: &DatabaseConnection, person_id: i32, lon: f64, lat: f64) {
    db.execute_unprepared(&format!(
        "INSERT INTO person_address (uuid, person_id, address_line1, locality, administrative_area, country_code, current, location, created_at, updated_at)
         VALUES (gen_random_uuid(), {person_id}, 'Street', 'City', 'SP', 'BR', true, ST_SetSRID(ST_MakePoint({lon}, {lat}), 4326)::geography, now(), now())"
    ))
    .await
    .unwrap();
}

fn ids(persons: &[Person]) -> Vec<i32> {
    let mut v: Vec<i32> = persons.iter().filter_map(|p| p.id).collect();
    v.sort();
    v
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn person_lifecycle_validation_and_lookups() {
    let db = fresh_db().await;
    let owner = register(&db, 1).await;

    // add: required and bounded fields
    let dob = NaiveDate::from_ymd_opt(1991, 2, 3).unwrap();
    assert!(PersonUseCase::add(&db, Person::new("".into(), "S".into(), dob, "X".into())).await.is_err());
    assert!(PersonUseCase::add(&db, Person::new("F".into(), "S".into(), dob, "".into())).await.is_err());
    assert!(matches!(kind(&PersonUseCase::add(&db, Person::new("F".repeat(256), "S".into(), dob, "X".into())).await.unwrap_err()), K::Validation));
    let added = PersonUseCase::add(&db, Person::new("Zed".into(), "Zulu".into(), dob, "X".into())).await.unwrap();
    assert!(added.person_info.is_some(), "a person is created together with its info row");

    // get: unknown id, then the full aggregate (info, addresses, business profiles)
    assert!(matches!(kind(&PersonUseCase::get(&db, 9999).await.unwrap_err()), K::NotFound));
    address(&db, owner.person_id, -46.63, -23.55).await;
    let profile = business_profile(&db, &owner, "Gym").await;
    let loaded = PersonUseCase::get(&db, owner.person_id).await.unwrap();
    assert_eq!(loaded.addresses.len(), 1);
    assert_eq!(loaded.business_profiles.len(), 1);
    assert_eq!(loaded.business_profiles[0].id, profile.id);
    assert!(loaded.person_info.is_some());

    // update needs the info block and keeps the aggregate consistent
    let mut edit = loaded.clone();
    edit.person_info = None;
    assert!(PersonUseCase::update(&db, edit).await.is_err());
    let mut edit = loaded.clone();
    edit.surname = "Renamed".into();
    let updated = PersonUseCase::update(&db, edit).await.unwrap();
    assert_eq!((updated.surname.as_str(), updated.addresses.len()), ("Renamed", 1));
    assert_eq!(PersonUseCase::get(&db, owner.person_id).await.unwrap().surname, "Renamed");

    // by uuid
    assert_eq!(PersonUseCase::find_by_uuid(&db, owner.person_uuid.clone()).await.unwrap().id, Some(owner.person_id));
    assert!(PersonUseCase::find_by_uuid(&db, "00000000-0000-0000-0000-00000000dead".into()).await.is_err());

    // ownership guard
    assert!(PersonUseCase::require_owner_access(1, 1).is_ok());
    assert!(matches!(kind(&PersonUseCase::require_owner_access(1, 2).unwrap_err()), K::Forbidden));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn friend_lists_search_and_suggestions_respect_relationships_and_limits() {
    let db = fresh_db().await;
    let (me, near, far, requested, friend) = (register(&db, 1).await, register(&db, 2).await, register(&db, 3).await, register(&db, 4).await, register(&db, 5).await);
    address(&db, me.person_id, -46.6333, -23.5505).await;
    address(&db, near.person_id, -46.6333, -23.5405).await;
    address(&db, far.person_id, -46.6333, -22.5505).await;
    address(&db, requested.person_id, -46.6333, -23.5455).await;
    address(&db, friend.person_id, -46.6333, -23.5455).await;

    // requests: me -> requested (pending); friend -> me (accepted)
    FriendUseCase::send_friend_request(&db, me.person_id, requested.person_id).await.unwrap();
    FriendUseCase::send_friend_request(&db, friend.person_id, me.person_id).await.unwrap();
    assert!(PersonUseCase::get_all_friends(&db, me.person_id).await.is_empty());
    assert_eq!(ids(&PersonUseCase::get_all_received_requests(&db, me.person_id).await), vec![friend.person_id]);
    assert_eq!(ids(&PersonUseCase::get_all_sent_requests(&db, me.person_id).await), vec![requested.person_id]);
    FriendUseCase::accept_friend_request(&db, me.person_id, friend.person_id).await.unwrap();
    assert_eq!(ids(&PersonUseCase::get_all_friends(&db, me.person_id).await), vec![friend.person_id]);
    assert_eq!(ids(&PersonUseCase::get_all_friends(&db, friend.person_id).await), vec![me.person_id]);

    // text search: empty -> nothing; by name; by email; never the caller; limit
    assert!(PersonUseCase::search_persons(&db, "  ", me.person_id, 10).await.is_empty());
    assert_eq!(ids(&PersonUseCase::search_persons(&db, "fIRST2", me.person_id, 10).await), vec![near.person_id]);
    assert_eq!(ids(&PersonUseCase::search_persons(&db, "USER3@", me.person_id, 10).await), vec![far.person_id]);
    assert!(!ids(&PersonUseCase::search_persons(&db, "First", me.person_id, 10).await).contains(&me.person_id));
    assert_eq!(PersonUseCase::search_persons(&db, "First", me.person_id, 2).await.len(), 2);
    assert!(PersonUseCase::search_persons_by_uuid(&db, "", me.person_uuid.clone(), 10).await.is_empty());
    assert_eq!(ids(&PersonUseCase::search_persons_by_uuid(&db, "First2", me.person_uuid.clone(), 10).await), vec![near.person_id]);
    assert_eq!(ids(&PersonUseCase::search_persons_by_uuid(&db, "user3@", me.person_uuid.clone(), 10).await), vec![far.person_id]);
    assert_eq!(PersonUseCase::search_persons_by_uuid(&db, "First", me.person_uuid.clone(), 1).await.len(), 1);

    // find friends: needs a filter; excludes anyone already related; text AND location intersect
    assert!(PersonUseCase::find_friends(&db, me.person_id, None, None, None, None, 10).await.is_empty());
    assert!(PersonUseCase::find_friends(&db, me.person_id, Some("  ".into()), Some(999.0), Some(0.0), None, 10).await.is_empty(), "invalid point and blank text");
    let by_text = PersonUseCase::find_friends(&db, me.person_id, Some("First".into()), None, None, None, 10).await;
    assert_eq!(ids(&by_text), vec![near.person_id, far.person_id], "related people (pending and accepted) are excluded");
    let by_point = PersonUseCase::find_friends(&db, me.person_id, None, Some(-23.5505), Some(-46.6333), Some(5.0), 10).await;
    assert_eq!(ids(&by_point), vec![near.person_id], "only the unrelated person within 5 km");
    let both = PersonUseCase::find_friends(&db, me.person_id, Some("First3".into()), Some(-23.5505), Some(-46.6333), Some(5.0), 10).await;
    assert!(both.is_empty(), "text matches a far person, so the intersection is empty");
    let wide = PersonUseCase::find_friends(&db, me.person_id, None, Some(-23.5505), Some(-46.6333), Some(-1.0), 1).await;
    assert_eq!(wide.len(), 1, "an invalid radius falls back to the default and the limit applies");

    // suggestions: bad radius or no saved address -> nothing; home address vs explicit point
    assert!(PersonUseCase::get_suggestions(&db, me.person_id, -1.0, None, None).await.is_empty());
    assert!(PersonUseCase::get_suggestions(&db, me.person_id, f64::NAN, None, None).await.is_empty());
    assert_eq!(ids(&PersonUseCase::get_suggestions(&db, me.person_id, 5.0, None, None).await), vec![near.person_id]);
    assert_eq!(ids(&PersonUseCase::get_suggestions(&db, me.person_id, 200.0, None, None).await), vec![near.person_id, far.person_id]);
    assert_eq!(ids(&PersonUseCase::get_suggestions(&db, me.person_id, 1.0, Some(-22.5505), Some(-46.6333)).await), vec![far.person_id], "explicit point wins over the home address");
    let nobody = register(&db, 6).await;
    assert!(PersonUseCase::get_suggestions(&db, nobody.person_id, 50.0, None, None).await.is_empty(), "no saved address and no point");

    // mentionable friends: only accepted friends, capped, never the caller
    assert!(PersonUseCase::search_mentionable_friends(&db, 0, "First", 10).await.is_empty());
    assert!(PersonUseCase::search_mentionable_friends(&db, me.person_id, " ", 10).await.is_empty());
    assert!(PersonUseCase::search_mentionable_friends(&db, nobody.person_id, "First", 10).await.is_empty(), "no friends");
    assert_eq!(ids(&PersonUseCase::search_mentionable_friends(&db, me.person_id, "First", 10).await), vec![friend.person_id], "pending and unrelated people are not mentionable");
    assert_eq!(ids(&PersonUseCase::search_mentionable_friends(&db, me.person_id, "user5@", 0).await), vec![friend.person_id], "limit 0 uses the default");
}
