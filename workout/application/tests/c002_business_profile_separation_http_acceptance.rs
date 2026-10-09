use application::{
    routes::{
        business_profile_routes::business_profile_routes, person_routes::person_routes,
        team_member_routes::team_member_routes,
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

/// Issues a JWT. `active_business_profile` is `Some((id, uuid))` when the
/// caller should be acting with a business profile active — the same claim
/// the mobile/web client sets after "switching" into a business profile.
fn access_token(
    email: &str,
    user_uuid: &str,
    person_id: i32,
    person_uuid: &str,
    active_business_profile: Option<(i32, &str)>,
) -> String {
    let (active_id, active_uuid) = match active_business_profile {
        Some((id, uuid)) => (Some(id), Some(uuid.to_string())),
        None => (None, None),
    };
    let claims = Claims::new(
        email.to_string(),
        chrono::Utc::now().timestamp() + 3600,
        user_uuid.to_string(),
        format!("Person {person_id}"),
        person_id,
        person_uuid.to_string(),
        "default".to_string(),
        active_id,
        active_uuid,
    );
    encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(b"c002-http-test-secret"),
    )
    .unwrap()
}

/// Verifies SYS-C002-004 / TC-005: a person-profile is never reachable
/// through a business-profile context. An accepted team membership grants a
/// business profile the narrow right to assign it a workout — never
/// authority over the team member's own profile, health data, or roster
/// listing.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn business_profile_context_never_grants_person_profile_authority() {
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
        ("ACCESS_TOKEN_SECRET", "c002-http-test-secret"),
        ("AUTH_RULES_ENABLED", "false"),
        ("TERMS_VERSION", "1.0.0"),
        ("PRIVACY_VERSION", "1.0.0"),
        ("HEALTH_DATA_CONSENT_VERSION", "1.0.0"),
    ]);
    let database = Database::connect(&database_url).await.unwrap();
    Migrator::refresh(&database).await.unwrap();

    // Person 1 owns a business profile. Person 2 is an Accepted team member
    // of that business profile and has granted health-data consent for
    // their own profile (so the only thing standing between profile 1 and
    // profile 2's health data is ownership, not consent).
    database
        .execute_unprepared(
            r#"INSERT INTO person
                 (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
               VALUES
                 (1, '00000000-0000-0000-0000-000000000071', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
                 (2, '00000000-0000-0000-0000-000000000072', 'Team', 'Member', '1990-01-01', 'X', now(), now());
               INSERT INTO "user"
                 (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
               VALUES
                 (1, '10000000-0000-0000-0000-000000000071', 'Owner Person', 'owner-c002@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000071', now(), now()),
                 (2, '10000000-0000-0000-0000-000000000072', 'Team Member', 'member-c002@example.test', 'unused', false, true, 2, '00000000-0000-0000-0000-000000000072', now(), now());
               INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
               VALUES
                 ('20000000-0000-0000-0000-000000000071', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000072', 1, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000073', 2, 'terms', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000074', 2, 'privacy', '1.0.0', now(), '127.0.0.1'),
                 ('20000000-0000-0000-0000-000000000075', 2, 'health_data', '1.0.0', now(), '127.0.0.1');
               INSERT INTO person_info (id, uuid, person_id, weight, height, created_at, updated_at)
               VALUES
                 (1, '50000000-0000-0000-0000-000000000072', 2, 70.5, 175.0, now(), now());
               INSERT INTO business_profile
                 (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at)
               VALUES
                 (1, '30000000-0000-0000-0000-000000000071', 1, '00000000-0000-0000-0000-000000000071', '888', 'Owner Gym', 'Professional', now(), now());
               INSERT INTO team_members
                 (id, uuid, business_profile_id, business_profile_uuid, person_id, person_uuid, status, created_at, updated_at)
               VALUES
                 (1, '40000000-0000-0000-0000-000000000071', 1, '30000000-0000-0000-0000-000000000071', 2, '00000000-0000-0000-0000-000000000072', 'Accepted', now(), now());"#,
        )
        .await
        .unwrap();

    let state = AppState {
        conn: Arc::new(database),
    };
    let app = axum::Router::new()
        .nest("/workout/api/people", person_routes(state.clone()))
        .nest(
            "/workout/api/business-profiles",
            business_profile_routes(state.clone()),
        )
        .nest(
            "/workout/api/team-members",
            team_member_routes(state.clone()),
        )
        .with_state(state);

    // Person 1 acting with the business profile active — the same claim
    // shape the client sends after switching into the business context.
    let owner_in_business_context = access_token(
        "owner-c002@example.test",
        "10000000-0000-0000-0000-000000000071",
        1,
        "00000000-0000-0000-0000-000000000071",
        Some((1, "30000000-0000-0000-0000-000000000071")),
    );

    // 1. The business profile's own team roster lists the accepted member,
    //    but never exposes their health data — the roster is a membership
    //    read model, not a person-profile projection.
    let roster_response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/team-members")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!("Bearer {owner_in_business_context}"),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(roster_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(roster_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let roster: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let members = roster["members"].as_array().unwrap();
    assert_eq!(members.len(), 1);
    assert_eq!(members[0]["uuid"], "00000000-0000-0000-0000-000000000072");
    assert!(
        members[0]["personInfo"].is_null(),
        "team roster leaked health/profile data: {:?}",
        members[0]["personInfo"]
    );

    // 2. Reading the team member's person profile by uuid through the
    //    business-profile-context token is still denied — only the person
    //    themselves may read `/people/me/{uuid}`.
    let get_profile_by_uuid = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/people/me/00000000-0000-0000-0000-000000000072")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!("Bearer {owner_in_business_context}"),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        get_profile_by_uuid.status(),
        axum::http::StatusCode::FORBIDDEN
    );

    // 3. Reading the team member's person profile by internal id through the
    //    same business-profile-context token is denied too.
    let get_profile_by_id = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/people/id/2")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!("Bearer {owner_in_business_context}"),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        get_profile_by_id.status(),
        axum::http::StatusCode::FORBIDDEN
    );

    // 4. `/people/me` always resolves to the caller's own person id — even
    //    while a business profile is active — so it returns profile 1, not
    //    the team member's profile 2.
    let get_me_response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/workout/api/people/me")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!("Bearer {owner_in_business_context}"),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_me_response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(get_me_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let me: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(me["uuid"], "00000000-0000-0000-0000-000000000071");
}
