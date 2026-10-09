mod support;

use business::gateway::aws_clients::{s3_client, sqs_client};
use business::use_cases::sqs_consumer_use_case::SqsConsumerUseCase;
use entity::{person_entity, person_media_entity};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;
use support::{fresh_db, localstack_bucket_and_queue, register};

fn event(bucket: &str, name: &str, key: &str) -> String {
    json!({ "Records": [{ "eventName": name, "s3": { "bucket": { "name": bucket }, "object": { "key": key, "size": 3 } } }] }).to_string()
}

async fn put(bucket: &str, key: &str, content_type: &str) {
    s3_client()
        .await
        .put_object()
        .bucket(bucket)
        .key(key)
        .content_type(content_type)
        .body(b"abc".to_vec().into())
        .send()
        .await
        .unwrap();
}

async fn send(queue: &str, body: String) {
    sqs_client()
        .await
        .send_message()
        .queue_url(queue)
        .message_body(body)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires TEST_DATABASE_URL and the infra/test LocalStack (S3 and SQS on :4566)"]
async fn media_events_are_persisted_idempotently_and_sync_the_person_images() {
    let db = fresh_db().await;
    let (bucket, queue) = localstack_bucket_and_queue().await;
    let person = register(&db, 1).await;
    let me = person.person_uuid.clone();

    // without a configured queue the consumer is a no-op
    unsafe { std::env::remove_var("AWS_SQS_QUEUE_URL") };
    assert_eq!(SqsConsumerUseCase::poll_and_process(&db).await.unwrap(), 0);
    unsafe { std::env::set_var("AWS_SQS_QUEUE_URL", &queue) };

    let avatar = format!("person/{me}/avatar/{}", uuid::Uuid::new_v4());
    let cover = format!("person/{me}/cover/{}", uuid::Uuid::new_v4());
    let gallery = format!("person/{me}/gallery/{}", uuid::Uuid::new_v4());
    put(&bucket, &avatar, "image/png").await;
    put(&bucket, &cover, "image/jpeg").await;
    put(&bucket, &gallery, "video/mp4").await;

    send(&queue, event(&bucket, "ObjectCreated:Put", &avatar)).await;
    send(&queue, event(&bucket, "ObjectCreated:Put", &cover)).await;
    send(&queue, event(&bucket, "ObjectCreated:Put", &gallery)).await;
    send(&queue, event(&bucket, "ObjectCreated:Put", &avatar)).await; // duplicate delivery
    send(
        &queue,
        event(&bucket, "ObjectCreated:Put", &gallery.replace('/', "%2F")),
    )
    .await; // keys arrive URL-encoded
    send(&queue, event(&bucket, "ObjectRemoved:Delete", &avatar)).await; // not a creation
    send(
        &queue,
        event(&bucket, "ObjectCreated:Put", "uploads/owner-1/loose-file"),
    )
    .await; // not an album path
    send(
        &queue,
        event(
            &bucket,
            "ObjectCreated:Put",
            &format!("person/{}/avatar/x", uuid::Uuid::new_v4()),
        ),
    )
    .await; // unknown person
    send(&queue, "this is not json".to_string()).await; // malformed
    send(&queue, json!({ "Type": "Notification", "Message": event(&bucket, "ObjectCreated:Put", &format!("person/{me}/gallery/{}", uuid::Uuid::new_v4())) }).to_string()).await; // SNS envelope

    // every message is acknowledged, whether it was stored or skipped for good
    let mut handled = 0;
    for _ in 0..4 {
        handled += SqsConsumerUseCase::poll_and_process(&db).await.unwrap();
        if handled >= 10 {
            break;
        }
    }
    assert_eq!(handled, 10);

    let stored = person_media_entity::Entity::find().all(&db).await.unwrap();
    assert_eq!(
        stored.len(),
        4,
        "avatar, cover, gallery and the SNS-wrapped gallery item; duplicates and skips add nothing"
    );
    assert!(stored
        .iter()
        .any(|m| m.s3_key == gallery && m.album == "gallery"));
    assert!(
        stored
            .iter()
            .any(|m| m.mime_type == "application/octet-stream"),
        "an object S3 cannot describe is stored as octet-stream instead of poisoning the queue"
    );
    let row = person_entity::Entity::find_by_id(person.person_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.avatar.as_deref(), Some(avatar.as_str()));
    assert_eq!(row.cover_image.as_deref(), Some(cover.as_str()));
    assert_eq!(
        person_media_entity::Entity::find()
            .count(&db)
            .await
            .unwrap(),
        4
    );
}
