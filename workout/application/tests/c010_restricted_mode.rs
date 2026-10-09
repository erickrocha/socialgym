//! C-010 decision W15 (2026-10-07): a person without current Terms and Privacy consent can still cancel a
//! pending account deletion over REST, like the other recovery paths; ordinary routes stay closed.
use application::{routes::person_routes::person_routes, AppState};
use business::domain::access_token::Claims;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database};
use std::sync::Arc;
use tower::ServiceExt;

const SECRET: &str = "c010-restricted-secret";

fn token() -> String {
    let claims = Claims::new(
        "restricted@example.test".to_string(),
        chrono::Utc::now().timestamp() + 3600,
        "10000000-0000-0000-0000-0000000000b1".to_string(),
        "Restricted Person".to_string(),
        1,
        "00000000-0000-0000-0000-0000000000b1".to_string(),
        "default".to_string(),
        None,
        None,
    );
    encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap()
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn cancel_deletion_is_a_recovery_path_without_consent() {
    let url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("TOKEN_REVOCATION_ENABLED", "false");
    }
    let database = Database::connect(&url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();
    database
        .execute_unprepared(
            r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
                 VALUES (1, '00000000-0000-0000-0000-0000000000b1', 'Restricted', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user" (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
                 VALUES (1, '10000000-0000-0000-0000-0000000000b1', 'Restricted Person', 'restricted@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-0000000000b1', now(), now());"#,
        )
        .await
        .unwrap();
    let state = AppState {
        conn: Arc::new(database),
    };
    let app = person_routes(state.clone()).with_state(state);
    let call = |method: axum::http::Method, uri: &'static str| {
        let request = axum::http::Request::builder()
            .method(method)
            .uri(uri)
            .header(
                axum::http::header::AUTHORIZATION,
                format!("Bearer {}", token()),
            )
            .body(axum::body::Body::empty())
            .unwrap();
        app.clone().oneshot(request)
    };

    // The person never accepted anything: no consent rows exist.
    let ordinary = call(axum::http::Method::GET, "/me").await.unwrap().status();
    assert_eq!(
        ordinary,
        axum::http::StatusCode::FORBIDDEN,
        "an ordinary route is closed without consent"
    );
    let cancel = call(axum::http::Method::POST, "/me/account/cancel-deletion")
        .await
        .unwrap()
        .status();
    assert_ne!(
        cancel,
        axum::http::StatusCode::FORBIDDEN,
        "cancelling a deletion is a recovery path"
    );
    let request = call(axum::http::Method::POST, "/me/account/delete")
        .await
        .unwrap()
        .status();
    assert_ne!(
        request,
        axum::http::StatusCode::FORBIDDEN,
        "so is asking for one"
    );
}
