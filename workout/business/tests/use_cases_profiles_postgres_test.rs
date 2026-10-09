mod support;

use business::domain::business_error::BusinessErrorKind as K;
use business::domain::business_profile::BusinessProfile;
use business::domain::business_profile_address::BusinessProfileAddress;
use business::domain::enums::ProfileType;
use business::domain::person_address::PersonAddress;
use business::use_cases::business_profile_address_use_case::BusinessProfileAddressUseCase;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use business::use_cases::person_address_use_case::PersonAddressUseCase;
use business::use_cases::team_member_use_case::TeamMemberUseCase;
use entity::{business_profile_address_entity, profile_entity, team_member_entity};
use sea_orm::{EntityTrait, PaginatorTrait};
use support::{business_profile, fresh_db, kind, register};

fn bp_address(profile_id: i32, line: &str) -> BusinessProfileAddress {
    BusinessProfileAddress::new(
        profile_id,
        line.into(),
        None,
        "City".into(),
        "SP".into(),
        None,
        "BR".into(),
    )
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn business_profile_ownership_update_rules_and_cascading_delete() {
    let db = fresh_db().await;
    let (owner, other, member) = (
        register(&db, 1).await,
        register(&db, 2).await,
        register(&db, 3).await,
    );

    // create: ownership always comes from the actor, and the profile mapping is created
    let spoofed = BusinessProfile::new(
        other.person_id,
        other.person_uuid.clone(),
        "111".into(),
        "Alpha Gym".into(),
        ProfileType::Company,
        Some("Alpha".into()),
    );
    let created = BusinessProfileUseCase::add(&db, spoofed, &owner)
        .await
        .unwrap();
    assert_eq!(
        (created.owner_id, created.owner_uuid.as_str()),
        (owner.person_id, owner.person_uuid.as_str())
    );
    let id = created.id.unwrap();
    assert_eq!(profile_entity::Entity::find().count(&db).await.unwrap(), 1);

    // reads
    assert_eq!(
        BusinessProfileUseCase::get_by_id(&db, id)
            .await
            .unwrap()
            .business_name,
        "Alpha Gym"
    );
    assert_eq!(
        BusinessProfileUseCase::get_by_uuid(&db, created.uuid.clone().unwrap())
            .await
            .unwrap()
            .id,
        Some(id)
    );
    assert!(BusinessProfileUseCase::get_by_id(&db, 9999).await.is_none());
    assert!(BusinessProfileUseCase::get_by_uuid(
        &db,
        "00000000-0000-0000-0000-00000000dead".into()
    )
    .await
    .is_none());
    assert!(
        BusinessProfileUseCase::get_by_uuid(&db, "not-a-uuid".into())
            .await
            .is_none()
    );
    assert_eq!(
        BusinessProfileUseCase::get_by_owner_id(&db, owner.person_id)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        BusinessProfileUseCase::get_by_owner_id(&db, other.person_id)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        BusinessProfileUseCase::get_by_owner_uuid(&db, owner.person_uuid.clone())
            .await
            .unwrap()
            .len(),
        1
    );

    // update: needs an id, an existing profile and the owner; the uuid cannot be rewritten
    let mut edit = BusinessProfileUseCase::get_by_id(&db, id).await.unwrap();
    edit.business_name = "Alpha Fitness".into();
    let mut no_id = edit.clone();
    no_id.id = None;
    assert!(matches!(
        kind(
            &BusinessProfileUseCase::update(&db, no_id, &owner)
                .await
                .unwrap_err()
        ),
        K::Validation
    ));
    let mut missing = edit.clone();
    missing.id = Some(9999);
    assert!(matches!(
        kind(
            &BusinessProfileUseCase::update(&db, missing, &owner)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    assert!(matches!(
        kind(
            &BusinessProfileUseCase::update(&db, edit.clone(), &other)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    let mut tampered = edit.clone();
    tampered.uuid = Some("00000000-0000-0000-0000-000000000bad".into());
    tampered.owner_id = other.person_id;
    let updated = BusinessProfileUseCase::update(&db, tampered, &owner)
        .await
        .unwrap();
    assert_eq!(updated.business_name, "Alpha Fitness");
    assert_eq!(updated.uuid, created.uuid, "uuid is not editable");
    assert_eq!(updated.owner_id, owner.person_id, "owner is not editable");

    // addresses hang off the profile: only its owner may add, change or remove them
    let addr = BusinessProfileAddressUseCase::save(
        &db,
        bp_address(id, "Main St"),
        owner.person_id,
        Some(-23.5505),
        Some(-46.6333),
    )
    .await
    .unwrap();
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::save(
                &db,
                bp_address(id, "Evil"),
                other.person_id,
                None,
                None
            )
            .await
            .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::save(
                &db,
                bp_address(9999, "X"),
                owner.person_id,
                None,
                None
            )
            .await
            .unwrap_err()
        ),
        K::NotFound
    ));
    let mut change = addr.clone();
    change.address_line1 = "Other St".into();
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::save(&db, change.clone(), other.person_id, None, None)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert_eq!(
        BusinessProfileAddressUseCase::save(
            &db,
            change,
            owner.person_id,
            Some(-23.55),
            Some(-46.63)
        )
        .await
        .unwrap()
        .address_line1,
        "Other St"
    );
    let mut ghost = addr.clone();
    ghost.id = Some(9999);
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::save(&db, ghost, owner.person_id, None, None)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    assert_eq!(
        BusinessProfileUseCase::get_by_id(&db, id)
            .await
            .unwrap()
            .addresses
            .len(),
        1
    );

    let second = BusinessProfileAddressUseCase::save(
        &db,
        bp_address(id, "Second St"),
        owner.person_id,
        None,
        None,
    )
    .await
    .unwrap();
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::delete_by_id(&db, addr.id.unwrap(), other.person_id)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::delete_by_id(&db, 9999, owner.person_id)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    BusinessProfileAddressUseCase::delete_by_id(&db, addr.id.unwrap(), owner.person_id)
        .await
        .unwrap();
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::delete_by_uuid(
                &db,
                second.uuid.clone().unwrap(),
                other.person_id
            )
            .await
            .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &BusinessProfileAddressUseCase::delete_by_uuid(
                &db,
                "00000000-0000-0000-0000-00000000dead".into(),
                owner.person_id
            )
            .await
            .unwrap_err()
        ),
        K::NotFound
    ));
    BusinessProfileAddressUseCase::delete_by_uuid(
        &db,
        second.uuid.clone().unwrap(),
        owner.person_id,
    )
    .await
    .unwrap();
    assert!(BusinessProfileUseCase::get_by_id(&db, id)
        .await
        .unwrap()
        .addresses
        .is_empty());

    // delete: owner only; cascades addresses, memberships and the profile mapping
    BusinessProfileAddressUseCase::save(
        &db,
        bp_address(id, "Last St"),
        owner.person_id,
        None,
        None,
    )
    .await
    .unwrap();
    TeamMemberUseCase::send_team_member_request(&db, id, member.person_id)
        .await
        .unwrap();
    assert!(matches!(
        kind(
            &BusinessProfileUseCase::delete(&db, id, &other)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &BusinessProfileUseCase::delete(&db, 9999, &owner)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    BusinessProfileUseCase::delete(&db, id, &owner)
        .await
        .unwrap();
    assert!(BusinessProfileUseCase::get_by_id(&db, id).await.is_none());
    assert_eq!(
        business_profile_address_entity::Entity::find()
            .count(&db)
            .await
            .unwrap(),
        0
    );
    assert_eq!(
        team_member_entity::Entity::find().count(&db).await.unwrap(),
        0
    );
    assert_eq!(profile_entity::Entity::find().count(&db).await.unwrap(), 0);
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn professional_discovery_combines_text_type_and_location_filters() {
    let db = fresh_db().await;
    let owner = register(&db, 1).await;
    let gym = BusinessProfileUseCase::add(
        &db,
        BusinessProfile::new(
            0,
            String::new(),
            "1".into(),
            "Iron Gym".into(),
            ProfileType::Company,
            None,
        ),
        &owner,
    )
    .await
    .unwrap();
    let coach = BusinessProfileUseCase::add(
        &db,
        BusinessProfile::new(
            0,
            String::new(),
            "2".into(),
            "Iron Coach".into(),
            ProfileType::Professional,
            Some("Ironclad".into()),
        ),
        &owner,
    )
    .await
    .unwrap();
    let far = BusinessProfileUseCase::add(
        &db,
        BusinessProfile::new(
            0,
            String::new(),
            "3".into(),
            "Zen Studio".into(),
            ProfileType::Company,
            None,
        ),
        &owner,
    )
    .await
    .unwrap();
    for (p, lat) in [(&gym, -23.5505), (&coach, -23.5455), (&far, -20.0)] {
        BusinessProfileAddressUseCase::save(
            &db,
            bp_address(p.id.unwrap(), "Addr"),
            owner.person_id,
            Some(lat),
            Some(-46.6333),
        )
        .await
        .unwrap();
    }
    let ids = |v: Vec<BusinessProfile>| {
        let mut i: Vec<i32> = v.iter().filter_map(|p| p.id).collect();
        i.sort();
        i
    };
    let (g, c, f) = (gym.id.unwrap(), coach.id.unwrap(), far.id.unwrap());

    // a marketplace search never lists everything
    assert!(
        BusinessProfileUseCase::discover(&db, None, None, None, None, None, 10)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        BusinessProfileUseCase::discover(
            &db,
            Some("   ".into()),
            Some(ProfileType::Company),
            Some(200.0),
            None,
            None,
            10
        )
        .await
        .unwrap()
        .is_empty(),
        "blank text and an invalid point"
    );

    // text, text + type, location, location + type, text AND location
    assert_eq!(
        ids(
            BusinessProfileUseCase::discover(&db, Some("iron".into()), None, None, None, None, 10)
                .await
                .unwrap()
        ),
        vec![g, c]
    );
    assert_eq!(
        ids(BusinessProfileUseCase::discover(
            &db,
            Some("iron".into()),
            Some(ProfileType::Professional),
            None,
            None,
            None,
            10
        )
        .await
        .unwrap()),
        vec![c]
    );
    assert_eq!(
        ids(BusinessProfileUseCase::discover(
            &db,
            None,
            None,
            Some(-23.55),
            Some(-46.6333),
            Some(5.0),
            10
        )
        .await
        .unwrap()),
        vec![g, c]
    );
    assert_eq!(
        ids(BusinessProfileUseCase::discover(
            &db,
            None,
            Some(ProfileType::Company),
            Some(-23.55),
            Some(-46.6333),
            Some(5.0),
            10
        )
        .await
        .unwrap()),
        vec![g]
    );
    assert_eq!(
        ids(BusinessProfileUseCase::discover(
            &db,
            Some("zEN".into()),
            None,
            Some(-23.55),
            Some(-46.6333),
            Some(5.0),
            10
        )
        .await
        .unwrap()),
        Vec::<i32>::new(),
        "text hits outside the radius are dropped"
    );
    assert_eq!(
        ids(BusinessProfileUseCase::discover(
            &db,
            Some("zEN".into()),
            None,
            Some(-23.55),
            Some(-46.6333),
            Some(1000.0),
            10
        )
        .await
        .unwrap()),
        vec![f],
        "the radius is capped at 500 km, which still reaches 370 km"
    );

    // an invalid radius falls back to the default (200 km), and the limit is applied after the intersection
    assert_eq!(
        BusinessProfileUseCase::discover(
            &db,
            None,
            None,
            Some(-23.55),
            Some(-46.6333),
            Some(f64::NAN),
            1
        )
        .await
        .unwrap()
        .len(),
        1
    );
    assert_eq!(
        BusinessProfileUseCase::discover(
            &db,
            Some("iron".into()),
            None,
            Some(-23.55),
            Some(-46.6333),
            None,
            0
        )
        .await
        .unwrap()
        .len(),
        2,
        "limit 0 uses the default"
    );

    // wildcards are matched literally
    assert!(
        BusinessProfileUseCase::discover(&db, Some("%".into()), None, None, None, None, 10)
            .await
            .unwrap()
            .is_empty()
    );
    let _ = business_profile;
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn person_addresses_keep_one_current_and_belong_to_their_person() {
    let db = fresh_db().await;
    let (a, b) = (register(&db, 1).await, register(&db, 2).await);
    let new_addr = |person: i32, line: &str| {
        PersonAddress::new(
            person,
            line.into(),
            None,
            "City".into(),
            "SP".into(),
            None,
            "BR".into(),
            false,
        )
    };

    let first = PersonAddressUseCase::add_person_address(
        &db,
        new_addr(a.person_id, "One"),
        Some(-23.55),
        Some(-46.63),
    )
    .await
    .unwrap();
    let second =
        PersonAddressUseCase::add_person_address(&db, new_addr(a.person_id, "Two"), None, None)
            .await
            .unwrap();
    assert!(
        first.current && second.current,
        "a new address is saved as current"
    );
    let current = |rows: Vec<entity::person_address_entity::PersonAddressEntity>| {
        rows.into_iter().filter(|r| r.current).count()
    };
    let all =
        business::gateway::person_address_gateway::PersonAddressGateway::find_all_by_person_id(
            &db,
            a.person_id,
        )
        .await;
    assert_eq!(
        (all.len(), current(all)),
        (2, 1),
        "only the newest address stays current"
    );

    // update: only the owner; the owner is always the actor; unknown ids are not found
    let mut edit = second.clone();
    edit.address_line1 = "Two B".into();
    assert!(matches!(
        kind(
            &PersonAddressUseCase::update_person_address(
                &db,
                edit.clone(),
                b.person_id,
                None,
                None
            )
            .await
            .unwrap_err()
        ),
        K::Forbidden
    ));
    assert_eq!(
        PersonAddressUseCase::update_person_address(
            &db,
            edit,
            a.person_id,
            Some(-23.5),
            Some(-46.6)
        )
        .await
        .unwrap()
        .address_line1,
        "Two B"
    );
    let mut ghost = second.clone();
    ghost.id = Some(9999);
    assert!(matches!(
        kind(
            &PersonAddressUseCase::update_person_address(&db, ghost, a.person_id, None, None)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    let created_by_update = PersonAddressUseCase::update_person_address(
        &db,
        new_addr(b.person_id, "Spoof"),
        a.person_id,
        None,
        None,
    )
    .await
    .unwrap();
    assert_eq!(
        created_by_update.person_id, a.person_id,
        "an address created through update belongs to the actor"
    );

    // delete by id and uuid: owner only
    assert!(matches!(
        kind(
            &PersonAddressUseCase::delete_person_address(&db, first.id.unwrap(), b.person_id)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &PersonAddressUseCase::delete_person_address(&db, 9999, a.person_id)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    PersonAddressUseCase::delete_person_address(&db, first.id.unwrap(), a.person_id)
        .await
        .unwrap();
    assert!(matches!(
        kind(
            &PersonAddressUseCase::delete_person_address_by_uuid(
                &db,
                second.uuid.clone().unwrap(),
                b.person_id
            )
            .await
            .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &PersonAddressUseCase::delete_person_address_by_uuid(
                &db,
                "00000000-0000-0000-0000-00000000dead".into(),
                a.person_id
            )
            .await
            .unwrap_err()
        ),
        K::NotFound
    ));
    PersonAddressUseCase::delete_person_address_by_uuid(
        &db,
        second.uuid.clone().unwrap(),
        a.person_id,
    )
    .await
    .unwrap();
}
