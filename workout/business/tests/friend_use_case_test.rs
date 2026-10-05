use business::domain::business_error::BusinessErrorKind;
use business::domain::enums::InviteStatus;
use business::domain::friend::Friend;
use business::domain::friendship_outbox_event::FriendshipOutboxEvent;
use business::use_cases::friend_use_case::FriendUseCase;
use chrono::Utc;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

fn friendship_model(
    person_id: i32,
    friend_id: i32,
    status: &str,
) -> entity::friends_entity::FriendsEntity {
    entity::friends_entity::FriendsEntity {
        id: 1,
        person_id,
        friend_id,
        status: status.to_string(),
        uuid: Uuid::new_v4(),
        person_uuid: Uuid::new_v4(),
        friend_uuid: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn outbox_model() -> entity::friendship_notification_outbox_entity::Model {
    entity::friendship_notification_outbox_entity::Model {
        id: 1,
        event_uuid: Uuid::new_v4(),
        friendship_uuid: Uuid::new_v4(),
        event_type: "friend_request_created".to_string(),
        actor_person_uuid: Uuid::new_v4(),
        recipient_person_uuid: Uuid::new_v4(),
        occurred_at: Utc::now(),
        attempt_count: 0,
        next_attempt_at: Utc::now(),
        last_error: None,
        published_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn person_model(id: i32, first_name: &str) -> entity::person_entity::PersonEntity {
    entity::person_entity::PersonEntity {
        id,
        uuid: Uuid::new_v4(),
        first_name: first_name.to_string(),
        surname: "Person".to_string(),
        date_of_birth: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        gender: "X".to_string(),
        avatar: None,
        cover_image: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[tokio::test]
async fn rejects_friend_request_to_self_before_database_access() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let error = FriendUseCase::send_friend_request(&db, 7, 7)
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Validation);
}

#[test]
fn request_created_event_targets_the_receiver() {
    let mut request = Friend::new(
        1,
        2,
        "10000000-0000-0000-0000-000000000001".to_string(),
        "10000000-0000-0000-0000-000000000002".to_string(),
        InviteStatus::Pending,
    );
    request.uuid = Some("20000000-0000-0000-0000-000000000001".to_string());

    let event = FriendshipOutboxEvent::request_created(&request).unwrap();

    assert_eq!(event.event_type, "friend_request_created");
    assert_eq!(event.friendship_uuid, request.uuid.unwrap());
    assert_eq!(event.actor_person_uuid, request.person_uuid);
    assert_eq!(event.recipient_person_uuid, request.friend_uuid);
}

#[test]
fn request_accepted_event_targets_the_original_requester() {
    let mut request = Friend::new(
        1,
        2,
        "10000000-0000-0000-0000-000000000001".to_string(),
        "10000000-0000-0000-0000-000000000002".to_string(),
        InviteStatus::Accepted,
    );
    request.uuid = Some("20000000-0000-0000-0000-000000000001".to_string());

    let event = FriendshipOutboxEvent::request_accepted(&request).unwrap();

    assert_eq!(event.event_type, "friend_request_accepted");
    assert_eq!(event.friendship_uuid, request.uuid.unwrap());
    assert_eq!(event.actor_person_uuid, request.friend_uuid);
    assert_eq!(event.recipient_person_uuid, request.person_uuid);
}

#[tokio::test]
async fn creates_a_pending_request_and_outbox_event() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<entity::friends_entity::FriendsEntity>::new()])
        .append_query_results(vec![vec![person_model(1, "John")]])
        .append_query_results(vec![vec![person_model(2, "Jane")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .append_query_results(vec![vec![outbox_model()]])
        .into_connection();

    let request = FriendUseCase::send_friend_request(&db, 1, 2).await.unwrap();

    assert_eq!(request.person_id, 1);
    assert_eq!(request.friend_id, 2);
    assert_eq!(request.status, InviteStatus::Pending);
}

#[tokio::test]
async fn accepts_a_pending_request_and_emits_acceptance_event() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![friendship_model(1, 2, "Accepted")]])
        .append_query_results(vec![vec![outbox_model()]])
        .into_connection();

    let accepted = FriendUseCase::accept_friend_request(&db, 2, 1)
        .await
        .unwrap();

    assert_eq!(accepted.status, InviteStatus::Accepted);
}

#[tokio::test]
async fn only_the_receiver_may_accept_or_reject_and_only_the_sender_may_cancel() {
    let sender_accept_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .into_connection();
    let receiver_cancel_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .into_connection();
    let sender_reject_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .into_connection();

    assert_eq!(
        FriendUseCase::accept_friend_request(&sender_accept_db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
    assert_eq!(
        FriendUseCase::cancel_friend_request(&receiver_cancel_db, 2, 1)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
    assert_eq!(
        FriendUseCase::deny_friend_request(&sender_reject_db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
}

#[tokio::test]
async fn terminal_friendship_states_conflict_with_new_transitions() {
    for status in ["Accepted", "Pending"] {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![friendship_model(1, 2, status)]])
            .into_connection();
        assert_eq!(
            FriendUseCase::send_friend_request(&db, 1, 2)
                .await
                .unwrap_err()
                .kind,
            BusinessErrorKind::Conflict
        );
    }

    let accepted_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Accepted")]])
        .into_connection();
    assert_eq!(
        FriendUseCase::cancel_friend_request(&accepted_db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Conflict
    );
}

#[tokio::test]
async fn inverse_pending_requests_auto_accept() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(2, 1, "Pending")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![friendship_model(2, 1, "Accepted")]])
        .append_query_results(vec![vec![outbox_model()]])
        .into_connection();

    let accepted = FriendUseCase::send_friend_request(&db, 1, 2).await.unwrap();

    assert_eq!(accepted.status, InviteStatus::Accepted);
}

#[tokio::test]
async fn reopens_rejected_or_cancelled_requests_in_the_new_direction() {
    for terminal_status in ["Rejected", "Cancelled"] {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![friendship_model(2, 1, terminal_status)]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
            .append_query_results(vec![vec![outbox_model()]])
            .into_connection();

        let reopened = FriendUseCase::send_friend_request(&db, 1, 2).await.unwrap();
        assert_eq!(reopened.status, InviteStatus::Pending);
        assert_eq!(reopened.person_id, 1);
        assert_eq!(reopened.friend_id, 2);
    }
}

#[tokio::test]
async fn sender_can_cancel_a_pending_request() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![friendship_model(1, 2, "Cancelled")]])
        .into_connection();

    let cancelled = FriendUseCase::cancel_friend_request(&db, 1, 2)
        .await
        .unwrap();

    assert_eq!(cancelled.status, InviteStatus::Cancelled);
}

