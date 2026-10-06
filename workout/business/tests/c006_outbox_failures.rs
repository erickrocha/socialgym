//! C-006 friendship outbox error mapping: a database outage must surface as a `BusinessError`
//! on every gateway call, and malformed ids must be rejected before touching the database.
use business::domain::friendship_outbox_event::FriendshipOutboxEvent;
use business::gateway::friendship_outbox_gateway::FriendshipOutboxGateway;
use chrono::Utc;
use entity::friendship_notification_outbox_entity as outbox;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use uuid::Uuid;

fn event(friendship: &str) -> FriendshipOutboxEvent {
    FriendshipOutboxEvent {
        event_uuid: Uuid::new_v4().to_string(),
        event_type: "friend_request_created".into(),
        friendship_uuid: friendship.into(),
        actor_person_uuid: Uuid::new_v4().to_string(),
        recipient_person_uuid: Uuid::new_v4().to_string(),
        occurred_at: Utc::now(),
    }
}

fn model() -> outbox::Model {
    let now = Utc::now();
    outbox::Model {
        id: 1,
        event_uuid: Uuid::new_v4(),
        friendship_uuid: Uuid::new_v4(),
        event_type: "friend_request_created".into(),
        actor_person_uuid: Uuid::new_v4(),
        recipient_person_uuid: Uuid::new_v4(),
        occurred_at: now,
        attempt_count: 0,
        next_attempt_at: now,
        last_error: None,
        published_at: None,
        created_at: now,
        updated_at: now,
    }
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn c006_outbox_gateway_reports_database_outage_and_rejects_malformed_ids() {
    let url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    // Malformed ids never reach the database.
    let bad = FriendshipOutboxGateway::persist(&db, event("not-a-uuid")).await.unwrap_err();
    assert!(bad.message.contains("Invalid friendship event UUID"), "{}", bad.message);

    // Healthy round trip so the later failures are attributable to the closed connection.
    let valid = event(&Uuid::new_v4().to_string());
    FriendshipOutboxGateway::persist(&db, valid.clone()).await.unwrap();
    assert!(FriendshipOutboxGateway::persist(&db, valid).await.is_err(), "event uuid is unique");
    assert!(!FriendshipOutboxGateway::find_pending(&db, 10).await.unwrap().is_empty());

    db.close_by_ref().await.unwrap();
    let message = |e: business::domain::business_error::BusinessError| e.message;
    assert!(message(FriendshipOutboxGateway::find_pending(&db, 10).await.unwrap_err()).contains("pending friendship events"));
    assert!(message(FriendshipOutboxGateway::has_earlier_unpublished(&db, &model()).await.unwrap_err()).contains("event order"));
    assert!(message(FriendshipOutboxGateway::persist(&db, event(&Uuid::new_v4().to_string())).await.unwrap_err()).contains("persist friendship event"));
    assert!(message(FriendshipOutboxGateway::mark_published(&db, model()).await.unwrap_err()).contains("mark friendship event published"));
    assert!(message(FriendshipOutboxGateway::mark_failed(&db, model(), "boom").await.unwrap_err()).contains("retry"));
}
