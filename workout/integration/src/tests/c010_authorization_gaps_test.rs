//! C-010 task 1: one failing test for every authorization or integrity gap confirmed in the `workout`
//! gRPC surface. Each test states the behavior AFTER the fix, so all of them fail today and must pass
//! when the task named in the comment lands (design.md, "Known gaps"). They run against a disposable
//! PostGIS database (`TEST_DATABASE_URL`) and call the handlers or `GrpcAuthLayer` directly.
use crate::auth::grpc_auth_layer::GrpcAuthLayer;
use crate::proto::business_profile::business_profile_service_server::BusinessProfileService;
use crate::proto::business_profile::{BusinessProfileRequestId, BusinessProfileRequestOwnerId, RemoveBusinessProfileAddressRequest};
use crate::proto::exercise::exercise_service_server::ExerciseService;
use crate::proto::exercise::{Exercise, ExerciseRequest, exercise_request};
use crate::proto::person::person_service_server::PersonService;
use crate::proto::person::{RemovePersonAddressRequest, SearchMentionableFriendsRequest};
use crate::proto::settings::settings_service_server::SettingsService;
use crate::proto::settings::SettingOwnerIdRequest;
use crate::proto::workout::workout_service_server::WorkoutService;
use crate::proto::workout::{Workout, WorkoutExercisesRequest, WorkoutRequest, workout_request};
use crate::service::business_profile_service::GrpcBusinessProfileService;
use crate::service::exercise_service::GrpcExerciseService;
use crate::service::person_service::GrpcPersonService;
use crate::service::settings_service::GrpcSettingService;
use crate::service::workout_service::GrpcWorkoutService;
use business::domain::access_token::Claims;
use business::domain::user::User;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use std::sync::Arc;
use tonic::body::Body as TonicBody;
use tonic::{Code, Request};
use tower::{Layer, ServiceExt};

const ALICE: &str = "00000000-0000-0000-0000-0000000000a1";
const BOB: &str = "00000000-0000-0000-0000-0000000000a2";
const CAROL: &str = "00000000-0000-0000-0000-0000000000a3";
const DAVE: &str = "00000000-0000-0000-0000-0000000000a4";
const PROFILE: &str = "30000000-0000-0000-0000-0000000000a1";
const SECRET: &str = "c010-test-secret";

fn person_uuid(id: i32) -> &'static str {
    [ALICE, BOB, CAROL, DAVE][(id - 1) as usize]
}

/// A request authenticated as person `id`, as `GrpcAuthLayer` would deliver it.
fn as_person<T>(message: T, id: i32) -> Request<T> {
    let mut request = Request::new(message);
    request.extensions_mut().insert(User::new(
        Some(format!("Person {id}")),
        format!("person{id}@example.test"),
        "hashed".to_string(),
        id,
        person_uuid(id).to_string(),
    ));
    request
}

