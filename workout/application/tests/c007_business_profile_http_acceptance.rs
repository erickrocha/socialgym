use application::{
    routes::{
        business_profile_routes::business_profile_routes, team_member_routes::team_member_routes,
    },
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
        &EncodingKey::from_secret(b"c007-http-test-secret"),
    )
    .unwrap()
}

/// Verifies the business-profile-and-professional-discovery HTTP surface
/// (C-007): deletion is owner-only, and the public discovery search never
/// returns results without a query or a location filter.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn business_profile_delete_and_discover_routes_enforce_ownership_and_search_filters() {
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
        ("ACCESS_TOKEN_SECRET", "c007-http-test-secret"),
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
                 (1, '00000000-0000-0000-0000-000000000061', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-000000000062', 'Other', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user"
                 (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
               VALUES
                 (1, '10000000-0000-0000-0000-000000000061', 'Owner Person', 'owner-bp@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000061', now(), now()),
                 (2, '10000000-0000-0000-0000-000000000062', 'Other Person', 'other-bp@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000062', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
               VALUES
                 ('20000000-0000-0000-0000-000000000061', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000062', 1, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000063', 2, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000064', 2, 'privacy', '1.0.0', now(), '127.0.0.1');
               INSERT INTO business_profile
                 (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at)
               VALUES
                 (1, '30000000-0000-0000-0000-000000000061', 1, '00000000-0000-0000-0000-000000000061', '999', 'Discoverable Gym', 'Professional', now(), now());"#,
        )
        .await
        .unwrap();

    let state = AppState {
        conn: Arc::new(database),
    };
    let app = axum::Router::new()
        .nest(
            "/workout/api/business-profiles",
            business_profile_routes(state.clone()),
        )
        .with_state(state);

    // Non-owner cannot delete the business profile.
    let forbidden_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("DELETE")
                .uri("/workout/api/business-profiles/id/1")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        access_token(
                            "other-bp@example.test",
                            "10000000-0000-0000-0000-000000000062",
                            2,
                            "00000000-0000-0000-0000-000000000062",
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

    // Discovery search with no query and no location never returns everything.
    let empty_discover_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/business-profiles/discover")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        access_token(
                            "owner-bp@example.test",
                            "10000000-0000-0000-0000-000000000061",
                            1,
                            "00000000-0000-0000-0000-000000000061",
                        )
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(empty_discover_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(empty_discover_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let results: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(results.as_array().unwrap().len(), 0);

    // Discovery search by text query finds the business profile.
    let discover_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/business-profiles/discover?query=Discoverable")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        access_token(
                            "owner-bp@example.test",
                            "10000000-0000-0000-0000-000000000061",
                            1,
                            "00000000-0000-0000-0000-000000000061",
                        )
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(discover_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(discover_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let results: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let results = results.as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["businessName"], "Discoverable Gym");

    // Owner can delete the business profile.
    let owner_delete_response = app
        .oneshot(
            axum::http::Request::builder()
                .method("DELETE")
                .uri("/workout/api/business-profiles/id/1")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        access_token(
                            "owner-bp@example.test",
                            "10000000-0000-0000-0000-000000000061",
                            1,
                            "00000000-0000-0000-0000-000000000061",
                        )
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        owner_delete_response.status(),
        axum::http::StatusCode::NO_CONTENT
    );
}

/// A non-owner reading a profile never gets its `taxId`, and discovery matches the
/// text literally (a `%` or `_` is not a wildcard that lists every profile).
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn business_profile_reads_hide_tax_id_and_discovery_matches_text_literally() {
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
        ("ACCESS_TOKEN_SECRET", "c007-http-test-secret"),
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
                 (1, '00000000-0000-0000-0000-000000000071', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-000000000072', 'Other', 'Person', '1990-01-01', 'X', now(), now());
               INSERT INTO "user"
                 (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
               VALUES
                 (1, '10000000-0000-0000-0000-000000000071', 'Owner Person', 'owner-71@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000071', now(), now()),
                 (2, '10000000-0000-0000-0000-000000000072', 'Other Person', 'other-72@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000072', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
               VALUES
                 ('20000000-0000-0000-0000-000000000071', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000072', 1, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000073', 2, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000074', 2, 'privacy', '1.0.0', now(), '127.0.0.1');
               INSERT INTO business_profile
                 (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at)
               VALUES
                 (1, '30000000-0000-0000-0000-000000000071', 1, '00000000-0000-0000-0000-000000000071', '999', 'Discoverable Gym', 'Professional', now(), now());
               INSERT INTO team_members
                 (id, uuid, business_profile_id, business_profile_uuid, person_id, person_uuid, status, created_at, updated_at)
               VALUES
                 (1, '40000000-0000-0000-0000-000000000071', 1, '30000000-0000-0000-0000-000000000071', 2, '00000000-0000-0000-0000-000000000072', 'Pending', now(), now());"#,
        )
        .await
        .unwrap();
    let state = AppState {
        conn: Arc::new(database),
    };
    let app = axum::Router::new()
        .nest("/workout/api/business-profiles", business_profile_routes(state.clone()))
        .nest("/workout/api/team-members", team_member_routes(state.clone()))
        .with_state(state);
    let owner = access_token(
        "owner-71@example.test",
        "10000000-0000-0000-0000-000000000071",
        1,
        "00000000-0000-0000-0000-000000000071",
    );
    let other = access_token(
        "other-72@example.test",
        "10000000-0000-0000-0000-000000000072",
        2,
        "00000000-0000-0000-0000-000000000072",
    );
    let get = |uri: &'static str, token: &str| {
        let request = axum::http::Request::builder()
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
            (status, serde_json::from_slice::<serde_json::Value>(&bytes).unwrap_or_default())
        }
    };

    // The owner sees the tax id; another person does not.
    let (status, body) = get("/workout/api/business-profiles/id/1", &owner).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(body["taxId"], "999");
    let (status, body) = get("/workout/api/business-profiles/id/1", &other).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(body["taxId"], "");
    assert_eq!(body["businessName"], "Discoverable Gym");

    // Owner listings and discovery redact it for other people too.
    let (_, body) = get("/workout/api/business-profiles/owner/id/1", &other).await;
    assert_eq!(body[0]["taxId"], "");
    let (_, body) = get("/workout/api/business-profiles/discover?query=Discoverable", &other).await;
    assert_eq!(body[0]["taxId"], "");

    // An invited person sees the inviting business, but not its tax id.
    let (status, body) = get("/workout/api/team-members", &other).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(body["receivedRequests"][0]["businessName"], "Discoverable Gym");
    assert_eq!(body["receivedRequests"][0]["taxId"], "");

    // `%` and `_` are literal characters, not wildcards.
    let (status, body) = get("/workout/api/business-profiles/discover?query=%25", &other).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 0);
    let (_, body) = get("/workout/api/business-profiles/discover?query=_", &other).await;
    assert_eq!(body.as_array().unwrap().len(), 0);
}
