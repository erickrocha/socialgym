use mongodb::bson::doc;
use mongodb::{Database, IndexModel};

/// MongoDB creates no indexes beyond `_id` by default, so every filtered query
/// here was a full collection scan until now. `create_index` is idempotent
/// (a repeat call with the same keys/options is a no-op), so this can safely
/// run on every boot instead of needing a one-off migration step.
pub async fn ensure_indexes(db: &Database) {
    let targets: [(&str, &str); 5] = [
        ("posts", "authorUuid"),
        ("workouts", "personUuid"),
        ("evolutions", "personUuid"),
        ("mention_notification_events", "status"),
        ("in_app_notifications", "recipientPersonUuid"),
    ];

    for (collection, field) in targets {
        let index = IndexModel::builder().keys(doc! { field: 1 }).build();
        if let Err(e) = db
            .collection::<mongodb::bson::Document>(collection)
            .create_index(index)
            .await
        {
            log::error!("Failed to create index on {}.{}: {}", collection, field, e);
        }
    }

    // Sessions are almost always queried by person + date range together.
    let session_index = IndexModel::builder()
        .keys(doc! { "personUuid": 1, "startedAt": 1 })
        .build();
    if let Err(e) = db
        .collection::<mongodb::bson::Document>("workouts")
        .create_index(session_index)
        .await
    {
        log::error!("Failed to create compound index on workouts.(personUuid, startedAt): {}", e);
    }
}
