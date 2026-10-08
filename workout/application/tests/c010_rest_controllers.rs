//! C-010 task 14: the REST controllers C-010 added or changed (consents, data exports, workout composition,
//! business-profile reads) run in process against a disposable database, so their coverage is measured
//! (the contract tests run them in a container, which the coverage run does not see).
use application::routes::{person_routes::person_routes, workout_routes::workout_routes};
use application::AppState;
use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use business::domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

const SECRET: &str = "c010-controllers-secret";
const ALICE: &str = "00000000-0000-0000-0000-0000000000a1";
const CAROL: &str = "00000000-0000-0000-0000-0000000000a3";

fn token(id: i32, person_uuid: &str) -> String {
    let claims = Claims::new(
        format!("p{id}@example.test"),
        chrono::Utc::now().timestamp() + 3600,
        format!("10000000-0000-0000-0000-0000000000a{id}"),
        format!("Person {id}"),
        id,
        person_uuid.to_string(),
        "default".to_string(),
        None,
        None,
    );
    encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(SECRET.as_bytes())).unwrap()
}

async fn call(app: &axum::Router, method: Method, uri: &str, token: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut request = Request::builder().method(method).uri(uri).header(header::AUTHORIZATION, format!("Bearer {token}"));
    let body = match body {
        Some(value) => {
            request = request.header(header::CONTENT_TYPE, "application/json");
            Body::from(value.to_string())
        }
        None => Body::empty(),
    };
    let response = app.clone().oneshot(request.body(body).unwrap()).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn consents_exports_and_workout_composition_over_rest() {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("TOKEN_REVOCATION_ENABLED", "false");
        std::env::set_var("TERMS_VERSION", "1.0.0");
        std::env::set_var("PRIVACY_VERSION", "1.0.0");
        std::env::set_var("HEALTH_DATA_CONSENT_VERSION", "1.0.0");
    }
    let database = Database::connect(&url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();
    database
        .execute_unprepared(&format!(
            r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
                 (1, '{ALICE}', 'Alice', 'Owner', '1990-01-01', 'X', now(), now()),
                 (3, '{CAROL}', 'Carol', 'Stranger', '1990-01-01', 'X', now(), now());
               INSERT INTO "user" (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at) VALUES
                 (1, '10000000-0000-0000-0000-0000000000a1', 'Person 1', 'p1@example.test', 'unused', false, true, 1, '{ALICE}', now(), now()),
                 (3, '10000000-0000-0000-0000-0000000000a3', 'Person 3', 'p3@example.test', 'unused', false, true, 3, '{CAROL}', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
                 SELECT md5('c010-' || p.id || d.document)::uuid, p.id, d.document, '1.0.0', now(), '127.0.0.1'
                 FROM (VALUES (1), (3)) AS p(id), (VALUES ('terms'), ('privacy')) AS d(document);
               INSERT INTO workout (id, uuid, owner_id, owner_uuid, name, difficulty, muscle_group, visibility, status, created_at, updated_at)
                 VALUES (1, '40000000-0000-0000-0000-0000000000a1', 1, '{ALICE}', 'Leg day', 'Soft', 'legs', 'Private', 'Accepted', now(), now());"#
        ))
        .await
        .unwrap();
    let state = AppState { conn: Arc::new(database) };
    let people = person_routes(state.clone()).with_state(state.clone());
    let workouts = workout_routes(state.clone()).with_state(state.clone());
    let (alice, carol) = (token(1, ALICE), token(3, CAROL));

    // Consents: listed, pending, accepted once (a second time is a conflict) and revoked.
    let (status, list) = call(&people, Method::GET, "/me/consents", &alice, None).await;
    assert_eq!((status, list.as_array().map(Vec::len)), (StatusCode::OK, Some(2)));
    let (status, pending) = call(&people, Method::GET, "/me/consents/pending", &alice, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(pending.as_array().unwrap().iter().any(|p| p["document"] == "health_data"));
    let accept = json!({"document": "health_data", "version": "1.0.0", "accepted": true});
    assert_eq!(call(&people, Method::POST, "/me/consents", &alice, Some(accept.clone())).await.0, StatusCode::OK);
    assert_eq!(call(&people, Method::POST, "/me/consents", &alice, Some(accept)).await.0, StatusCode::CONFLICT);
    let refuse = json!({"document": "health_data", "version": "1.0.0", "accepted": false});
    assert_eq!(call(&people, Method::POST, "/me/consents", &carol, Some(refuse)).await.0, StatusCode::BAD_REQUEST, "accepted=false is not a consent");
    assert!(call(&people, Method::DELETE, "/me/consents/health_data", &alice, None).await.0.is_success());

    // Data exports: a job is created and listed, read by id, and not downloadable before it is ready.
    let (status, created) = call(&people, Method::POST, "/me/data-exports", &alice, None).await;
    assert_eq!(status, StatusCode::OK, "{created}");
    let id = created["id"].as_str().unwrap().to_string();
    let (status, listed) = call(&people, Method::GET, "/me/data-exports", &alice, None).await;
    assert_eq!((status, listed.as_array().map(Vec::len)), (StatusCode::OK, Some(1)));
    assert_eq!(call(&people, Method::GET, &format!("/me/data-exports/{id}"), &alice, None).await.0, StatusCode::OK);
    assert_eq!(call(&people, Method::GET, &format!("/me/data-exports/{id}/download"), &alice, None).await.0, StatusCode::CONFLICT, "a job still running has nothing to download");
    assert_eq!(call(&people, Method::GET, &format!("/me/data-exports/{id}"), &carol, None).await.0, StatusCode::NOT_FOUND, "another person's export is not found");
    assert_eq!(call(&people, Method::GET, "/me/data-exports/not-a-uuid", &alice, None).await.0, StatusCode::BAD_REQUEST);

    // Composing exercises into a workout by id: the owner may; a stranger is told there is no such workout.
    let exercise = json!([{"name": "Squat", "ownerId": 1, "ownerName": "Alice", "ownerUuid": ALICE, "category": "Force", "sets": 3, "repsOrDuration": 10, "visibility": "Private"}]);
    let (status, added) = call(&workouts, Method::POST, "/1/exercises", &alice, Some(exercise.clone())).await;
    assert_eq!(status, StatusCode::CREATED, "{added}");
    assert_eq!(added.as_array().map(Vec::len), Some(1));
    assert_eq!(call(&workouts, Method::POST, "/1/exercises", &carol, Some(exercise.clone())).await.0, StatusCode::NOT_FOUND);
    assert_eq!(call(&workouts, Method::POST, "/99/exercises", &alice, Some(exercise.clone())).await.0, StatusCode::NOT_FOUND);
    let (status, by_uuid) = call(&workouts, Method::POST, "/uuid/40000000-0000-0000-0000-0000000000a1/exercises", &alice, Some(exercise)).await;
    assert_eq!(status, StatusCode::OK, "{by_uuid}");
    let (status, composed) = call(&workouts, Method::GET, "/1/exercises", &alice, None).await;
    assert_eq!((status, composed.as_array().map(Vec::len)), (StatusCode::OK, Some(2)));
}
