use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::{Database, IndexModel};

/// MongoDB creates no indexes beyond `_id` by default, so every filtered query
/// here was a full collection scan until now. `create_index` is idempotent
/// (a repeat call with the same keys/options is a no-op), so this can safely
/// run on every boot instead of needing a one-off migration step.
pub async fn ensure_indexes(db: &Database) {
    let targets: [(&str, &str); 7] = [
        ("posts", "authorUuid"),
        ("workouts", "personUuid"),
        ("evolutions", "personUuid"),
        ("mention_notification_events", "status"),
        ("in_app_notifications", "recipientPersonUuid"),
        ("messages", "conversationUuid"),
        ("conversations", "businessProfileUuid"),
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

    // ── Chat ────────────────────────────────────────────────────────────────
    let compound: [(&str, mongodb::bson::Document); 3] = [
        // conversation list: everything a person can see, newest activity first
        ("conversations", doc! { "participantPersonUuids": 1, "updatedAt": -1 }),
        // paged conversation history (with a stable same-millisecond tie-break)
        ("messages", doc! { "conversationUuid": 1, "sentAt": -1, "_id": -1 }),
        // cheap ChatMessage filter for the unread badge
        ("in_app_notifications", doc! { "recipientPersonUuid": 1, "notificationType": 1 }),
    ];
    for (collection, keys) in compound {
        let index = IndexModel::builder().keys(keys).build();
        if let Err(e) = db
            .collection::<mongodb::bson::Document>(collection)
            .create_index(index)
            .await
        {
            log::error!("Failed to create compound index on {collection}: {e}");
        }
    }

    let unique: [(&str, mongodb::bson::Document); 2] = [
        // get-or-create race guard
        ("conversations", doc! { "dedupeKey": 1 }),
        // idempotent send
        ("messages", doc! { "dedupeKey": 1 }),
    ];
    for (collection, keys) in unique {
        let index = IndexModel::builder()
            .keys(keys)
            .options(IndexOptions::builder().unique(true).build())
            .build();
        if let Err(e) = db
            .collection::<mongodb::bson::Document>(collection)
            .create_index(index)
            .await
        {
            log::error!("Failed to create unique index on {collection}.dedupeKey: {e}");
        }
    }
}
