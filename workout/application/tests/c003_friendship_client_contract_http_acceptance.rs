use application::{
    routes::{friend_routes::friend_routes, person_routes::person_routes},
    AppState,
};
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
        &EncodingKey::from_secret(b"c003-http-test-secret"),
    )
    .unwrap()
}

/// Verifies SYS-C003-005 / TC-006: REST clients receive stable success,
/// conflict, forbidden, not-found, and validation outcomes for friendship
/// operations. The gRPC friendship protocol compiles (`cargo check -p
/// integration`) but is not exercised here; no live gRPC test harness exists
/// in this codebase yet, consistent with the precedent set by C-002/C-007.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn friendship_rest_client_contract_smoke() {
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
        ("ACCESS_TOKEN_SECRET", "c003-http-test-secret"),
        ("AUTH_RULES_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = Database::connect(&database_url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();

    // Three people: a sender, a receiver, and an unrelated bystander who
    // will be used to exercise forbidden/not-found transitions.
    database
        .execute_unprepared(
            r#"INSERT INTO person
                 (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
               VALUES
                 (1, '00000000-0000-0000-0000-000000000091', 'Sender', 'Person', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-000000000092', 'Receiver', 'Person', '1990-01-01', 'X', now(), now()),
                 (3, '00000000-0000-0000-0000-000000000093', 'Bystander', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user"
                 (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
               VALUES
                 (1, '10000000-0000-0000-0000-000000000091', 'Sender Person', 'sender-c003@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000091', now(), now()),
                 (2, '10000000-0000-0000-0000-000000000092', 'Receiver Person', 'receiver-c003@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000092', now(), now()),
                 (3, '10000000-0000-0000-0000-000000000093', 'Bystander Person', 'bystander-c003@example.test', 'unused', false, true, 3, '00000000-0000-0000-0000-000000000093', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
               VALUES
                 ('20000000-0000-0000-0000-000000000091', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000092', 1, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000093', 2, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000094', 2, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000095', 3, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000096', 3, 'privacy', '1.0.0', now(), '127.0.0.1');"#,
        )
        .await
        .unwrap();

    let state = AppState {
        conn: Arc::new(database),
    };
    let app = axum::Router::new()
        .nest("/workout/api/friends", friend_routes(state.clone()))
        .nest("/workout/api/people", person_routes(state.clone()))
        .with_state(state);

    let sender_token = access_token(
        "sender-c003@example.test",
        "10000000-0000-0000-0000-000000000091",
        1,
        "00000000-0000-0000-0000-000000000091",
    );
    let receiver_token = access_token(
        "receiver-c003@example.test",
        "10000000-0000-0000-0000-000000000092",
        2,
        "00000000-0000-0000-0000-000000000092",
    );
    let bystander_token = access_token(
        "bystander-c003@example.test",
        "10000000-0000-0000-0000-000000000093",
        3,
        "00000000-0000-0000-0000-000000000093",
    );

    let request = |method: &str, uri: String, token: &str| {
        axum::http::Request::builder()
            .method(method)
            .uri(uri)
            .header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"))
            .body(axum::body::Body::empty())
            .unwrap()
    };

    // 1. Validation: a person cannot send a friend request to themselves.
    let self_request = app
        .clone()
        .oneshot(request(
            "PUT",
            "/workout/api/friends/request/1".to_string(),
            &sender_token,
        ))
        .await
        .unwrap();
    assert_eq!(
        self_request.status(),
        axum::http::StatusCode::BAD_REQUEST,
        "self-request must be rejected as a validation error"
    );

    // 2. Success: sender requests, receiver accepts.
    let send_response = app
        .clone()
        .oneshot(request(
            "PUT",
            "/workout/api/friends/request/2".to_string(),
            &sender_token,
        ))
        .await
        .unwrap();
    assert_eq!(send_response.status(), axum::http::StatusCode::OK);

    // 3. Forbidden: the sender cannot accept their own outgoing request —
    //    only the receiver may transition it to Accepted.
    let sender_tries_to_accept = app
        .clone()
        .oneshot(request(
            "PUT",
            "/workout/api/friends/accept/2".to_string(),
            &sender_token,
        ))
        .await
        .unwrap();
    assert_eq!(
        sender_tries_to_accept.status(),
        axum::http::StatusCode::FORBIDDEN
    );

    let accept_response = app
        .clone()
        .oneshot(request(
            "PUT",
            "/workout/api/friends/accept/1".to_string(),
            &receiver_token,
        ))
        .await
        .unwrap();
    assert_eq!(accept_response.status(), axum::http::StatusCode::OK);

    // 4. Conflict: sending a second request once already accepted-friends.
    let duplicate_request = app
        .clone()
        .oneshot(request(
            "PUT",
            "/workout/api/friends/request/2".to_string(),
            &sender_token,
        ))
        .await
        .unwrap();
    assert_eq!(duplicate_request.status(), axum::http::StatusCode::CONFLICT);

    // 5. Not found: the bystander tries to accept/cancel a friend request
    //    that does not involve them at all.
    let bystander_cancel = app
        .clone()
        .oneshot(request(
            "PUT",
            "/workout/api/friends/cancel/2".to_string(),
            &bystander_token,
        ))
        .await
        .unwrap();
    assert_eq!(
        bystander_cancel.status(),
        axum::http::StatusCode::NOT_FOUND,
        "no pending/relevant friend request exists between the bystander and person 2"
    );

    // 6. Success: the accepted friendship is now visible in the owner-scoped
    //    relationship list and friend page for the sender.
    let relationships_response = app
        .clone()
        .oneshot(request(
            "GET",
            "/workout/api/friends/relationships/id/1".to_string(),
            &sender_token,
        ))
        .await
        .unwrap();
    assert_eq!(relationships_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(relationships_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let relationships: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let relationships = relationships.as_array().unwrap();
    assert_eq!(relationships.len(), 1);
    assert_eq!(relationships[0]["friendId"], 2);

    // 7. Forbidden: the bystander cannot read the sender's relationship list.
    let forbidden_relationships = app
        .clone()
        .oneshot(request(
            "GET",
            "/workout/api/friends/relationships/id/1".to_string(),
            &bystander_token,
        ))
        .await
        .unwrap();
    assert_eq!(
        forbidden_relationships.status(),
        axum::http::StatusCode::FORBIDDEN
    );

    // 7b. Owner/friendship scope on person-profile routes: a non-friend is
    //     forbidden, an accepted friend is allowed.
    let status_of = |uri: &str, token: &str| {
        let req = request("GET", uri.to_string(), token);
        let app = app.clone();
        async move { app.oneshot(req).await.unwrap().status() }
    };
    let status = status_of("/workout/api/people/me/friend/1", &bystander_token).await;
    assert_eq!(status, axum::http::StatusCode::FORBIDDEN);
    let status = status_of("/workout/api/people/me/friend/2", &sender_token).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    let status = status_of(
        "/workout/api/people/id/1/mentionable-friends?query=a",
        &bystander_token,
    )
    .await;
    assert_eq!(status, axum::http::StatusCode::FORBIDDEN);
    let status = status_of(
        "/workout/api/people/id/1/mentionable-friends?query=a",
        &sender_token,
    )
    .await;
    assert_eq!(status, axum::http::StatusCode::OK);

    // 8. Success: removing the friendship succeeds, and a second removal of
    //    the now-nonexistent friendship returns not-found.
    let remove_response = app
        .clone()
        .oneshot(request(
            "DELETE",
            "/workout/api/friends/2".to_string(),
            &sender_token,
        ))
        .await
        .unwrap();
    assert_eq!(remove_response.status(), axum::http::StatusCode::OK);

    let remove_again_response = app
        .oneshot(request(
            "DELETE",
            "/workout/api/friends/2".to_string(),
            &sender_token,
        ))
        .await
        .unwrap();
    assert_eq!(
        remove_again_response.status(),
        axum::http::StatusCode::NOT_FOUND
    );
}
