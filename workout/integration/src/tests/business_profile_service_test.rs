//! C-010 task 7: the remaining business-profile operations over gRPC (TC-009): the Active Business Profile,
//! discovery, delete with its cascade, the create transaction (W8), and what an outsider sees. Handler level,
//! against a disposable PostGIS database (`TEST_DATABASE_URL`).
use super::server_support::error_key;
use crate::proto::business_profile::business_profile_service_server::BusinessProfileService;
use crate::proto::business_profile::{
    BusinessProfile, DeleteBusinessProfileRequest, DiscoverBusinessProfilesRequest, GetActiveBusinessProfileRequest,
};
use crate::service::business_profile_service::GrpcBusinessProfileService;
use business::commons::i18n::ErrorKey;
use business::domain::user::User;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use std::sync::Arc;
use tonic::{Code, Request};

const UUIDS: [&str; 4] = [
    "00000000-0000-0000-0000-0000000000f1",
    "00000000-0000-0000-0000-0000000000f2",
    "00000000-0000-0000-0000-0000000000f3",
    "00000000-0000-0000-0000-0000000000f4",
];
const PROFILE: &str = "30000000-0000-0000-0000-0000000000f1";

fn as_person<T>(message: T, id: i32) -> Request<T> {
    let mut request = Request::new(message);
    request.extensions_mut().insert(User::new(Some(format!("Person {id}")), format!("p{id}@example.test"), "hashed".into(), id, UUIDS[(id - 1) as usize].into()));
    request
}

/// Alice (1) owns "Alice Gym" with two addresses (one with coordinates in Sao Paulo) and Dave (4) as an
/// accepted team member; Bob (2) and Carol (3) are outsiders.
async fn world() -> Arc<DatabaseConnection> {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to a disposable PostGIS database");
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    db.execute_unprepared(&format!(
        r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
             (1, '{a}', 'Alice', 'Owner', '1990-01-01', 'X', now(), now()),
             (2, '{b}', 'Bob', 'Outsider', '1990-01-01', 'X', now(), now()),
             (3, '{c}', 'Carol', 'Outsider', '1990-01-01', 'X', now(), now()),
             (4, '{d}', 'Dave', 'Member', '1990-01-01', 'X', now(), now());
           INSERT INTO business_profile (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at) VALUES
             (1, '{PROFILE}', 1, '{a}', '999', 'Alice Gym', 'Professional', now(), now());
           INSERT INTO business_profile_address (id, uuid, business_profile_id, address_line1, postal_code, locality, administrative_area, country_code, created_at, updated_at, location) VALUES
             (1, '20000000-0000-0000-0000-0000000000f1', 1, 'Main St 10', '01000-000', 'Sao Paulo', 'SP', 'BR', now(), now(), ST_SetSRID(ST_MakePoint(-46.6333, -23.5505), 4326)::geography),
             (2, '20000000-0000-0000-0000-0000000000f2', 1, 'Side St 20', '02000-000', 'Sao Paulo', 'SP', 'BR', now(), now(), NULL);
           INSERT INTO team_members (id, uuid, business_profile_id, business_profile_uuid, person_id, person_uuid, status, created_at, updated_at) VALUES
             (1, '50000000-0000-0000-0000-0000000000f1', 1, '{PROFILE}', 4, '{d}', 'Accepted', now(), now());
           INSERT INTO profile (id, uuid, person_id, person_uuid, business_profile_id, business_profile_uuid, created_at, updated_at) VALUES
             (1, '40000000-0000-0000-0000-0000000000f1', 1, '{a}', 1, '{PROFILE}', now(), now())"#,
        a = UUIDS[0], b = UUIDS[1], c = UUIDS[2], d = UUIDS[3]
    ))
    .await
    .unwrap();
    // The fixtures use fixed ids, which leave the id sequences behind.
    db.execute_unprepared(
        "SELECT setval(pg_get_serial_sequence('business_profile', 'id'), 1); SELECT setval(pg_get_serial_sequence('profile', 'id'), 1)",
    )
    .await
    .unwrap();
    Arc::new(db)
}

