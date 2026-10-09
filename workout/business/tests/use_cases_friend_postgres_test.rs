mod support;

use business::domain::business_error::BusinessErrorKind as K;
use business::domain::enums::InviteStatus;
use business::use_cases::friend_use_case::FriendUseCase;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use support::{fresh_db, kind, register};

async fn outbox_events(db: &sea_orm::DatabaseConnection) -> i64 {
    let row = db
        .query_one_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT count(*)::bigint AS n FROM friendship_notification_outbox",
        ))
        .await
        .unwrap()
        .unwrap();
    row.try_get::<i64>("", "n").unwrap()
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn friendship_lifecycle_authorizes_each_transition_and_records_outbox_events() {
    let db = fresh_db().await;
    let (a, b, c) = (
        register(&db, 1).await,
        register(&db, 2).await,
        register(&db, 3).await,
    );
    let (ia, ib, ic) = (a.person_id, b.person_id, c.person_id);

    // send: not to yourself, not to unknown people
    assert!(matches!(
        kind(
            &FriendUseCase::send_friend_request(&db, ia, ia)
                .await
                .unwrap_err()
        ),
        K::Validation
    ));
    assert!(matches!(
        kind(
            &FriendUseCase::send_friend_request(&db, 9999, ib)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    assert!(matches!(
        kind(
            &FriendUseCase::send_friend_request(&db, ia, 9999)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));

    // request -> one outbox event; a second pending request from the same sender conflicts
    let request = FriendUseCase::send_friend_request(&db, ia, ib)
        .await
        .unwrap();
    assert_eq!(request.status, InviteStatus::Pending);
    assert_eq!(outbox_events(&db).await, 1);
    assert!(matches!(
        kind(
            &FriendUseCase::send_friend_request(&db, ia, ib)
                .await
                .unwrap_err()
        ),
        K::Conflict
    ));
    assert_eq!(
        FriendUseCase::find_pending_friendship_links(&db, ib, true)
            .await
            .unwrap()
            .iter()
            .map(|l| l.0)
            .collect::<Vec<_>>(),
        vec![ia]
    );
    assert_eq!(
        FriendUseCase::find_pending_friendship_links(&db, ia, false)
            .await
            .unwrap()
            .iter()
            .map(|l| l.0)
            .collect::<Vec<_>>(),
        vec![ib]
    );
    assert!(FriendUseCase::find_pending_friendship_links(&db, ia, true)
        .await
        .unwrap()
        .is_empty());

    // only the receiver answers, only the sender cancels; unknown requests are not found
    assert!(
        matches!(
            kind(
                &FriendUseCase::accept_friend_request(&db, ia, ib)
                    .await
                    .unwrap_err()
            ),
            K::Forbidden
        ),
        "the sender cannot accept"
    );
    assert!(matches!(
        kind(
            &FriendUseCase::deny_friend_request(&db, ia, ib)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(
        matches!(
            kind(
                &FriendUseCase::cancel_friend_request(&db, ib, ia)
                    .await
                    .unwrap_err()
            ),
            K::Forbidden
        ),
        "the receiver cannot cancel"
    );
    assert!(matches!(
        kind(
            &FriendUseCase::accept_friend_request(&db, ib, ic)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    assert!(
        matches!(
            kind(
                &FriendUseCase::ensure_accepted_friend(&db, ia, ib)
                    .await
                    .unwrap_err()
            ),
            K::Forbidden
        ),
        "pending is not a friendship"
    );
    assert!(
        matches!(
            kind(
                &FriendUseCase::ensure_accepted_friend(&db, ia, ic)
                    .await
                    .unwrap_err()
            ),
            K::Forbidden
        ),
        "no relationship"
    );
    assert!(matches!(
        kind(
            &FriendUseCase::ensure_accepted_friend(&db, ia, ia)
                .await
                .unwrap_err()
        ),
        K::Validation
    ));

    // deny, then the sender may ask again (the same row is re-opened)
    assert_eq!(
        FriendUseCase::deny_friend_request(&db, ib, ia)
            .await
            .unwrap()
            .status,
        InviteStatus::Rejected
    );
    assert!(
        matches!(
            kind(
                &FriendUseCase::accept_friend_request(&db, ib, ia)
                    .await
                    .unwrap_err()
            ),
            K::Conflict
        ),
        "closed requests cannot be answered again"
    );
    assert_eq!(
        FriendUseCase::send_friend_request(&db, ia, ib)
            .await
            .unwrap()
            .status,
        InviteStatus::Pending
    );
    assert_eq!(outbox_events(&db).await, 2);

    // cancel, then the other side asks: the row flips direction
    assert_eq!(
        FriendUseCase::cancel_friend_request(&db, ia, ib)
            .await
            .unwrap()
            .status,
        InviteStatus::Cancelled
    );
    let flipped = FriendUseCase::send_friend_request(&db, ib, ia)
        .await
        .unwrap();
    assert_eq!(
        (flipped.person_id, flipped.friend_id, flipped.status),
        (ib, ia, InviteStatus::Pending)
    );

    // crossing requests auto-accept
    let accepted = FriendUseCase::send_friend_request(&db, ia, ib)
        .await
        .unwrap();
    assert_eq!(accepted.status, InviteStatus::Accepted);
    assert!(
        matches!(
            kind(
                &FriendUseCase::send_friend_request(&db, ib, ia)
                    .await
                    .unwrap_err()
            ),
            K::Conflict
        ),
        "already friends"
    );
    FriendUseCase::ensure_accepted_friend(&db, ia, ib)
        .await
        .unwrap();
    FriendUseCase::ensure_accepted_friend(&db, ib, ia)
        .await
        .unwrap();

    // listing is always from the caller's point of view
    let mine = FriendUseCase::find_all_friend(&db, ia).await.unwrap();
    assert_eq!(
        (mine.len(), mine[0].person_id, mine[0].friend_id),
        (1, ia, ib)
    );
    let theirs = FriendUseCase::find_all_friend(&db, ib).await.unwrap();
    assert_eq!((theirs[0].person_id, theirs[0].friend_id), (ib, ia));
    assert!(FriendUseCase::find_all_friend(&db, ic)
        .await
        .unwrap()
        .is_empty());

    // remove: unknown is not found; removal is symmetric
    assert!(matches!(
        kind(&FriendUseCase::remove_friend(&db, ia, ic).await.unwrap_err()),
        K::NotFound
    ));
    FriendUseCase::remove_friend(&db, ib, ia).await.unwrap();
    assert!(FriendUseCase::find_all_friend(&db, ia)
        .await
        .unwrap()
        .is_empty());
    assert!(matches!(
        kind(&FriendUseCase::remove_friend(&db, ia, ib).await.unwrap_err()),
        K::NotFound
    ));
}