/// Alice (1) and Bob (2) are friends; Carol (3) is a stranger; Dave (4) is an accepted team member of
/// Alice's business profile, which has two addresses; Alice has two person addresses and settings.
async fn world() -> Arc<DatabaseConnection> {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to a disposable PostGIS database");
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    db.execute_unprepared(&format!(
        r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
             (1, '{ALICE}', 'Alice', 'Owner', '1990-01-01', 'X', now(), now()),
             (2, '{BOB}', 'Bob', 'Friend', '1990-01-01', 'X', now(), now()),
             (3, '{CAROL}', 'Carol', 'Stranger', '1990-01-01', 'X', now(), now()),
             (4, '{DAVE}', 'Dave', 'Member', '1990-01-01', 'X', now(), now());
           INSERT INTO "user" (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at) VALUES
             (1, '10000000-0000-0000-0000-0000000000a1', 'Person 1', 'person1@example.test', 'unused', false, true, 1, '{ALICE}', now(), now()),
             (3, '10000000-0000-0000-0000-0000000000a3', 'Person 3', 'person3@example.test', 'unused', false, true, 3, '{CAROL}', now(), now());
           INSERT INTO friends (uuid, person_id, person_uuid, friend_id, friend_uuid, status, created_at, updated_at) VALUES
             ('60000000-0000-0000-0000-0000000000a1', 1, '{ALICE}', 2, '{BOB}', 'Accepted', now(), now()),
             ('60000000-0000-0000-0000-0000000000a2', 2, '{BOB}', 1, '{ALICE}', 'Accepted', now(), now());
           INSERT INTO settings (uuid, person_id, person_uuid, language, theme, notifications_enabled, context_menu_position, home_page, created_at, updated_at) VALUES
             ('40000000-0000-0000-0000-0000000000a1', 1, '{ALICE}', 'en', 'light', true, 'Left', 'feed', now(), now());
           INSERT INTO person_address (id, uuid, person_id, address_line1, locality, administrative_area, country_code, current, location, created_at, updated_at) VALUES
             (1, '70000000-0000-0000-0000-0000000000a1', 1, 'Home', 'Sao Paulo', 'SP', 'BR', true, ST_SetSRID(ST_MakePoint(-46.6333, -23.5505), 4326)::geography, now(), now()),
             (2, '70000000-0000-0000-0000-0000000000a2', 1, 'Work', 'Sao Paulo', 'SP', 'BR', false, ST_SetSRID(ST_MakePoint(-46.6333, -23.5405), 4326)::geography, now(), now());
           INSERT INTO business_profile (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at) VALUES
             (1, '{PROFILE}', 1, '{ALICE}', '999', 'Alice Gym', 'Professional', now(), now());
           INSERT INTO business_profile_address (id, uuid, business_profile_id, address_line1, locality, administrative_area, country_code, created_at, updated_at) VALUES
             (1, '20000000-0000-0000-0000-0000000000a1', 1, 'Main St', 'Sao Paulo', 'SP', 'BR', now(), now()),
             (2, '20000000-0000-0000-0000-0000000000a2', 1, 'Side St', 'Sao Paulo', 'SP', 'BR', now(), now());
           INSERT INTO team_members (id, uuid, business_profile_id, business_profile_uuid, person_id, person_uuid, status, created_at, updated_at) VALUES
             (1, '50000000-0000-0000-0000-0000000000a1', 1, '{PROFILE}', 4, '{DAVE}', 'Accepted', now(), now())"#
    ))
    .await
    .unwrap();
    Arc::new(db)
}

async fn count(db: &DatabaseConnection, table: &str, id: i32) -> i64 {
    let row = db
        .query_one_raw(Statement::from_string(DbBackend::Postgres, format!("SELECT count(*) AS n FROM {table} WHERE id = {id}")))
        .await
        .unwrap()
        .unwrap();
    row.try_get("", "n").unwrap()
}

// ---------------------------------------------------------------- handler gaps

/// W1 (task 9): the mentionable-friends search must act only for the caller.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL; expected to FAIL until C-010 task 9"]
async fn w1_mentionable_friends_are_searchable_only_by_their_owner() {
    let service = GrpcPersonService::new(world().await);
    let search = |caller: Option<i32>| {
        let message = SearchMentionableFriendsRequest { person_id: 1, query: "Bob".into(), limit: 10 };
        let request = match caller {
            Some(id) => as_person(message, id),
            None => Request::new(message),
        };
        service.search_mentionable_friends(request)
    };
    assert_eq!(search(Some(1)).await.unwrap().into_inner().people.len(), 1, "Alice finds her own friend");
    assert_eq!(search(Some(3)).await.unwrap_err().code(), Code::PermissionDenied, "a stranger must not list Alice's friends");
    assert_eq!(search(None).await.unwrap_err().code(), Code::Unauthenticated, "no actor, no answer");
}

/// W3 (task 7): removing a business-profile address by `uuid` removes that address.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL; expected to FAIL until C-010 task 7"]
async fn w3_a_business_profile_address_is_removed_by_uuid() {
    let db = world().await;
    let service = GrpcBusinessProfileService::new(db.clone());
    let request = RemoveBusinessProfileAddressRequest { id: 0, uuid: "20000000-0000-0000-0000-0000000000a2".into() };
    service.remove_business_profile_address(as_person(request, 1)).await.expect("remove by uuid");
    assert_eq!(count(&db, "business_profile_address", 2).await, 0, "the named address is gone");
    assert_eq!(count(&db, "business_profile_address", 1).await, 1, "the other address stays");
}

