mod support;

use business::domain::business_error::BusinessErrorKind as K;
use business::domain::enums::{Position, WeightUnit};
use business::domain::settings::Settings;
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::account_deletion_use_case::AccountDeletionUseCase;
use business::use_cases::account_purge_use_case::AccountPurgeUseCase;
use business::use_cases::friend_use_case::FriendUseCase;
use business::use_cases::setings_use_case::SettingsUseCase;
use business::use_cases::team_member_use_case::TeamMemberUseCase;
use business::use_cases::workout_use_case::WorkoutUseCase;
use entity::{business_profile_entity, person_entity, user_entity, workout_entity};
use sea_orm::{EntityTrait, PaginatorTrait};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use support::{business_profile, exercise, fresh_db, kind, register, workout};

/// Stand-in for timeline's internal gRPC service: succeeds while `status` is below 300.
mod timeline_proto {
    tonic::include_proto!("grpc.timeline");
}
use timeline_proto::internal_service_server::{InternalService, InternalServiceServer};
use timeline_proto::*;

struct TimelineStub(Arc<AtomicU16>);

#[tonic::async_trait]
impl InternalService for TimelineStub {
    async fn delete_person_data(
        &self,
        request: tonic::Request<PersonDataRequest>,
    ) -> Result<tonic::Response<DeletePersonDataResponse>, tonic::Status> {
        // The caller must present the shared secret.
        assert_eq!(request.metadata().get("x-internal-secret").and_then(|v| v.to_str().ok()), Some("test-secret"));
        // Mirrors the old HTTP stub's convention: a status below 300 is success.
        match self.0.load(Ordering::SeqCst) {
            status if status < 300 => Ok(tonic::Response::new(DeletePersonDataResponse {})),
            _ => Err(tonic::Status::unavailable("timeline down")),
        }
    }
    async fn export_person_data(
        &self,
        _: tonic::Request<PersonDataRequest>,
    ) -> Result<tonic::Response<ExportPersonDataResponse>, tonic::Status> {
        Ok(tonic::Response::new(ExportPersonDataResponse { export_json: "{}".into() }))
    }
}

