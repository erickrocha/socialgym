use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::enums::MimeType;
use crate::domain::person_media::{PersonMedia, PersonMediaEntityMapper};
use crate::gateway::person_gateway::PersonGateway;
use crate::gateway::person_media_gateway::PersonMediaGateway;
use crate::gateway::s3_gateway::S3Gateway;
use crate::gateway::sqs_gateway::SqsGateway;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_sqs::Client as SqsClient;
use sea_orm::DbConn;
use serde::Deserialize;
use std::env;

const AWS_SQS_QUEUE_URL: &str = "AWS_SQS_QUEUE_URL";
const AWS_WORKOUT_BUCKET: &str = "AWS_WORKOUT_BUCKET";

// ---------------------------------------------------------------------------
// S3 event JSON shapes
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct S3Event {
    #[serde(rename = "Records")]
    records: Vec<S3EventRecord>,
}

#[derive(Debug, Deserialize)]
struct S3EventRecord {
    #[serde(rename = "eventName")]
    event_name: String,
    s3: S3EventData,
}

#[derive(Debug, Deserialize)]
struct S3EventData {
    bucket: S3BucketData,
    object: S3ObjectData,
}

#[derive(Debug, Deserialize)]
struct S3BucketData {
    name: String,
}

#[derive(Debug, Deserialize)]
struct S3ObjectData {
    key: String,
    size: Option<i64>,
}

// ---------------------------------------------------------------------------
// Consumer use case
// ---------------------------------------------------------------------------

pub struct SqsConsumerUseCase {}

