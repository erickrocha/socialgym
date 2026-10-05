use application::{routes::authentication_routes::auth_routes, AppState};
use entity::{consent_entity, person_entity, user_entity};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, EntityTrait};
use std::env;
use std::sync::Arc;
use tower::ServiceExt;

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

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn signup_returns_an_access_token_for_valid_registration() {
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

    let _environment = TestEnvironment::set(&[
        ("ACCESS_TOKEN_SECRET", "c001-signup-test-secret"),
        ("REFRESH_TOKEN_SECRET", "c001-signup-refresh-secret"),
        ("TOKEN_REVOCATION_ENABLED", "false"),
        ("AUTH_RULES_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = Database::connect(&database_url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();

    let state = AppState {
        conn: Arc::new(database.clone()),
    };
    let app = auth_routes(state.clone()).with_state(state);
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method(axum::http::Method::POST)
                .uri("/signup")
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(
                    serde_json::json!({
                        "firstname": "Acceptance",
                        "surname": "User",
                        "dateOfBirth": "1990-01-01",
                        "gender": "X",
                        "email": "tc001-signup@example.test",
                        "password": "Str0ng!Password",
                        "termsVersion": "1.0.0",
                        "privacyVersion": "1.0.0",
                        "termsAccepted": true,
                        "privacyAccepted": true
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let tokens: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(tokens["accessToken"]
        .as_str()
        .is_some_and(|token| !token.is_empty()));

    assert_eq!(
        person_entity::Entity::find()
            .all(&database)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        user_entity::Entity::find()
            .all(&database)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        consent_entity::Entity::find()
            .all(&database)
            .await
            .unwrap()
            .len(),
        2
    );
}