async fn count(db: &DatabaseConnection, table: &str) -> i64 {
    let row = db.query_one_raw(Statement::from_string(DbBackend::Postgres, format!("SELECT count(*) AS n FROM {table}"))).await.unwrap().unwrap();
    row.try_get("", "n").unwrap()
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn the_active_business_profile_is_the_one_in_the_token() {
    let db = world().await;
    let service = GrpcBusinessProfileService::new(db.clone());

    let mut request = as_person(GetActiveBusinessProfileRequest {}, 1);
    request.extensions_mut().insert(BusinessProfileUseCase::get_by_id(&db, 1).await.expect("profile"));
    let active = service.get_active_business_profile(request).await.expect("active profile").into_inner();
    assert_eq!((active.uuid.as_str(), active.owner_uuid.as_str()), (PROFILE, UUIDS[0]), "the caller's own profile, in full");

    let none = service.get_active_business_profile(as_person(GetActiveBusinessProfileRequest {}, 1)).await.unwrap_err();
    assert_eq!((none.code(), error_key(&none)), (Code::FailedPrecondition, ErrorKey::BusinessProfileNotFound.as_str()), "REST answers 400");
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn discovery_finds_by_text_and_location_and_shows_outsiders_only_the_coarse_address() {
    let service = GrpcBusinessProfileService::new(world().await);
    let discover = |caller: i32, query: Option<&str>, location: bool| {
        let request = DiscoverBusinessProfilesRequest {
            query: query.map(str::to_string),
            latitude: location.then_some(-23.5505),
            longitude: location.then_some(-46.6333),
            radius_km: location.then_some(5.0),
            ..Default::default()
        };
        service.discover_business_profiles(as_person(request, caller))
    };

    let by_text = discover(3, Some("alice"), false).await.unwrap().into_inner().business_profiles;
    assert_eq!(by_text.len(), 1, "found by text");
    let by_location = discover(3, None, true).await.unwrap().into_inner().business_profiles;
    assert_eq!(by_location.len(), 1, "found by location");
    for profile in by_text.iter().chain(&by_location) {
        assert!(profile.tax_id.is_empty() && profile.owner_uuid.is_empty(), "no tax id or owner for an outsider");
        assert!(profile.addresses.iter().all(|a| a.address_line_1.is_empty() && a.postal_code.is_empty() && a.latitude.is_none()), "no street, postal code or coordinates");
        assert!(profile.addresses.iter().all(|a| a.locality == "Sao Paulo"), "the city is still shown");
    }
    let owner = discover(1, Some("alice"), false).await.unwrap().into_inner().business_profiles;
    assert!(owner[0].addresses.iter().all(|a| !a.address_line_1.is_empty()) && owner[0].tax_id == "999", "the owner sees everything");
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn only_the_owner_deletes_a_profile_and_the_delete_removes_everything_that_hangs_from_it() {
    let db = world().await;
    let service = GrpcBusinessProfileService::new(db.clone());
    let delete = |caller: i32, id: i32| service.delete_business_profile(as_person(DeleteBusinessProfileRequest { id }, caller));

    assert_eq!(delete(3, 1).await.unwrap_err().code(), Code::PermissionDenied, "an outsider cannot delete it");
    assert_eq!(delete(1, 99).await.unwrap_err().code(), Code::NotFound);
    assert_eq!(count(&db, "business_profile").await, 1, "nothing changed yet");

    delete(1, 1).await.expect("the owner deletes it");
    for table in ["business_profile", "business_profile_address", "team_members", "profile"] {
        assert_eq!(count(&db, table).await, 0, "{table} was cascaded");
    }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn creating_a_profile_is_all_or_nothing() {
    let db = world().await;
    let service = GrpcBusinessProfileService::new(db.clone());
    let new_profile = || BusinessProfile { business_name: "Second Gym".into(), business_type: "Company".into(), tax_id: "123".into(), ..Default::default() };

    let created = service.add_business_profile(as_person(new_profile(), 2)).await.expect("create").into_inner();
    assert_eq!(created.owner_uuid, UUIDS[1], "the owner is the caller");
    assert_eq!((count(&db, "business_profile").await, count(&db, "profile").await), (2, 2));

    // The second write cannot happen: the profile row must not stay behind.
    db.execute_unprepared("ALTER TABLE profile RENAME TO profile_away").await.unwrap();
    let failed = service.add_business_profile(as_person(new_profile(), 3)).await;
    db.execute_unprepared("ALTER TABLE profile_away RENAME TO profile").await.unwrap();
    assert!(failed.is_err(), "the create fails when the profile list cannot be written");
    assert_eq!(count(&db, "business_profile").await, 2, "the half-created profile was rolled back");
}