#[tokio::test]
async fn deny_marks_pending_request_as_rejected() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![friendship_model(1, 2, "Rejected")]])
        .into_connection();

    let rejected = FriendUseCase::deny_friend_request(&db, 2, 1).await.unwrap();

    assert_eq!(rejected.status, InviteStatus::Rejected);
}

#[tokio::test]
async fn requires_accepted_relationship_to_access_friend_profile() {
    let pending_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Pending")]])
        .into_connection();
    let accepted_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship_model(1, 2, "Accepted")]])
        .into_connection();
    let missing_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<entity::friends_entity::FriendsEntity>::new()])
        .into_connection();

    assert_eq!(
        FriendUseCase::ensure_accepted_friend(&pending_db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
    assert!(FriendUseCase::ensure_accepted_friend(&accepted_db, 1, 2)
        .await
        .is_ok());
    assert_eq!(
        FriendUseCase::ensure_accepted_friend(&missing_db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
}

#[tokio::test]
async fn removes_an_accepted_friendship() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 0,
            rows_affected: 1,
        }])
        .into_connection();

    assert!(FriendUseCase::remove_friend(&db, 1, 2).await.is_ok());
}

#[tokio::test]
async fn request_creation_returns_not_found_when_sender_is_missing() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<entity::friends_entity::FriendsEntity>::new()])
        .append_query_results(vec![Vec::<entity::person_entity::PersonEntity>::new()])
        .into_connection();

    assert_eq!(
        FriendUseCase::send_friend_request(&db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::NotFound
    );
}

#[tokio::test]
async fn accept_and_cancel_return_not_found_when_request_is_missing() {
    let accept_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<entity::friends_entity::FriendsEntity>::new()])
        .into_connection();
    let cancel_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<entity::friends_entity::FriendsEntity>::new()])
        .into_connection();

    assert_eq!(
        FriendUseCase::accept_friend_request(&accept_db, 2, 1)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::NotFound
    );
    assert_eq!(
        FriendUseCase::cancel_friend_request(&cancel_db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::NotFound
    );
}

#[tokio::test]
async fn removing_missing_friendship_returns_not_found() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 0,
            rows_affected: 0,
        }])
        .into_connection();

    assert_eq!(
        FriendUseCase::remove_friend(&db, 1, 2)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::NotFound
    );
}

#[tokio::test]
async fn friend_profile_rejects_self_link_before_database_access() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    assert_eq!(
        FriendUseCase::ensure_accepted_friend(&db, 1, 1)
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::Validation
    );
}

#[test]
fn friendship_outbox_event_requires_persisted_uuid() {
    let request = Friend::new(
        1,
        2,
        "10000000-0000-0000-0000-000000000001".to_string(),
        "10000000-0000-0000-0000-000000000002".to_string(),
        InviteStatus::Pending,
    );

    assert_eq!(
        FriendshipOutboxEvent::request_created(&request)
            .unwrap_err()
            .kind,
        BusinessErrorKind::Infrastructure
    );
}
