use application::{AppState, routes::workout_routes::workout_routes};
use business::domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database};
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

fn access_token(email: &str, user_uuid: &str, person_id: i32, person_uuid: &str) -> String {
    let claims = Claims::new(
        email.to_string(),
        chrono::Utc::now().timestamp() + 3600,
        user_uuid.to_string(),
        format!("Person {person_id}"),
        person_id,
        person_uuid.to_string(),
        "default".to_string(),
        None,
        None,
    );
    encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(b"c004-http-test-secret"),
    )
    .unwrap()
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn workout_owner_routes_enforce_authenticated_identity_and_return_stable_errors() {
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
        ("ACCESS_TOKEN_SECRET", "c004-http-test-secret"),
        ("AUTH_RULES_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = Database::connect(&database_url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();

    database
        .execute_unprepared(
            r#"INSERT INTO person
                 (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
               VALUES
                 (1, '00000000-0000-0000-0000-000000000041', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-000000000042', 'Other', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user"
                 (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
               VALUES
                 (1, '10000000-0000-0000-0000-000000000041', 'Owner Person', 'owner@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000041', now(), now()),
                 (2, '10000000-0000-0000-0000-000000000042', 'Other Person', 'other@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000042', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
               VALUES
                 ('20000000-0000-0000-0000-000000000041', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000042', 1, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000043', 2, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000044', 2, 'privacy', '1.0.0', now(), '127.0.0.1');
               INSERT INTO workout
                 (id, uuid, owner_id, owner_uuid, name, description, difficulty, muscle_group, visibility, status, created_at, updated_at)
               VALUES
                 (1, '30000000-0000-0000-0000-000000000041', 1, '00000000-0000-0000-0000-000000000041', 'Owner Workout', 'Acceptance fixture', 'Easy', 'Chest', 'private', 'Accepted', now(), now());"#,
        )
        .await
        .unwrap();

    let state = AppState {
        conn: Arc::new(database),
    };
    let app = axum::Router::new()
        .nest("/workout/api/workouts", workout_routes(state.clone()))
        .with_state(state);

    let owner_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/workouts/owner/uuid/00000000-0000-0000-0000-000000000041")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        access_token(
                            "owner@example.test",
                            "10000000-0000-0000-0000-000000000041",
                            1,
                            "00000000-0000-0000-0000-000000000041",
                        )
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(owner_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(owner_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let workouts: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(workouts.as_array().unwrap().len(), 1);
    assert_eq!(workouts[0]["name"], "Owner Workout");

    let forbidden_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/workouts/owner/uuid/00000000-0000-0000-0000-000000000041")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        access_token(
                            "other@example.test",
                            "10000000-0000-0000-0000-000000000042",
                            2,
                            "00000000-0000-0000-0000-000000000042",
                        )
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        forbidden_response.status(),
        axum::http::StatusCode::FORBIDDEN
    );
    let body = axum::body::to_bytes(forbidden_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error["errorKey"], "WorkoutNotFound");

    let missing_response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/workouts/uuid/30000000-0000-0000-0000-000000000099")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        access_token(
                            "owner@example.test",
                            "10000000-0000-0000-0000-000000000041",
                            1,
                            "00000000-0000-0000-0000-000000000041",
                        )
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_response.status(), axum::http::StatusCode::NOT_FOUND);
    let body = axum::body::to_bytes(missing_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error["errorKey"], "WorkoutNotFound");
}
