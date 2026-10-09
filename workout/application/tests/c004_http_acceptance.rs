use application::{routes::workout_routes::workout_routes, AppState};
use business::domain::access_token::Claims;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
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

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn workout_reads_hide_private_workouts_and_private_composed_exercises() {
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

    // Person 1 owns a private workout (1) and a public workout (2). The public
    // workout composes a public exercise (1) and a private exercise (2).
    database
        .execute_unprepared(
            r#"INSERT INTO person
                 (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
               VALUES
                 (1, '00000000-0000-0000-0000-000000000051', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-000000000052', 'Other', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user"
                 (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
               VALUES
                 (1, '10000000-0000-0000-0000-000000000051', 'Owner Person', 'owner51@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000051', now(), now()),
                 (2, '10000000-0000-0000-0000-000000000052', 'Other Person', 'other52@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000052', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
               VALUES
                 ('20000000-0000-0000-0000-000000000051', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000052', 1, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000053', 2, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000054', 2, 'privacy', '1.0.0', now(), '127.0.0.1');
               INSERT INTO workout
                 (id, uuid, owner_id, owner_uuid, name, description, difficulty, muscle_group, visibility, status, created_at, updated_at)
               VALUES
                 (1, '30000000-0000-0000-0000-000000000051', 1, '00000000-0000-0000-0000-000000000051', 'Private Workout', 'x', 'Easy', 'Chest', 'private', 'Accepted', now(), now()),
                 (2, '30000000-0000-0000-0000-000000000052', 1, '00000000-0000-0000-0000-000000000051', 'Public Workout', 'x', 'Easy', 'Chest', 'public', 'Accepted', now(), now());
               INSERT INTO exercise
                 (id, uuid, name, category, owner_id, owner_uuid, owner_name, sets, reps_or_duration, description, visibility, created_at, updated_at)
               VALUES
                 (1, '40000000-0000-0000-0000-000000000051', 'Public Exercise', 'Force', 1, '00000000-0000-0000-0000-000000000051', 'Owner', 3, 10, 'x', 'public', now(), now()),
                 (2, '40000000-0000-0000-0000-000000000052', 'Private Exercise', 'Force', 1, '00000000-0000-0000-0000-000000000051', 'Owner', 3, 10, 'x', 'private', now(), now());
               INSERT INTO workout_exercise
                 (id, uuid, workout_id, exercise_id, order_index, created_at, updated_at)
               VALUES
                 (1, '50000000-0000-0000-0000-000000000051', 2, 1, 0, now(), now()),
                 (2, '50000000-0000-0000-0000-000000000052', 2, 2, 1, now(), now());"#,
        )
        .await
        .unwrap();

    let state = AppState {
        conn: Arc::new(database),
    };
    let app = axum::Router::new()
        .nest("/workout/api/workouts", workout_routes(state.clone()))
        .with_state(state);
    let owner = access_token(
        "owner51@example.test",
        "10000000-0000-0000-0000-000000000051",
        1,
        "00000000-0000-0000-0000-000000000051",
    );
    let other = access_token(
        "other52@example.test",
        "10000000-0000-0000-0000-000000000052",
        2,
        "00000000-0000-0000-0000-000000000052",
    );

    let call = |method: &'static str, uri: &'static str, token: &str| {
        let request = axum::http::Request::builder()
            .method(method)
            .uri(uri)
            .header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"))
            .body(axum::body::Body::empty())
            .unwrap();
        let app = app.clone();
        async move {
            let response = app.oneshot(request).await.unwrap();
            let status = response.status();
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let json = serde_json::from_slice::<serde_json::Value>(&bytes).unwrap_or_default();
            (status, json)
        }
    };
    let names = |value: &serde_json::Value| -> Vec<String> {
        value
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item["name"].as_str().unwrap().to_string())
            .collect()
    };

    // The owner lists both workouts.
    let (status, body) = call("GET", "/workout/api/workouts/1", &owner).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 2);

    // Another person lists only the public workout, without the private exercise.
    let (status, body) = call("GET", "/workout/api/workouts/1", &other).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(names(&body), vec!["Public Workout"]);
    assert_eq!(names(&body[0]["exercises"]), vec!["Public Exercise"]);

    // A direct read of a private workout is indistinguishable from a missing one.
    let (status, _) = call("GET", "/workout/api/workouts/id/1", &other).await;
    assert_eq!(status, axum::http::StatusCode::NOT_FOUND);

    // A public workout read by id hides the private composed exercise too.
    let (status, body) = call("GET", "/workout/api/workouts/id/2", &other).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(names(&body["exercises"]), vec!["Public Exercise"]);
    let (_, body) = call("GET", "/workout/api/workouts/id/2", &owner).await;
    assert_eq!(body["exercises"].as_array().unwrap().len(), 2);

    // The exercise collection of a public workout is filtered the same way.
    let (status, body) = call("GET", "/workout/api/workouts/2/exercises", &other).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(names(&body), vec!["Public Exercise"]);

    // A non-owner cannot delete someone else's workout.
    let (status, _) = call("DELETE", "/workout/api/workouts/id/2", &other).await;
    assert_eq!(status, axum::http::StatusCode::FORBIDDEN);
}
