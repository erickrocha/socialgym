use application::{routes::person_routes::person_routes, AppState};
use business::domain::access_token::Claims;
use entity::{person_address_entity, revoked_token_entity};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Database, EntityTrait, Set};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceExt;
use uuid::Uuid;

static TEST_LOCK: Mutex<()> = Mutex::const_new(());
const ACCESS_SECRET: &str = "c001-security-access-secret";

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

async fn seed_people(database: &sea_orm::DatabaseConnection, with_consent: bool) {
    database
        .execute_unprepared(
            r#"INSERT INTO person
                 (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
               VALUES
                 (1, '00000000-0000-0000-0000-000000000071', 'First', 'Owner', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-000000000072', 'Second', 'Owner', '1990-01-01', 'X', now(), now());
               INSERT INTO "user"
                 (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
               VALUES
                 (1, '10000000-0000-0000-0000-000000000071', 'First Owner', 'first@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000071', now(), now()),
                 (2, '10000000-0000-0000-0000-000000000072', 'Second Owner', 'second@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000072', now(), now());"#,
        )
        .await
        .unwrap();

    if with_consent {
        database
            .execute_unprepared(
                r#"INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
                   VALUES
                     ('20000000-0000-0000-0000-000000000071', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
                     ('20000000-0000-0000-0000-000000000072', 1, 'privacy', '1.0.0', now(), '127.0.0.1'),
                     ('20000000-0000-0000-0000-000000000073', 2, 'terms', '1.0.0', now(), '127.0.0.1'),
                     ('20000000-0000-0000-0000-000000000074', 2, 'privacy', '1.0.0', now(), '127.0.0.1');"#,
            )
            .await
            .unwrap();
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
        &EncodingKey::from_secret(ACCESS_SECRET.as_bytes()),
    )
    .unwrap()
}

fn app(database: sea_orm::DatabaseConnection) -> axum::Router {
    let state = AppState {
        conn: Arc::new(database),
    };
    axum::Router::new()
        .nest("/workout/api/people", person_routes(state.clone()))
        .with_state(state)
}

fn authorized_request(
    method: axum::http::Method,
    uri: &str,
    token: &str,
) -> axum::http::Request<axum::body::Body> {
    axum::http::Request::builder()
        .method(method)
        .uri(uri)
        .header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"))
        .body(axum::body::Body::empty())
        .unwrap()
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn tc004_protected_route_rejects_missing_invalid_and_revoked_tokens() {
    let _guard = TEST_LOCK.lock().await;
    let _environment = TestEnvironment::set(&[
        ("ACCESS_TOKEN_SECRET", ACCESS_SECRET),
        ("AUTH_RULES_ENABLED", "false"),
        ("TOKEN_REVOCATION_ENABLED", "true"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = disposable_database().await;
    seed_people(&database, true).await;
    let app = app(database.clone());
    let uri = "/workout/api/people/me";

    let missing_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri(uri)
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        missing_response.status(),
        axum::http::StatusCode::UNAUTHORIZED
    );

    let invalid_token = {
        let claims = Claims::new(
            "first@example.test".to_string(),
            chrono::Utc::now().timestamp() + 3600,
            "10000000-0000-0000-0000-000000000071".to_string(),
            "First Owner".to_string(),
            1,
            "00000000-0000-0000-0000-000000000071".to_string(),
            "default".to_string(),
            None,
            None,
        );
        encode(
            &Header::new(Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(b"wrong-c001-signing-secret"),
        )
        .unwrap()
    };
    let invalid_response = app
        .clone()
        .oneshot(authorized_request(
            axum::http::Method::GET,
            uri,
            &invalid_token,
        ))
        .await
        .unwrap();
    assert_eq!(
        invalid_response.status(),
        axum::http::StatusCode::UNAUTHORIZED
    );

    let revoked_token = access_token(
        "first@example.test",
        "10000000-0000-0000-0000-000000000071",
        1,
        "00000000-0000-0000-0000-000000000071",
    );
    let claims = jsonwebtoken::decode::<Claims>(
        &revoked_token,
        &jsonwebtoken::DecodingKey::from_secret(ACCESS_SECRET.as_bytes()),
        &jsonwebtoken::Validation::new(Algorithm::HS512),
    )
    .unwrap()
    .claims;
    revoked_token_entity::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        jti: Set(claims.jti),
        user_id: Set(1),
        token_type: Set("access".to_string()),
        expires_at: Set(chrono::DateTime::from_timestamp(claims.exp, 0).unwrap()),
        ..Default::default()
    }
    .insert(&database)
    .await
    .unwrap();

    let revoked_response = app
        .oneshot(authorized_request(
            axum::http::Method::GET,
            uri,
            &revoked_token,
        ))
        .await
        .unwrap();
    assert_eq!(
        revoked_response.status(),
        axum::http::StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn tc005_missing_consent_blocks_normal_route_but_keeps_recovery_route_available() {
    let _guard = TEST_LOCK.lock().await;
    let _environment = TestEnvironment::set(&[
        ("ACCESS_TOKEN_SECRET", ACCESS_SECRET),
        ("AUTH_RULES_ENABLED", "false"),
        ("TOKEN_REVOCATION_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = disposable_database().await;
    seed_people(&database, false).await;
    let app = app(database);
    let token = access_token(
        "first@example.test",
        "10000000-0000-0000-0000-000000000071",
        1,
        "00000000-0000-0000-0000-000000000071",
    );

    let protected_response = app
        .clone()
        .oneshot(authorized_request(
            axum::http::Method::GET,
            "/workout/api/people/me",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(
        protected_response.status(),
        axum::http::StatusCode::FORBIDDEN
    );
    let body = axum::body::to_bytes(protected_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error["errorKey"], "CONSENT_REQUIRED");

    let recovery_response = app
        .oneshot(authorized_request(
            axum::http::Method::GET,
            "/workout/api/people/me/consents/pending",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(recovery_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(recovery_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let pending: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let pending_documents: Vec<&str> = pending
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|consent| consent["document"].as_str())
        .collect();
    assert!(pending_documents.contains(&"terms"));
    assert!(pending_documents.contains(&"privacy"));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn tc006_person_cannot_delete_another_owners_address() {
    let _guard = TEST_LOCK.lock().await;
    let _environment = TestEnvironment::set(&[
        ("ACCESS_TOKEN_SECRET", ACCESS_SECRET),
        ("AUTH_RULES_ENABLED", "false"),
        ("TOKEN_REVOCATION_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
    ]);
    let database = disposable_database().await;
    seed_people(&database, true).await;
    database
        .execute_unprepared(
            r#"INSERT INTO person_address
                 (id, uuid, person_id, address_line1, locality, administrative_area, country_code, current, created_at, updated_at)
               VALUES
                 (1, '30000000-0000-0000-0000-000000000071', 1, '1 Owner Street', 'Test City', 'Test Area', 'US', true, now(), now());"#,
        )
        .await
        .unwrap();
    let app = app(database.clone());
    let second_owner_token = access_token(
        "second@example.test",
        "10000000-0000-0000-0000-000000000072",
        2,
        "00000000-0000-0000-0000-000000000072",
    );

    let response = app
        .oneshot(authorized_request(
            axum::http::Method::DELETE,
            "/workout/api/people/me/address/1",
            &second_owner_token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);

    let address = person_address_entity::Entity::find_by_id(1)
        .one(&database)
        .await
        .unwrap()
        .expect("the other owner's address must remain after forbidden deletion");
    assert_eq!(address.person_id, 1);
    assert_eq!(address.address_line1, "1 Owner Street");
}
