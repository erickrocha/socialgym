//! C-010 gap W10 (fixed in task 2): no log line of the service may carry a credential or the email.
//! `UserUseCase::add` used to log the whole user record twice (password redacted, email and person uuid
//! in clear) and the sign-in flow logged the email.
mod support;
use business::domain::user::User;
use business::use_cases::user_use_case::UserUseCase;
use chrono::{DateTime, Utc};
use entity::user_entity::UserEntity;
use log::{Level, LevelFilter, Log, Metadata, Record};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use std::sync::Mutex;
use uuid::Uuid;

struct Capture;

static LINES: Mutex<Vec<String>> = Mutex::new(Vec::new());

impl Log for Capture {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &Record<'_>) {
        // Only this service's own lines: the SQL driver's mock connection traces its fixture rows, which
        // are test data, not something the use case wrote.
        if record.target().starts_with("business") {
            LINES
                .lock()
                .unwrap()
                .push(format!("{} {}", record.level(), record.args()));
        }
    }
    fn flush(&self) {}
}

static CAPTURE: Capture = Capture;

fn stored(email: &str) -> UserEntity {
    UserEntity {
        id: 1,
        name: Some("Log Person".to_string()),
        email: email.to_string(),
        password: "$2b$12$stored-hash-from-the-database".to_string(),
        enabled: true,
        first_login: true,
        person_id: 1,
        person_uuid: Uuid::new_v4(),
        uuid: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        failed_login_attempts: 0,
        locked_until: None::<DateTime<Utc>>,
        token_valid_after: None,
        deletion_requested_at: None,
        deletion_scheduled_at: None,
    }
}

#[tokio::test]
async fn adding_a_user_writes_no_password_hash_or_email_to_the_logs() {
    let _ = log::set_logger(&CAPTURE);
    log::set_max_level(LevelFilter::Trace);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![stored("log.person@example.test")]])
        .into_connection();
    let user = User::new(
        Some("Log Person".to_string()),
        "log.person@example.test".to_string(),
        "Str0ng!Passw0rd-for-logs".to_string(),
        1,
        Uuid::new_v4().to_string(),
    );

    UserUseCase::add(&db, user).await.unwrap();

    let lines = LINES.lock().unwrap();
    let leaked: Vec<&String> = lines
        .iter()
        .filter(|line| {
            line.contains("Str0ng!Passw0rd-for-logs")
                || line.contains("$2b$")
                || line.contains("log.person@example.test")
        })
        .collect();
    assert!(
        leaked.is_empty(),
        "credentials reached the logs: {leaked:?}"
    );
    assert!(
        lines
            .iter()
            .all(|line| !line.starts_with(&Level::Error.to_string()) || !line.contains("password")),
        "no error line names a password either"
    );
}

#[tokio::test]
async fn a_pre_signed_upload_link_is_not_written_to_the_logs() {
    let _ = log::set_logger(&CAPTURE);
    log::set_max_level(LevelFilter::Trace);
    unsafe {
        std::env::set_var("AWS_WORKOUT_BUCKET", "c010-link-bucket");
        std::env::set_var("AWS_REGION", "us-east-1");
        std::env::set_var("AWS_ACCESS_KEY_ID", "test");
        std::env::set_var("AWS_SECRET_ACCESS_KEY", "test");
    }

    let storage =
        business::use_cases::image_storage_use_case::ImageStorageUseCase::generate_presigned_url(
            "person".to_string(),
            7,
            "00000000-0000-0000-0000-0000000000c7",
            "gallery",
            "image/png",
        )
        .await
        .expect("link");
    assert!(
        storage.url.contains("c010-link-bucket"),
        "the link itself is returned to the caller"
    );

    let lines = LINES.lock().unwrap();
    let leaked: Vec<&String> = lines
        .iter()
        .filter(|line| {
            line.contains("c010-link-bucket")
                || line.contains("X-Amz-Signature")
                || line.contains("0000000000c7")
        })
        .collect();
    assert!(
        leaked.is_empty(),
        "the pre-signed link reached the logs: {leaked:?}"
    );
}

/// TC-006 step 4: the sign-up, sign-in, token validation, data export and logout flows, run against a real
/// database, write no email, password, token or link to the logs.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn the_sign_up_sign_in_and_export_flows_write_no_credential_token_or_email_to_the_logs() {
    use business::use_cases::authentication::Authentication;
    use business::use_cases::data_export_use_case::DataExportUseCase;
    use business::use_cases::sign_up_use_case::{SignUpRequest, SignUpUseCase};
    let _ = log::set_logger(&CAPTURE);
    log::set_max_level(LevelFilter::Trace);
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", "c010-log-access-secret");
        std::env::set_var("REFRESH_TOKEN_SECRET", "c010-log-refresh-secret");
        std::env::set_var("TERMS_VERSION", "1.0.0");
        std::env::set_var("PRIVACY_VERSION", "1.0.0");
    }
    LINES.lock().unwrap().clear();
    let db = support::fresh_db().await;
    let email = "flow.person@example.test";
    let password = "Str0ng!Passw0rd-flow";
    let signed_up = SignUpUseCase::execute(
        &db,
        SignUpRequest {
            firstname: "Flow".into(),
            surname: "Person".into(),
            date_of_birth: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            gender: "X".into(),
            email: email.into(),
            password: password.into(),
            terms_accepted: true,
            privacy_accepted: true,
            terms_version: "1.0.0".into(),
            privacy_version: "1.0.0".into(),
            language: "en".into(),
            real_ip: Some("203.0.113.9".into()),
            forwarded_for: None,
        },
        chrono::NaiveDate::from_ymd_opt(2026, 10, 8).unwrap(),
    )
    .await
    .expect("sign-up");
    let signed_in = Authentication::execute(&db, email.to_string(), password.to_string())
        .await
        .expect("sign-in");
    Authentication::validate(&db, signed_in.access_token.clone())
        .await
        .expect("validate");
    DataExportUseCase::create(&db, signed_in.person_id)
        .await
        .expect("export");

    let secrets = [
        email.to_string(),
        password.to_string(),
        signed_up.access_token.clone(),
        signed_in.access_token.clone(),
        signed_in.refresh_token.clone().unwrap(),
        "Bearer".to_string(),
        "X-Amz".to_string(),
        "203.0.113.9".to_string(),
    ];
    let lines = LINES.lock().unwrap();
    assert!(
        !lines.is_empty(),
        "the flows do log; the check is not vacuous"
    );
    let leaked: Vec<&String> = lines
        .iter()
        .filter(|line| secrets.iter().any(|secret| line.contains(secret.as_str())))
        .collect();
    assert!(
        leaked.is_empty(),
        "something private reached the logs: {leaked:?}"
    );
}
