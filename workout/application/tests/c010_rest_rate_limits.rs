//! C-010 (owner decision 2026-10-07): `/refresh` has a per-address limit like `/login` and `/signup`.
use application::{routes::authentication_routes::auth_routes, AppState};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn refresh_is_limited_per_address_after_twenty_calls_a_minute() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", "c010-access-secret");
        std::env::set_var("REFRESH_TOKEN_SECRET", "c010-refresh-secret");
    }
    let database = Database::connect(&database_url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();
    let state = AppState {
        conn: Arc::new(database),
    };
    let app = auth_routes(state.clone()).with_state(state);

    let call = |ip: &'static str| {
        let request = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri("/refresh")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .header("x-real-ip", ip)
            .body(axum::body::Body::from(r#"{"refresh_token":"not-a-token"}"#))
            .unwrap();
        app.clone().oneshot(request)
    };
    for attempt in 1..=20 {
        let status = call("192.0.2.10").await.unwrap().status();
        assert_eq!(
            status,
            axum::http::StatusCode::UNAUTHORIZED,
            "call {attempt} reaches the use case"
        );
    }
    assert_eq!(
        call("192.0.2.10").await.unwrap().status(),
        axum::http::StatusCode::TOO_MANY_REQUESTS,
        "the 21st call is refused"
    );
    assert_eq!(
        call("192.0.2.11").await.unwrap().status(),
        axum::http::StatusCode::UNAUTHORIZED,
        "another address is not affected"
    );
}