/// W13 (task 6): removing a person address by `uuid` removes that address (the REST route does).
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL; expected to FAIL until C-010 task 6"]
async fn w13_a_person_address_is_removed_by_uuid() {
    let db = world().await;
    let service = GrpcPersonService::new(db.clone());
    let request = RemovePersonAddressRequest { id: 0, uuid: "70000000-0000-0000-0000-0000000000a2".into() };
    service.remove_person_address(as_person(request, 1)).await.expect("remove by uuid");
    assert_eq!(count(&db, "person_address", 2).await, 0, "the named address is gone");
    assert_eq!(count(&db, "person_address", 1).await, 1, "the other address stays");
}

/// W14 (task 6): the settings read by owner uuid that REST serves (`/settings/owner/uuid/{uuid}`).
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL; expected to FAIL until C-010 task 6"]
async fn w14_settings_are_readable_by_the_owner_uuid() {
    let service = GrpcSettingService::new(world().await);
    let own = SettingOwnerIdRequest { owner_id: 0, owner_uuid: ALICE.into() };
    let found = service.get_by_owner_ids(as_person(own, 1)).await.expect("own settings by uuid").into_inner();
    assert_eq!(found.owner_uuid, ALICE);
    let foreign = SettingOwnerIdRequest { owner_id: 0, owner_uuid: ALICE.into() };
    assert_eq!(service.get_by_owner_ids(as_person(foreign, 3)).await.unwrap_err().code(), Code::PermissionDenied);
}

/// W5 (task 7): addresses and owner ids reach only the owner and accepted team members.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL; expected to FAIL until C-010 task 7"]
async fn w5_business_profile_addresses_and_owner_ids_are_restricted() {
    let service = GrpcBusinessProfileService::new(world().await);
    let by_uuid = |caller: i32| {
        let request = BusinessProfileRequestId { id: 0, uuid: PROFILE.into() };
        service.get_business_profile_by_id(as_person(request, caller))
    };
    for member in [1, 4] {
        let profile = by_uuid(member).await.unwrap().into_inner();
        assert_eq!(profile.addresses.len(), 2, "person {member} sees the addresses");
        assert_eq!(profile.owner_uuid, ALICE, "person {member} sees the owner");
    }
    for outsider in [2, 3] {
        let profile = by_uuid(outsider).await.unwrap().into_inner();
        assert!(profile.addresses.is_empty(), "person {outsider} must not see the addresses");
        assert!(profile.owner_uuid.is_empty() && profile.owner_id == 0, "person {outsider} must not see the owner ids");
        assert_eq!(profile.business_name, "Alice Gym", "the public fields stay");
    }
    let by_owner = BusinessProfileRequestOwnerId { owner_id: 1, owner_uuid: String::new() };
    let listed = service.get_business_profile_by_owner_id(as_person(by_owner, 3)).await.unwrap().into_inner();
    assert!(listed.business_profiles.iter().all(|p| p.addresses.is_empty() && p.owner_uuid.is_empty()), "the owner listing hides them too");
}

fn private_exercise() -> Exercise {
    Exercise { name: "Squat".into(), category: "Strength".into(), sets: 3, reps_or_duration: 10, visibility: "Private".into(), ..Default::default() }
}

fn exercise_id(id: i32) -> ExerciseRequest {
    ExerciseRequest { identifier: Some(exercise_request::Identifier::Id(id)) }
}

/// The status of a call that must fail.
fn status<T>(result: Result<tonic::Response<T>, tonic::Status>) -> Code {
    result.err().expect("the call must be refused").code()
}

fn workout_id(id: i32) -> WorkoutRequest {
    WorkoutRequest { identifier: Some(workout_request::Identifier::Id(id)) }
}

