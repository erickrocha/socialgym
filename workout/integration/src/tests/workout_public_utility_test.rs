//! C-010 task 8: the workout exercises read and the numeric-id compose (TC-010), and the legal documents and
//! address search (TC-011), over gRPC. Handler level for the workout operations and the in-process server of
//! `server_support` for the public and utility ones, against a disposable PostGIS database.
use super::server_support::{error_key, serve, with};
use crate::proto::address::address_search_service_client::AddressSearchServiceClient;
use crate::proto::address::SearchAddressRequest;
use crate::proto::auth::auth_service_client::AuthServiceClient;
use crate::proto::auth::SignupRequest;
use crate::proto::exercise::exercise_service_server::ExerciseService;
use crate::proto::exercise::Exercise;
use crate::proto::legal::legal_document_service_client::LegalDocumentServiceClient;
use crate::proto::legal::{GetLegalDocumentRequest, ListLegalDocumentsRequest};
use crate::proto::workout::workout_request::Identifier;
use crate::proto::workout::workout_service_server::WorkoutService;
use crate::proto::workout::{Workout, WorkoutExercisesRequest, WorkoutRequest};
use crate::service::exercise_service::GrpcExerciseService;
use crate::service::workout_service::GrpcWorkoutService;
use business::commons::i18n::ErrorKey;
use business::domain::user::User;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tonic::{Code, Request};

const UUIDS: [&str; 3] = [
    "00000000-0000-0000-0000-0000000000a1",
    "00000000-0000-0000-0000-0000000000a2",
    "00000000-0000-0000-0000-0000000000a3",
];

fn as_person<T>(message: T, id: i32) -> Request<T> {
    let mut request = Request::new(message);
    request.extensions_mut().insert(User::new(Some(format!("Person {id}")), format!("p{id}@example.test"), "hashed".into(), id, UUIDS[(id - 1) as usize].into()));
    request
}

/// Alice (1) and Bob (2) are friends; Carol (3) is a stranger.
async fn world() -> Arc<DatabaseConnection> {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to a disposable PostGIS database");
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    db.execute_unprepared(&format!(
        r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
             (1, '{a}', 'Alice', 'Owner', '1990-01-01', 'X', now(), now()),
             (2, '{b}', 'Bob', 'Friend', '1990-01-01', 'X', now(), now()),
             (3, '{c}', 'Carol', 'Stranger', '1990-01-01', 'X', now(), now());
           INSERT INTO friends (uuid, person_id, person_uuid, friend_id, friend_uuid, status, created_at, updated_at) VALUES
             ('60000000-0000-0000-0000-0000000000a1', 1, '{a}', 2, '{b}', 'Accepted', now(), now()),
             ('60000000-0000-0000-0000-0000000000a2', 2, '{b}', 1, '{a}', 'Accepted', now(), now())"#,
        a = UUIDS[0], b = UUIDS[1], c = UUIDS[2]
    ))
    .await
    .unwrap();
    Arc::new(db)
}

fn exercise(name: &str, visibility: &str) -> Exercise {
    Exercise { name: name.into(), category: "Strength".into(), sets: 3, reps_or_duration: 10, visibility: visibility.into(), ..Default::default() }
}

