//! C-010 gap W10 (fixed in task 2): no log line of the service may carry a credential or the email.
//! `UserUseCase::add` used to log the whole user record twice (password redacted, email and person uuid
//! in clear) and the sign-in flow logged the email.
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
            LINES.lock().unwrap().push(format!("{} {}", record.level(), record.args()));
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
    log::set_logger(&CAPTURE).unwrap();
    log::set_max_level(LevelFilter::Trace);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult { last_insert_id: 1, rows_affected: 1 }])
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
        .filter(|line| line.contains("Str0ng!Passw0rd-for-logs") || line.contains("$2b$") || line.contains("log.person@example.test"))
        .collect();
    assert!(leaked.is_empty(), "credentials reached the logs: {leaked:?}");
    assert!(lines.iter().all(|line| !line.starts_with(&Level::Error.to_string()) || !line.contains("password")), "no error line names a password either");
}
