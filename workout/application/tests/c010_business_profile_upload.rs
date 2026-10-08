//! C-010 task 5: asking for a business-profile image upload link without an Active Business Profile is a
//! client error (`400`), not the `500` the missing request extension used to give.
use application::{AppState, routes::business_profile_routes::business_profile_routes};
use business::domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database};
use std::sync::Arc;
use tower::ServiceExt;

const SECRET: &str = "c010-upload-secret";

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn an_image_upload_link_needs_an_active_business_profile() {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("TOKEN_REVOCATION_ENABLED", "false");
    }
    let database = Database::connect(&url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();
    database
        .execute_unprepared(
            r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
                 VALUES (1, '00000000-0000-0000-0000-0000000000c1', 'Upload', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user" (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
                 VALUES (1, '10000000-0000-0000-0000-0000000000c1', 'Upload Person', 'upload@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-0000000000c1', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
                 SELECT md5('c010-' || d.document)::uuid, 1, d.document, '1.0.0', now(), '127.0.0.1'
                 FROM (VALUES ('terms'), ('privacy')) AS d(document);"#,
        )
        .await
        .unwrap();
    let claims = Claims::new(
        "upload@example.test".to_string(),
        chrono::Utc::now().timestamp() + 3600,
        "10000000-0000-0000-0000-0000000000c1".to_string(),
        "Upload Person".to_string(),
        1,
        "00000000-0000-0000-0000-0000000000c1".to_string(),
        "default".to_string(),
        None,
        None,
    );
    let token = encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(SECRET.as_bytes())).unwrap();
    let state = AppState { conn: Arc::new(database) };
    let app = business_profile_routes(state.clone()).with_state(state);
    let request = axum::http::Request::builder()
        .uri("/upload/logo")
        .header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"))
        .body(axum::body::Body::empty())
        .unwrap();
    let status = app.oneshot(request).await.unwrap().status();
    assert_eq!(status, axum::http::StatusCode::BAD_REQUEST, "no Active Business Profile is a client error");
}

/// Task 7: over REST the active-profile read is a client error without one, and a profile's street address and
/// owner ids reach the owner and nobody outside the team.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn rest_serves_the_coarse_address_to_outsiders_and_needs_an_active_profile_for_active() {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("TOKEN_REVOCATION_ENABLED", "false");
    }
    let database = Database::connect(&url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();
    database
        .execute_unprepared(
            r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
                 (1, '00000000-0000-0000-0000-0000000000c1', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-0000000000c2', 'Outsider', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user" (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at) VALUES
                 (1, '10000000-0000-0000-0000-0000000000c1', 'Owner Person', 'owner@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-0000000000c1', now(), now()),
                 (2, '10000000-0000-0000-0000-0000000000c2', 'Outsider Person', 'outsider@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-0000000000c2', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
                 SELECT md5('c010-' || p.id || d.document)::uuid, p.id, d.document, '1.0.0', now(), '127.0.0.1'
                 FROM (VALUES (1), (2)) AS p(id), (VALUES ('terms'), ('privacy')) AS d(document);
               INSERT INTO business_profile (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at)
                 VALUES (1, '30000000-0000-0000-0000-0000000000c1', 1, '00000000-0000-0000-0000-0000000000c1', '999', 'Owner Gym', 'Professional', now(), now());
               INSERT INTO business_profile_address (id, uuid, business_profile_id, address_line1, locality, administrative_area, country_code, created_at, updated_at)
                 VALUES (1, '20000000-0000-0000-0000-0000000000c1', 1, 'Main St 10', 'Sao Paulo', 'SP', 'BR', now(), now());"#,
        )
        .await
        .unwrap();
    let token = |email: &str, uuid: &str, id: i32, person_uuid: &str| {
        let claims = Claims::new(email.to_string(), chrono::Utc::now().timestamp() + 3600, uuid.to_string(), format!("Person {id}"), id, person_uuid.to_string(), "default".to_string(), None, None);
        encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(SECRET.as_bytes())).unwrap()
    };
    let owner = token("owner@example.test", "10000000-0000-0000-0000-0000000000c1", 1, "00000000-0000-0000-0000-0000000000c1");
    let outsider = token("outsider@example.test", "10000000-0000-0000-0000-0000000000c2", 2, "00000000-0000-0000-0000-0000000000c2");
    let state = AppState { conn: Arc::new(database) };
    let app = business_profile_routes(state.clone()).with_state(state);
    let get = |uri: &'static str, bearer: String| {
        let request = axum::http::Request::builder()
            .uri(uri)
            .header(axum::http::header::AUTHORIZATION, format!("Bearer {bearer}"))
            .body(axum::body::Body::empty())
            .unwrap();
        app.clone().oneshot(request)
    };
    let json = |response: axum::http::Response<axum::body::Body>| async move {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()
    };

    let as_owner = json(get("/id/1", owner.clone()).await.unwrap()).await;
    assert_eq!(as_owner["addresses"][0]["addressLine1"], "Main St 10");
    assert_eq!(as_owner["ownerUuid"], "00000000-0000-0000-0000-0000000000c1");
    let as_outsider = json(get("/id/1", outsider.clone()).await.unwrap()).await;
    assert_eq!(as_outsider["addresses"][0]["locality"], "Sao Paulo", "the city is public");
    assert!(as_outsider["addresses"][0]["addressLine1"].as_str().is_none_or(str::is_empty), "{as_outsider}");
    assert!(as_outsider["ownerUuid"].as_str().is_none_or(str::is_empty) && as_outsider["taxId"].as_str().is_none_or(str::is_empty), "{as_outsider}");

    let active = get("/active", owner).await.unwrap().status();
    assert_eq!(active, axum::http::StatusCode::BAD_REQUEST, "no Active Business Profile is a client error");
}