fn by_id(id: i32) -> WorkoutRequest {
    WorkoutRequest { identifier: Some(Identifier::Id(id)) }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn the_exercises_of_a_workout_are_read_and_composed_by_id_or_uuid_with_the_rest_rules() {
    let db = world().await;
    let exercises = GrpcExerciseService::new(db.clone());
    let workouts = GrpcWorkoutService::new(db);

    let public = exercises.add_exercise(as_person(exercise("Row", "Public"), 1)).await.unwrap().into_inner();
    let private = exercises.add_exercise(as_person(exercise("Squat", "Private"), 1)).await.unwrap().into_inner();
    let workout = Workout { name: "Pull".into(), visibility: "Friends".into(), difficulty: "Easy".into(), muscle_group: "Back".into(), ..Default::default() };
    let created = workouts.add_workout(as_person(workout, 1)).await.unwrap().into_inner();

    // Composed by uuid, then by the numeric id (the second REST form).
    let by_uuid = WorkoutExercisesRequest { workout_uuid: created.uuid.clone(), exercises: vec![public.clone()], workout_id: 0 };
    workouts.add_exercises_to_workout(as_person(by_uuid, 1)).await.expect("compose by uuid");
    let by_number = WorkoutExercisesRequest { workout_uuid: String::new(), exercises: vec![private.clone()], workout_id: created.id };
    let full = workouts.add_exercises_to_workout(as_person(by_number, 1)).await.expect("compose by id").into_inner();
    assert_eq!(full.exercises.len(), 2, "the answer is the full composition");
    let neither = WorkoutExercisesRequest { workout_uuid: String::new(), exercises: vec![], workout_id: 0 };
    assert_eq!(workouts.add_exercises_to_workout(as_person(neither, 1)).await.unwrap_err().code(), Code::InvalidArgument);
    let stranger = WorkoutExercisesRequest { workout_uuid: String::new(), exercises: vec![public.clone()], workout_id: created.id };
    assert!(workouts.add_exercises_to_workout(as_person(stranger, 3)).await.is_err(), "only the owner composes a workout");

    let read = |caller: i32, id: i32| workouts.get_workout_exercises(as_person(by_id(id), caller));
    assert_eq!(read(1, created.id).await.unwrap().into_inner().exercises.len(), 2, "the owner reads both");
    let friend = read(2, created.id).await.unwrap().into_inner().exercises;
    assert_eq!(friend.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["Row"], "a friend reads only the exercises they may read");
    let stranger = read(3, created.id).await.unwrap_err();
    assert_eq!(stranger.code(), Code::NotFound, "a workout the caller cannot read does not exist for them");
    let uuid_form = WorkoutRequest { identifier: Some(Identifier::Uuid(created.uuid.clone())) };
    assert_eq!(workouts.get_workout_exercises(as_person(uuid_form, 1)).await.unwrap().into_inner().exercises.len(), 2);
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn the_legal_documents_are_public_and_limited_per_address() {
    let served = serve(3).await;
    let mut legal = LegalDocumentServiceClient::new(served.channel);
    let from = |ip: &'static str| [("x-real-ip", ip)];

    let list = legal.list_legal_documents(with(ListLegalDocumentsRequest {}, None, &from("192.0.2.1"))).await.expect("no token needed").into_inner();
    let mut names: Vec<_> = list.documents.iter().map(|d| d.document.as_str()).collect();
    names.sort();
    assert_eq!(names, ["health_data", "privacy", "terms"]);
    assert!(list.documents.iter().all(|d| !d.version.is_empty() && !d.content.is_empty()));
    let terms = legal.get_legal_document(with(GetLegalDocumentRequest { document: "terms".into() }, None, &from("192.0.2.1"))).await.unwrap().into_inner();
    assert_eq!(terms.document, "terms");
    let unknown = legal.get_legal_document(with(GetLegalDocumentRequest { document: "marketing".into() }, None, &from("192.0.2.1"))).await.unwrap_err();
    assert_eq!(unknown.code(), Code::NotFound);

    // Three calls from this address used the allowance (the limit of the test server is three).
    let limited = legal.list_legal_documents(with(ListLegalDocumentsRequest {}, None, &from("192.0.2.1"))).await.unwrap_err();
    assert_eq!(limited.code(), Code::ResourceExhausted);
    assert!(legal.list_legal_documents(with(ListLegalDocumentsRequest {}, None, &from("192.0.2.2"))).await.is_ok(), "another address is not affected");
}

/// A one-shot HTTP stand-in for the places provider: answers every request with `status` and `body`.
async fn provider(status: &'static str, body: &'static str) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else { return };
            let mut received = Vec::new();
            let mut chunk = [0u8; 4096];
            // Read the headers and the declared body, then answer.
            loop {
                let n = socket.read(&mut chunk).await.unwrap_or(0);
                if n == 0 { break }
                received.extend_from_slice(&chunk[..n]);
                let text = String::from_utf8_lossy(&received).to_string();
                if let Some(split) = text.find("\r\n\r\n") {
                    let length = text.to_lowercase().split("content-length:").nth(1).and_then(|r| r.trim_start().split("\r\n").next().and_then(|v| v.trim().parse::<usize>().ok())).unwrap_or(0);
                    if received.len() >= split + 4 + length { break }
                }
            }
            let response = format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        }
    });
    format!("http://{address}")
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn address_search_needs_a_token_and_maps_the_provider_outcomes() {
    let served = serve(100).await;
    let mut auth = AuthServiceClient::new(served.channel.clone());
    let mut address = AddressSearchServiceClient::new(served.channel);
    let token = auth
        .signup(SignupRequest {
            firstname: "Ada".into(),
            surname: "Lovelace".into(),
            date_of_birth: "1990-05-05".into(),
            gender: "F".into(),
            email: "ada@example.test".into(),
            password: "Str0ng!Passw0rd".into(),
            terms_version: "1.0.0".into(),
            privacy_version: "1.0.0".into(),
            terms_accepted: true,
            privacy_accepted: true,
        })
        .await
        .unwrap()
        .into_inner()
        .access_token;
    let search = |text: &str| with(SearchAddressRequest { text: text.into(), latitude: None, longitude: None }, Some(&token), &[]);

    let anonymous = address.search_address(with(SearchAddressRequest { text: "rua".into(), latitude: None, longitude: None }, None, &[])).await.unwrap_err();
    assert_eq!(anonymous.code(), Code::Unauthenticated, "address search needs a token");

    unsafe { std::env::remove_var("GOOGLE_MAPS_ENABLED") };
    let off = address.search_address(search("rua augusta")).await.unwrap_err();
    assert_eq!((off.code(), error_key(&off)), (Code::PermissionDenied, ErrorKey::AddressSearchDisabled.as_str()), "the feature is off by default");

    unsafe {
        std::env::set_var("GOOGLE_MAPS_ENABLED", "true");
        std::env::set_var("GOOGLE_MAPS_API_KEY", "test-key");
    }
    let empty = address.search_address(search("   ")).await.unwrap_err();
    assert_eq!(empty.code(), Code::InvalidArgument, "an empty text is refused before the provider is called");

    let ok = provider(
        "200 OK",
        r#"{"places":[{"id":"p1","formattedAddress":"Rua Augusta 100, Sao Paulo","location":{"latitude":-23.55,"longitude":-46.63},"addressComponents":[{"longText":"100","shortText":"100","types":["street_number"]},{"longText":"Rua Augusta","shortText":"R. Augusta","types":["route"]},{"longText":"Sao Paulo","shortText":"Sao Paulo","types":["locality"]},{"longText":"Sao Paulo","shortText":"SP","types":["administrative_area_level_1"]},{"longText":"Brazil","shortText":"BR","types":["country"]}]}]}"#,
    )
    .await;
    unsafe { std::env::set_var("GOOGLE_PLACES_SEARCH_URL", &ok) };
    let found = address.search_address(search("rua augusta")).await.expect("provider answers").into_inner();
    assert_eq!(found.candidates.len(), 1);
    let candidate = &found.candidates[0];
    assert_eq!((candidate.address_line_1.as_str(), candidate.locality.as_str(), candidate.country_code.as_str()), ("Rua Augusta 100", "Sao Paulo", "BR"));

    let down = provider("503 Service Unavailable", "{}").await;
    unsafe { std::env::set_var("GOOGLE_PLACES_SEARCH_URL", &down) };
    let outage = address.search_address(search("rua augusta")).await.unwrap_err();
    assert_eq!((outage.code(), error_key(&outage)), (Code::Unavailable, ErrorKey::AddressSearchFailed.as_str()), "a provider outage is retryable");
    unsafe {
        std::env::remove_var("GOOGLE_PLACES_SEARCH_URL");
        std::env::remove_var("GOOGLE_MAPS_ENABLED");
    }
}
