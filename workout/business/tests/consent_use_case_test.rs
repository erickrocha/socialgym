use business::commons::legal_documents;
use business::domain::business_error::BusinessErrorKind;
use business::use_cases::consent_use_case::ConsentUseCase;
use chrono::Utc;
use entity::consent_entity::Model as Consent;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

fn consent(
    person_id: i32,
    document: &str,
    version: &str,
    revoked_at: Option<chrono::DateTime<Utc>>,
) -> Consent {
    Consent {
        id: 1,
        uuid: Uuid::new_v4(),
        person_id,
        document: document.to_string(),
        version: version.to_string(),
        accepted_at: Utc::now(),
        ip: "127.0.0.1".to_string(),
        revoked_at,
    }
}

#[tokio::test]
async fn rejects_unknown_documents_and_outdated_versions_before_database_access() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let terms_version = legal_documents::current_version(legal_documents::TERMS).unwrap();

    let unknown_document = ConsentUseCase::accept(&db, 7, "unknown", "1.0.0", "127.0.0.1")
        .await
        .unwrap_err();
    let outdated_version = ConsentUseCase::accept(
        &db,
        7,
        legal_documents::TERMS,
        &format!("{terms_version}-old"),
        "127.0.0.1",
    )
    .await
    .unwrap_err();
    let unknown_requirement = ConsentUseCase::require_current(&db, 7, "unknown")
        .await
        .unwrap_err();
    let unknown_revoke = ConsentUseCase::revoke(&db, 7, "unknown").await.unwrap_err();

    for error in [
        unknown_document,
        outdated_version,
        unknown_requirement,
        unknown_revoke,
    ] {
        assert_eq!(error.kind, BusinessErrorKind::Validation);
    }
}

#[tokio::test]
async fn rejects_duplicate_current_consent() {
    let terms_version = legal_documents::current_version(legal_documents::TERMS).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![consent(
            7,
            legal_documents::TERMS,
            &terms_version,
            None,
        )]])
        .into_connection();

    let error = ConsentUseCase::accept(&db, 7, legal_documents::TERMS, &terms_version, "127.0.0.1")
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Conflict);
}

#[tokio::test]
async fn pending_lists_outdated_and_revoked_documents_with_only_active_history() {
    let terms_version = legal_documents::current_version(legal_documents::TERMS).unwrap();
    let privacy_version = legal_documents::current_version(legal_documents::PRIVACY).unwrap();
    let health_version = legal_documents::current_version(legal_documents::HEALTH_DATA).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![
            consent(7, legal_documents::TERMS, &terms_version, None),
            consent(
                7,
                legal_documents::PRIVACY,
                &format!("{privacy_version}-old"),
                None,
            ),
            consent(
                7,
                legal_documents::HEALTH_DATA,
                &health_version,
                Some(Utc::now()),
            ),
        ]])
        .into_connection();

    let pending = ConsentUseCase::pending(&db, 7).await.unwrap();

    assert_eq!(pending.len(), 2);
    let privacy = pending
        .iter()
        .find(|item| item.document == legal_documents::PRIVACY)
        .unwrap();
    assert_eq!(
        privacy.accepted_version.as_deref(),
        Some(format!("{privacy_version}-old").as_str())
    );
    let health = pending
        .iter()
        .find(|item| item.document == legal_documents::HEALTH_DATA)
        .unwrap();
    assert_eq!(health.accepted_version, None);
    assert!(pending
        .iter()
        .all(|item| item.document != legal_documents::TERMS));
}

#[tokio::test]
async fn revoke_returns_not_found_when_no_active_consent_exists() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 0,
            rows_affected: 0,
        }])
        .into_connection();

    let error = ConsentUseCase::revoke(&db, 7, legal_documents::TERMS)
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::NotFound);
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn consent_lifecycle_accepts_requires_lists_and_revokes_current_version() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    let database_name = database_url
        .split('?')
        .next()
        .unwrap_or(&database_url)
        .rsplit('/')
        .next()
        .unwrap_or_default();
    assert_eq!(database_name, "workout_test");

    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    db.execute_unprepared(
        r#"INSERT INTO person
             (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
           VALUES
             (1, '00000000-0000-0000-0000-000000000071', 'Consent', 'Owner', '1990-01-01', 'X', now(), now())"#,
    )
    .await
    .unwrap();

    let current_version = legal_documents::current_version(legal_documents::TERMS).unwrap();
    let accepted = ConsentUseCase::accept(
        &db,
        1,
        legal_documents::TERMS,
        &current_version,
        "127.0.0.1",
    )
    .await
    .unwrap();
    assert_eq!(accepted.document, legal_documents::TERMS);
    assert_eq!(accepted.version, current_version);
    assert!(accepted.revoked_at.is_none());

    assert!(
        ConsentUseCase::require_current(&db, 1, legal_documents::TERMS)
            .await
            .is_ok()
    );
    assert_eq!(
        ConsentUseCase::accept(
            &db,
            1,
            legal_documents::TERMS,
            &current_version,
            "127.0.0.1",
        )
        .await
        .unwrap_err()
        .kind,
        BusinessErrorKind::Conflict
    );

    let pending = ConsentUseCase::pending(&db, 1).await.unwrap();
    assert!(pending
        .iter()
        .all(|item| item.document != legal_documents::TERMS));

    ConsentUseCase::revoke(&db, 1, legal_documents::TERMS)
        .await
        .unwrap();
    assert_eq!(
        ConsentUseCase::require_current(&db, 1, legal_documents::TERMS)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
    assert_eq!(
        ConsentUseCase::revoke(&db, 1, legal_documents::TERMS)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::NotFound
    );
}