/// W4 (task 9): a mutation on a resource the caller cannot read is `NOT_FOUND`; a reader who is not the
/// owner still gets `PERMISSION_DENIED`.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL; expected to FAIL until C-010 task 9"]
async fn w4_mutating_an_unreadable_workout_or_exercise_is_not_found() {
    let db = world().await;
    let exercises = GrpcExerciseService::new(db.clone());
    let workouts = GrpcWorkoutService::new(db);

    let private = exercises.add_exercise(as_person(private_exercise(), 1)).await.unwrap().into_inner();
    let public = exercises.add_exercise(as_person(Exercise { visibility: "Public".into(), name: "Row".into(), ..private_exercise() }, 1)).await.unwrap().into_inner();

    assert_eq!(status(exercises.delete_exercise(as_person(exercise_id(private.id), 3)).await), Code::NotFound, "private exercise, stranger deletes");
    assert_eq!(status(exercises.update_exercise(as_person(Exercise { name: "x".into(), ..private.clone() }, 3)).await), Code::NotFound, "private exercise, stranger updates");
    assert_eq!(status(exercises.delete_exercise(as_person(exercise_id(public.id), 3)).await), Code::PermissionDenied, "a reader who is not the owner keeps 403");

    let workout = Workout { name: "Push".into(), visibility: "Private".into(), difficulty: "Easy".into(), muscle_group: "Chest".into(), ..Default::default() };
    let created = workouts.add_workout(as_person(workout, 1)).await.unwrap().into_inner();
    assert_eq!(status(workouts.delete_workout(as_person(workout_id(created.id), 3)).await), Code::NotFound, "private workout, stranger deletes");
    assert_eq!(status(workouts.update_workout(as_person(Workout { name: "x".into(), ..created.clone() }, 3)).await), Code::NotFound, "private workout, stranger updates");
    let add = WorkoutExercisesRequest { workout_uuid: created.uuid.clone(), exercises: vec![private.clone()] };
    assert_eq!(status(workouts.add_exercises_to_workout(as_person(add, 3)).await), Code::NotFound, "private workout, stranger adds exercises");
}

// ---------------------------------------------------------------- authentication layer gaps

fn token_for(person_id: i32, email: &str) -> String {
    let claims = Claims::new(
        email.to_string(),
        chrono::Utc::now().timestamp() + 3600,
        format!("user-{person_id}"),
        format!("Person {person_id}"),
        person_id,
        person_uuid(person_id).to_string(),
        "default".to_string(),
        None,
        None,
    );
    encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(SECRET.as_bytes())).unwrap()
}

/// The `grpc-status` the layer answers for a call, or `None` when it let the call through to the service.
async fn through_layer(db: Arc<DatabaseConnection>, path: &str, headers: &[(&str, String)]) -> Option<String> {
    let inner = tower::service_fn(|_request: hyper::Request<TonicBody>| async {
        Ok::<_, std::convert::Infallible>(hyper::Response::new(TonicBody::empty()))
    });
    let service = GrpcAuthLayer::new(db).layer(inner);
    let mut builder = hyper::Request::builder().uri(path);
    for (name, value) in headers {
        builder = builder.header(*name, value.as_str());
    }
    let response = service.oneshot(builder.body(TonicBody::empty()).unwrap()).await.unwrap();
    response.headers().get("grpc-status").and_then(|v| v.to_str().ok()).map(str::to_string)
}

/// W11 (fixed in task 2): an ordinary RPC from a person without current Terms and Privacy consent is
/// refused, while the recovery paths stay open.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn w11_grpc_enforces_current_terms_and_privacy_consent() {
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("AUTH_RULES_ENABLED", "false");
    }
    let db = world().await;
    let bearer = vec![("authorization", format!("Bearer {}", token_for(1, "person1@example.test")))];
    // Alice has accepted nothing: no consent rows exist.
    let ordinary = through_layer(db.clone(), "/grpc.workout.WorkoutService/GetWorkoutsByOwner", &bearer).await;
    assert_eq!(ordinary.as_deref(), Some("7"), "PERMISSION_DENIED without current consent");
    let check = through_layer(db, "/grpc.person.PersonService/HasActiveConsent", &bearer).await;
    assert_eq!(check, None, "the consent check itself must stay reachable");
}

/// W2 (task 9): `GetTeamRoster` opens only to the internal secret, never to a user token.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL; expected to FAIL until C-010 task 9"]
async fn w2_the_team_roster_accepts_only_the_internal_secret() {
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("AUTH_RULES_ENABLED", "false");
        std::env::set_var("INTERNAL_SERVICE_SECRET", "c010-internal-secret");
    }
    let db = world().await;
    let path = "/grpc.team_member.TeamMemberService/GetTeamRoster";
    let user = vec![("authorization", format!("Bearer {}", token_for(1, "person1@example.test")))];
    assert_eq!(through_layer(db.clone(), path, &user).await.as_deref(), Some("16"), "a user token must not open the roster");
    let wrong = vec![("x-internal-secret", "wrong".to_string())];
    assert_eq!(through_layer(db.clone(), path, &wrong).await.as_deref(), Some("16"), "a wrong secret is refused");
    let internal = vec![("x-internal-secret", "c010-internal-secret".to_string())];
    assert_eq!(through_layer(db, path, &internal).await, None, "the secret opens it");
}
