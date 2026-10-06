//! C-006 error-mapping coverage: every gateway call must turn a database outage into a
//! `BusinessError` (or an empty result where the contract says so) instead of panicking.
//! Needs no MongoDB: the client points at a closed port and gives up quickly.
use business::gateway::evolution_check_in_gateway::{EvolutionCheckInGateway, EvolutionCheckInGatewayPort};
use business::gateway::mention_notification_gateway::MentionNotificationGateway;
use business::gateway::push_device_gateway::PushDeviceGateway;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use domain::business_error::BusinessErrorKind;
use domain::enums::Visibility;
use domain::evolution_check_in::EvolutionCheckIn;
use domain::in_app_notification::InAppNotification;
use domain::mention_notification_event::MentionNotificationEvent;
use mongodb::bson::DateTime;
use mongodb::{Client, Database};

async fn unreachable_database() -> Database {
    Client::with_uri_str(
        "mongodb://127.0.0.1:1/?serverSelectionTimeoutMS=150&connectTimeoutMS=150",
    )
    .await
    .unwrap()
    .database("timeline_unreachable")
}

fn event() -> MentionNotificationEvent {
    MentionNotificationEvent::new(
        "e1".into(), "post".into(), "p1".into(), Some("p1".into()), None, 1,
        "author".into(), "Author".into(), "mentioned".into(), "hi".into(),
    )
}

fn notification() -> InAppNotification {
    InAppNotification::from_mention_event(
        "n1".into(), "recipient".into(), "actor".into(), "Actor".into(),
        Some("p1".into()), None, "post".into(), "p1".into(), "hi".into(),
    )
}

#[tokio::test]
async fn push_device_gateway_reports_database_outage_as_infrastructure_error() {
    let db = unreachable_database().await;
    let kind = |r: Result<_, domain::business_error::BusinessError>| -> BusinessErrorKind {
        let r: Result<(), _> = r.map(|_: ()| ());
        r.unwrap_err().kind
    };

    assert_eq!(kind(PushDeviceGateway::register(&db, "d", "o", "android", "t").await), BusinessErrorKind::Infrastructure);
    assert_eq!(kind(PushDeviceGateway::remove_owned(&db, "d", "o").await.map(|_| ())), BusinessErrorKind::Infrastructure);
    assert_eq!(kind(PushDeviceGateway::find_all_for_person(&db, "o").await.map(|_| ())), BusinessErrorKind::Infrastructure);
    assert_eq!(kind(PushDeviceGateway::remove_invalid_token(&db, "t").await), BusinessErrorKind::Infrastructure);
    assert_eq!(kind(PushDeviceGateway::delete_all_for_person(&db, "o").await), BusinessErrorKind::Infrastructure);
}

#[tokio::test]
async fn notification_gateway_and_use_case_report_database_outage() {
    let db = unreachable_database().await;
    let gateway = MentionNotificationGateway::new(&db);

    assert!(gateway.enqueue_many(vec![event()]).await.is_err());
    assert!(gateway.list_pending(5).await.is_err());
    assert!(gateway.claim_pending("e1").await.is_err());
    assert!(gateway.mark_processed("e1").await.is_err());
    assert!(gateway.mark_failed("e1", 1, "boom").await.is_err());
    assert!(gateway.persist_in_app_notification(notification()).await.is_err());
    assert!(gateway.persist_friendship_notification(notification()).await.is_err());
    assert!(gateway.list_pending_push_notifications(5).await.is_err());
    assert!(gateway.claim_push_notification("n1").await.is_err());
    assert!(gateway.update_push_state("n1", "Pending", 1, &[], None, Some("boom")).await.is_err());
    assert!(gateway.update_push_state("n1", "Processing", 1, &[], None, None).await.is_err());
    assert!(gateway.list_in_app_notifications("recipient", true, 5).await.is_err());
    assert!(gateway.mark_in_app_notification_read("n1", "recipient").await.is_err());
    assert!(gateway.delete_all_involving_person("recipient").await.is_err());

    // The use case propagates the same failures; process_pending cannot even list the queue.
    assert!(MentionNotificationUseCase::enqueue(&db, vec![event()]).await.is_err());
    assert!(MentionNotificationUseCase::process_pending(&db, 5).await.is_err());
    assert!(MentionNotificationUseCase::list_notifications(&db, "recipient", false, 5).await.is_err());
    assert!(MentionNotificationUseCase::mark_as_read(&db, "recipient", "n1").await.is_err());
    assert!(MentionNotificationUseCase::persist_friendship_notification(&db, notification()).await.is_err());
}

#[tokio::test]
async fn evolution_gateway_degrades_to_empty_history_and_errors_on_writes() {
    let db = unreachable_database().await;
    let gateway = EvolutionCheckInGateway::new(&db);
    let check_in = || {
        EvolutionCheckIn::new(
            "c1".into(), "owner".into(), DateTime::from_millis(1_000), None, Visibility::Private, None, None,
        )
    };

    // Reads never fail the request: an outage yields an empty history.
    let from = DateTime::from_millis(0);
    let to = DateTime::from_millis(2_000);
    assert!(gateway.find_all_by_person_uuid("owner".into(), from, to).await.is_empty());
    assert!(gateway.find_check_in("c1".into()).await.is_none());
    // Writes surface the failure.
    assert!(gateway.persist_check_in(check_in()).await.is_err());
    assert!(gateway.delete_all_by_person("owner").await.is_err());
}