async fn timeline_stub(status: Arc<AtomicU16>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    unsafe {
        std::env::set_var("TIMELINE_GRPC_URL", format!("http://127.0.0.1:{port}"));
        std::env::set_var("INTERNAL_SERVICE_SECRET", "test-secret");
    }
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(InternalServiceServer::new(TimelineStub(status)))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn settings_belong_to_their_person_and_resolve_by_every_key() {
    let db = fresh_db().await;
    let (a, b) = (register(&db, 1).await, register(&db, 2).await);
    let uc = SettingsUseCase::new(SettingsGateway::new(db.clone()));

    // registration bootstrapped the settings
    let mine = uc.get_by_owner_id(a.person_id).await.unwrap();
    assert!(mine.notifications_enabled);
    assert_eq!(uc.get_by_id(mine.id.unwrap()).await.unwrap().person_id, a.person_id);
    assert_eq!(uc.get_by_uuid(mine.uuid.clone().unwrap()).await.unwrap().id, mine.id);
    assert_eq!(uc.get_by_owner_uuid(a.person_uuid.clone()).await.unwrap().id, mine.id);
    assert!(uc.get_by_id(9999).await.is_err());
    assert!(uc.get_by_owner_id(9999).await.is_err());
    assert!(uc.get_by_uuid("00000000-0000-0000-0000-00000000dead".into()).await.is_err());
    assert!(matches!(kind(&uc.get_by_owner_uuid("00000000-0000-0000-0000-00000000dead".into()).await.unwrap_err()), K::NotFound));

    // update: only the owner; the owner columns come from the actor, not the payload
    let mut edit = mine.clone();
    edit.theme = "dark".into();
    edit.notifications_enabled = false;
    assert!(matches!(kind(&uc.persist(edit.clone(), &b).await.unwrap_err()), K::Forbidden));
    let saved = uc.persist(edit, &a).await.unwrap();
    assert_eq!((saved.theme.as_str(), saved.notifications_enabled, saved.person_id), ("dark", false, a.person_id));
    assert!(uc.get_by_owner_id(b.person_id).await.unwrap().notifications_enabled, "another person's settings are untouched");

    // creating without an id belongs to the actor even when the payload names someone else
    let fresh = Settings::new(b.person_id, b.person_uuid.clone(), "en".into(), "light".into(), true, Position::Left, "home".into(), Some(WeightUnit::Kilograms));
    assert!(matches!(kind(&uc.persist(fresh, &a).await.unwrap_err()), K::Conflict), "a person has exactly one settings row");
    // the bootstrap path used at registration is rejected by the database once the person has settings
    let again = Settings::new(b.person_id, b.person_uuid.clone(), "pt".into(), "dark".into(), true, Position::Top, "home".into(), None);
    assert!(uc.bootstrap_for_person(again).await.is_err());
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn account_deletion_is_scheduled_cancelled_and_purged_with_every_dependent_row() {
    let db = fresh_db().await;
    let status = Arc::new(AtomicU16::new(204));
    timeline_stub(status.clone()).await;
    let (leaving, friend, staying) = (register(&db, 1).await, register(&db, 2).await, register(&db, 3).await);

    // data that must disappear with the account
    let profile = business_profile(&db, &leaving, "Gym").await;
    TeamMemberUseCase::send_team_member_request(&db, profile.id.unwrap(), friend.person_id).await.unwrap();
    FriendUseCase::send_friend_request(&db, leaving.person_id, friend.person_id).await.unwrap();
    FriendUseCase::accept_friend_request(&db, friend.person_id, leaving.person_id).await.unwrap();
    WorkoutUseCase::persist(&db, workout("Mine", business::domain::exercise::Visibility::Private, vec![exercise("Squat", business::domain::exercise::Visibility::Private)]), &leaving, None, None).await.unwrap();
    WorkoutUseCase::persist(&db, workout("Kept", business::domain::exercise::Visibility::Private, vec![]), &staying, None, None).await.unwrap();

    // cancel needs a pending deletion and an existing user
    assert!(matches!(kind(&AccountDeletionUseCase::cancel_deletion(&db, 9999).await.unwrap_err()), K::NotFound));
    assert!(matches!(kind(&AccountDeletionUseCase::cancel_deletion(&db, leaving.id.unwrap()).await.unwrap_err()), K::Validation));

    // scheduled with a grace period: the account is disabled but not yet due
    let scheduled = AccountDeletionUseCase::request_deletion(&db, leaving.id.unwrap(), false).await.unwrap();
    assert!(scheduled.scheduled_at > scheduled.requested_at);
    assert!(!user_entity::Entity::find_by_id(leaving.id.unwrap()).one(&db).await.unwrap().unwrap().enabled);
    assert_eq!(AccountPurgeUseCase::purge_due_accounts(&db).await.unwrap(), 0, "not due yet");

    // cancel restores the account
    AccountDeletionUseCase::cancel_deletion(&db, leaving.id.unwrap()).await.unwrap();
    assert!(user_entity::Entity::find_by_id(leaving.id.unwrap()).one(&db).await.unwrap().unwrap().deletion_scheduled_at.is_none());

    // immediate deletion is due now; a failing timeline keeps every row and reports nothing purged
    let now = AccountDeletionUseCase::request_deletion(&db, leaving.id.unwrap(), true).await.unwrap();
    assert_eq!(now.scheduled_at, now.requested_at);
    status.store(500, Ordering::SeqCst);
    assert_eq!(AccountPurgeUseCase::purge_due_accounts(&db).await.unwrap(), 0);
    assert!(person_entity::Entity::find_by_id(leaving.person_id).one(&db).await.unwrap().is_some());

    // timeline recovered: the cascade removes the person and everything depending on them
    status.store(204, Ordering::SeqCst);
    assert_eq!(AccountPurgeUseCase::purge_due_accounts(&db).await.unwrap(), 1);
    assert!(person_entity::Entity::find_by_id(leaving.person_id).one(&db).await.unwrap().is_none());
    assert!(user_entity::Entity::find_by_id(leaving.id.unwrap()).one(&db).await.unwrap().is_none());
    assert_eq!(business_profile_entity::Entity::find().count(&db).await.unwrap(), 0);
    assert_eq!(workout_entity::Entity::find().count(&db).await.unwrap(), 1, "only the other person's workout is left");
    assert!(person_entity::Entity::find_by_id(friend.person_id).one(&db).await.unwrap().is_some());
    assert!(person_entity::Entity::find_by_id(staying.person_id).one(&db).await.unwrap().is_some());
    assert_eq!(AccountPurgeUseCase::purge_due_accounts(&db).await.unwrap(), 0, "idempotent");
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn settings_migration_collapses_existing_duplicates_and_enforces_uniqueness() {
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    let db = fresh_db().await;
    let a = register(&db, 1).await;
    // go back two steps (drops the unique index), create duplicates, then migrate forward again
    Migrator::down(&db, Some(2)).await.unwrap();
    let dup = |lang: &str, updated: &str| format!(
        "INSERT INTO settings (uuid, person_id, person_uuid, language, theme, notifications_enabled, context_menu_position, home_page, created_at, updated_at) \
         VALUES (gen_random_uuid(), {}, '{}', '{lang}', 'light', true, 'Left', 'home', now(), '{updated}')", a.person_id, a.person_uuid);
    db.execute_unprepared(&dup("old", "2020-01-01")).await.unwrap();
    db.execute_unprepared(&dup("newest", "2030-01-01")).await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    let rows = db.query_all_raw(Statement::from_string(DbBackend::Postgres, format!("SELECT language FROM settings WHERE person_id = {}", a.person_id))).await.unwrap();
    assert_eq!(rows.len(), 1, "one row per person remains");
    assert_eq!(rows[0].try_get::<String>("", "language").unwrap(), "newest", "the most recently updated row wins");
    assert!(db.execute_unprepared(&dup("again", "2031-01-01")).await.is_err(), "the database now rejects a second row");
}