impl SqsConsumerUseCase {
    /// Performs a single long-poll against SQS, processes every `ObjectCreated`
    /// record, and acknowledges (deletes) each successfully handled message.
    ///
    /// Messages that fail processing are **not** deleted so the visibility
    /// timeout expires and they are retried (or sent to a DLQ if configured).
    ///
    /// Returns the number of messages successfully processed.
    pub async fn poll_and_process(db: &DbConn) -> Result<usize, BusinessError> {
        let queue_url = match env::var(AWS_SQS_QUEUE_URL) {
            Ok(v) if !v.is_empty() => v,
            _ => {
                log::warn!("AWS_SQS_QUEUE_URL is not set – SQS consumer will not poll");
                return Ok(0);
            }
        };

        let bucket = env::var(AWS_WORKOUT_BUCKET).unwrap_or_default();

        let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let sqs_client = SqsClient::new(&shared_config);
        let s3_client = S3Client::new(&shared_config);

        // Long-poll: wait up to 20 s, fetch up to 10 messages per call
        let messages = SqsGateway::receive_messages(&sqs_client, &queue_url, 10, 20).await?;

        if messages.is_empty() {
            return Ok(0);
        }

        let mut processed = 0usize;

        for message in messages {
            let receipt_handle = match message.receipt_handle() {
                Some(rh) => rh.to_string(),
                None => {
                    log::warn!("SQS message has no receipt handle – skipping");
                    continue;
                }
            };

            let body = match message.body() {
                Some(b) => b.to_string(),
                None => {
                    log::warn!("SQS message has no body – discarding");
                    let _ =
                        SqsGateway::delete_message(&sqs_client, &queue_url, &receipt_handle).await;
                    continue;
                }
            };

            match Self::process_message(db, &s3_client, &bucket, &body).await {
                Ok(_) => {
                    if let Err(e) =
                        SqsGateway::delete_message(&sqs_client, &queue_url, &receipt_handle).await
                    {
                        log::error!(
                            "Could not delete SQS message after successful processing: {:?}",
                            e
                        );
                    }
                    processed += 1;
                }
                Err(e) => {
                    log::error!(
                        "Failed to process SQS message – will retry after visibility timeout: {:?}",
                        e
                    );
                    // intentionally not deleting: let the visibility timeout expire
                }
            }
        }

        Ok(processed)
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Parses the SQS message body as an S3 event JSON and persists every
    /// `ObjectCreated` record to the `person_media` table.
    ///
    /// Returns `Ok(())` for both successful processing and permanent/skippable
    /// errors (malformed events, unknown person, already-processed records) so
    /// the caller always deletes the message and avoids infinite retries.
    /// Only returns `Err` for **transient** failures (e.g. DB connectivity)
    /// where a retry after the visibility timeout makes sense.
    async fn process_message(
        db: &DbConn,
        s3_client: &S3Client,
        bucket: &str,
        body: &str,
    ) -> Result<(), BusinessError> {
        // Unwrap an optional SNS envelope so we handle both S3→SQS and
        // S3→SNS→SQS delivery paths.
        let event_json = Self::unwrap_sns_envelope(body);

        let event: S3Event = match serde_json::from_str(&event_json) {
            Ok(e) => e,
            Err(e) => {
                // Malformed body – nothing we can do; discard permanently.
                log::error!("Malformed S3 event body (will discard message): {e}\nbody={body}");
                return Ok(());
            }
        };

        for record in event.records {
            if !record.event_name.starts_with("ObjectCreated") {
                log::debug!("Ignoring non-create S3 event: {}", record.event_name);
                continue;
            }

            // S3 URL-encodes object keys in event notifications
            let raw_key = &record.s3.object.key;
            let object_key = urlencoding::decode(raw_key)
                .map(|cow| cow.into_owned())
                .unwrap_or_else(|_| raw_key.clone());

            let effective_bucket = if bucket.is_empty() {
                record.s3.bucket.name.as_str()
            } else {
                bucket
            };

            log::info!(
                "Processing S3 ObjectCreated event: bucket={} key={} size={:?}",
                effective_bucket,
                object_key,
                record.s3.object.size,
            );

            // persist_media already distinguishes transient vs permanent errors
            Self::persist_media(db, s3_client, effective_bucket, &object_key).await?;
        }

        Ok(())
    }

    /// Resolves `person_uuid` and `album` from the S3 key, fetches the MIME
    /// type via a `HeadObject` call, and inserts a row in `person_media`.
    ///
    /// Expected key format: `person/{person_uuid}/{album}/{file_uuid}`
    ///
    /// **Permanent / skippable** errors (bad key format, unknown person,
    /// duplicate s3_key) return `Ok(())` so the caller deletes the message
    /// and avoids an infinite retry loop.
    ///
    /// Only **transient** errors (DB connectivity failures) propagate as `Err`
    /// so that the visibility timeout can trigger a genuine retry.
    async fn persist_media(
        db: &DbConn,
        s3_client: &S3Client,
        bucket: &str,
        object_key: &str,
    ) -> Result<(), BusinessError> {
        // ---- Parse key segments ----
        let parts: Vec<&str> = object_key.splitn(4, '/').collect();
        if parts.len() < 4 || parts[0] != "person" {
            log::warn!(
                "Object key '{}' does not match album path 'person/{{uuid}}/{{album}}/{{file}}' – discarding",
                object_key
            );
            return Ok(()); // permanent skip
        }
        let person_uuid = parts[1];
        let album = parts[2];

        // ---- Idempotency: skip if already saved ----
        match PersonMediaGateway::find_by_s3_key(db, object_key).await {
            Ok(Some(existing)) => {
                log::info!(
                    "s3_key '{}' already recorded as person_media id={} – skipping duplicate",
                    object_key,
                    existing.id
                );
                return Ok(());
            }
            Ok(None) => {} // not yet stored, continue
            Err(e) => {
                // DB error – treat as transient, propagate so the message is retried
                return Err(BusinessError::new(format!("DB error checking for duplicate s3_key '{}': {e}",object_key)));
            }
        }

        // ---- Resolve person ----
        let person_model = match PersonGateway::find_by_uuid(db, person_uuid).await {
            Ok(Some(p)) => p,
            Ok(None) => {
                // Person was deleted; nothing to do – discard permanently
                log::warn!("No person found for uuid '{}' (key '{}') – discarding message", person_uuid, object_key);
                return Ok(());
            }
            Err(e) => {
                log::warn!("Invalid person uuid '{}' in object key '{}': {} – discarding message", person_uuid, object_key, e);
                return Ok(());
            }
        };
        let person_id = person_model.id;

        // ---- Fetch MIME type via HeadObject ----
        let content_type = S3Gateway::head_object(s3_client, bucket, object_key)
            .await
            .unwrap_or_else(|e| {
                log::warn!("HeadObject failed for key '{}': {:?} – defaulting to application/octet-stream",object_key,e);
                "application/octet-stream".to_string()
            });

        let mime_type = MimeType::from_content_type(&content_type);

        log::info!("Saving person_media – person_id={} person_uuid={} album={} mime={} key={}",person_id,person_uuid,album,content_type,object_key);

        // ---- Persist to person_media ----
        let media = PersonMedia::new(person_id,person_uuid.to_string(),mime_type,album.to_string(),object_key.to_string());

        let persist_result = PersonMediaGateway::save(db, media).await;

        match persist_result {
            Ok(active_model) => {
                let saved_media = PersonMediaEntityMapper::from_active_model(active_model);
                log::info!("Saved person_media record with id={}",saved_media.id.unwrap());
            }
            Err(e) => {
                // Check for duplicate-key / unique-constraint violation and treat
                // it as a harmless idempotent case (the record exists already).
                let err_str = e.to_string().to_lowercase();
                if err_str.contains("duplicate") || err_str.contains("unique") {
                    log::warn!("Duplicate person_media for key '{}' (concurrent insert?) – treating as already saved",object_key);
                    return Ok(());
                }
                // Genuine transient DB error – propagate for retry
                log::error!("Failed to persist person_media to DB: {:?}", e);
                return Err(BusinessError::new(format!("Failed to persist media record {}", err_str)));
            }
        }

        // ---- Optionally sync person.avatar / person.cover_image ----
        Self::maybe_sync_person_image(db, person_id, album, object_key).await;
        Ok(())
    }

    /// When `album` is `"avatar"` or `"cover"`, updates the matching column on
    /// the `person` table so the existing image-delivery path keeps working.
    async fn maybe_sync_person_image(db: &DbConn, person_id: i32, album: &str, s3_key: &str) {
        use entity::person;
        use sea_orm::{ActiveModelTrait, ActiveValue::Set};

        let update_result = match album {
            "avatar" => {
                let model = person::ActiveModel {
                    id: Set(person_id),
                    avatar: Set(Some(s3_key.to_string())),
                    ..Default::default()
                };
                model.update(db).await
            }
            "cover" => {
                let model = person::ActiveModel {
                    id: Set(person_id),
                    cover_image: Set(Some(s3_key.to_string())),
                    ..Default::default()
                };
                model.update(db).await
            }
            _ => return,
        };

        if let Err(e) = update_result {
            log::error!("Could not sync person.{} for person_id={}: {:?}",album,person_id,e);
        }
    }

    /// If `body` is an SNS notification envelope, extracts the inner `Message`
    /// JSON string and returns it as an owned `String`. Otherwise, returns the
    /// original `body` as an owned `String` unchanged.
    ///
    /// This makes the consumer work regardless of whether S3 publishes directly
    /// to SQS or via an SNS topic.
    fn unwrap_sns_envelope(body: &str) -> String {
        // SNS envelopes contain a top-level "Type": "Notification" field.
        // We do a quick string-search to avoid a full JSON parse when not needed.
        if body.contains("\"Type\"") && body.contains("\"Message\"") {
            if let Ok(outer) = serde_json::from_str::<serde_json::Value>(body) {
                if let Some(inner) = outer.get("Message").and_then(|v| v.as_str()) {
                    return inner.to_owned();
                }
            }
        }
        body.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwrap_sns_envelope_passthrough_for_plain_s3_event() {
        let body = r#"{"Records":[{"eventName":"ObjectCreated:Put","s3":{"bucket":{"name":"b"},"object":{"key":"k","size":1}}}]}"#;
        assert_eq!(SqsConsumerUseCase::unwrap_sns_envelope(body), body);
    }

    #[test]
    fn unwrap_sns_envelope_extracts_inner_message() {
        let inner = r#"{"Records":[{"eventName":"ObjectCreated:Put","s3":{"bucket":{"name":"b"},"object":{"key":"k","size":1}}}]}"#;
        // Build a minimal SNS envelope
        let envelope = serde_json::json!({
            "Type": "Notification",
            "MessageId": "abc",
            "Message": inner,
        })
        .to_string();

        assert_eq!(
            SqsConsumerUseCase::unwrap_sns_envelope(&envelope),
            inner.to_string()
        );
    }

    #[test]
    fn key_segments_parsed_correctly() {
        let key = "person/uuid-1234/avatar/file-uuid-5678";
        let parts: Vec<&str> = key.splitn(4, '/').collect();
        assert_eq!(parts[0], "person");
        assert_eq!(parts[1], "uuid-1234");
        assert_eq!(parts[2], "avatar");
        assert_eq!(parts[3], "file-uuid-5678");
    }

    #[test]
    fn key_with_wrong_prefix_is_detected() {
        let key = "uploads/owner-1/some-uuid.jpg";
        let parts: Vec<&str> = key.splitn(4, '/').collect();
        // Should be skipped because parts[0] != "person"
        assert_ne!(parts[0], "person");
    }

    #[test]
    fn mime_type_from_content_type_roundtrip() {
        let mime = MimeType::from_content_type("image/jpeg");
        assert_eq!(mime.to_text(), "image/jpeg");

        let mime = MimeType::from_content_type("video/mp4");
        assert_eq!(mime.to_text(), "video/mp4");

        let mime = MimeType::from_content_type("application/octet-stream");
        assert_eq!(mime.to_text(), "application/octet-stream");
    }
}
