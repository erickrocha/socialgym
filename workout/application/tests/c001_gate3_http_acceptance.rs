use application::{routes::authentication_routes::auth_routes, AppState};
use business::domain::access_token::Claims;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, EntityTrait};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceExt;

static TEST_LOCK: Mutex<()> = Mutex::const_new(());

struct TestEnvironment(Vec<(&'static str, Option<std::ffi::OsString>)>);

impl TestEnvironment {
    fn set(values: &[(&'static str, &'static str)]) -> Self {
        let previous = values
            .iter()
            .map(|(key, _)| (*key, env::var_os(key)))
            .collect();
        for (key, value) in values {
            unsafe { env::set_var(key, value) };
        }
        Self(previous)
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        for (key, value) in &self.0 {
            unsafe {
                match value {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
        }
    }
}

async fn disposable_database() -> sea_orm::DatabaseConnection {
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    let database_name = database_url
        .split('?')
        .next()
        .unwrap_or(&database_url)
        .rsplit('/')
        .next()
        .unwrap_or_default();
    assert_eq!(
        database_name, "workout_test",
        "refusing to refresh a database not named workout_test"
    );
    let database = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();
    database
}

fn signup_payload(
    email: &str,
    terms_version: &str,
    privacy_version: &str,
    accepted: bool,
) -> String {
    serde_json::json!({
        "firstname": "Gate",
        "surname": "Three",
        "dateOfBirth": "1990-01-01",
        "gender": "X",
        "email": email,
        "password": "Str0ng!Password",
        "termsVersion": terms_version,
        "privacyVersion": privacy_version,
        "termsAccepted": accepted,
        "privacyAccepted": accepted
    })
    .to_string()
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn tc002_signup_rejects_missing_and_outdated_consent() {
    let _guard = TEST_LOCK.lock().await;
    let _environment = TestEnvironment::set(&[
        ("ACCESS_TOKEN_SECRET", "c001-tc002-access-secret"),
        ("REFRESH_TOKEN_SECRET", "c001-tc002-refresh-secret"),
        ("TOKEN_REVOCATION_ENABLED", "false"),
        ("AUTH_RULES_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = disposable_database().await;
    let state = AppState {
        conn: Arc::new(database.clone()),
    };
    let app = auth_routes(state.clone()).with_state(state);

    for (email, terms_version, accepted) in [
        ("tc002-missing@example.test", "1.0.0", false),
        ("tc002-outdated@example.test", "0.9.0", true),
    ] {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(axum::http::Method::POST)
                    .uri("/signup")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(signup_payload(
                        email,
                        terms_version,
                        "1.0.0",
                        accepted,
                    )))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(error["errorKey"], "CONSENT_REQUIRED");
    }

    assert_eq!(
        entity::user_entity::Entity::find()
            .all(&database)
            .await
            .unwrap()
            .len(),
        0,
        "rejected registrations must not create users"
    );
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn tc008_token_lifecycle_refreshes_without_consent_and_rejects_reuse() {
    let _guard = TEST_LOCK.lock().await;
    let _environment = TestEnvironment::set(&[
        ("ACCESS_TOKEN_SECRET", "c001-tc008-access-secret"),
        ("REFRESH_TOKEN_SECRET", "c001-tc008-refresh-secret"),
        ("TOKEN_REVOCATION_ENABLED", "true"),
        ("AUTH_RULES_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = disposable_database().await;
    let state = AppState {
        conn: Arc::new(database.clone()),
    };
    let app = auth_routes(state.clone()).with_state(state);

    let signup_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(axum::http::Method::POST)
                .uri("/signup")
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(signup_payload(
                    "tc008@example.test",
                    "1.0.0",
                    "1.0.0",
                    true,
                )))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(signup_response.status(), axum::http::StatusCode::OK);

    let login_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(axum::http::Method::POST)
                .uri("/login")
                .header(
                    axum::http::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(axum::body::Body::from(
                    "email=tc008%40example.test&password=Str0ng%21Password",
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(login_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(login_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let tokens: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let access_token = tokens["accessToken"].as_str().unwrap();
    let original_refresh_token = tokens["refreshToken"].as_str().unwrap();

    let access_claims = decode::<Claims>(
        access_token,
        &DecodingKey::from_secret(b"c001-tc008-access-secret"),
        &Validation::new(Algorithm::HS512),
    )
    .unwrap()
    .claims;
    let refresh_claims = decode::<Claims>(
        original_refresh_token,
        &DecodingKey::from_secret(b"c001-tc008-refresh-secret"),
        &Validation::new(Algorithm::HS512),
    )
    .unwrap()
    .claims;
    assert!((10_799..=10_800).contains(&(access_claims.exp - access_claims.iat)));
    assert!((604_799..=604_800).contains(&(refresh_claims.exp - refresh_claims.iat)));
    assert_eq!(tokens["expireIn"].as_i64(), Some(access_claims.exp));

    database
        .execute_unprepared("DELETE FROM consent")
        .await
        .unwrap();

    let refresh_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(axum::http::Method::POST)
                .uri("/refresh")
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(
                    serde_json::json!({ "refresh_token": original_refresh_token }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(refresh_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(refresh_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let rotated: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let rotated_refresh_token = rotated["refreshToken"].as_str().unwrap();
    let rotated_claims = decode::<Claims>(
        rotated_refresh_token,
        &DecodingKey::from_secret(b"c001-tc008-refresh-secret"),
        &Validation::new(Algorithm::HS512),
    )
    .unwrap()
    .claims;
    assert_ne!(rotated_claims.jti, refresh_claims.jti);

    let reuse_response = app
        .oneshot(
            axum::http::Request::builder()
                .method(axum::http::Method::POST)
                .uri("/refresh")
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(
                    serde_json::json!({ "refresh_token": original_refresh_token }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        reuse_response.status(),
        axum::http::StatusCode::UNAUTHORIZED
    );
}
