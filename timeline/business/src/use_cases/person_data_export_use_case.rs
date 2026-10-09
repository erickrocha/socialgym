use domain::business_error::BusinessError;
use futures::TryStreamExt;
use mongodb::bson::{Bson, Document, doc};
use mongodb::{Collection, Database};

pub struct PersonDataExportUseCase;

impl PersonDataExportUseCase {
    /// Everything `person_uuid` has put in the timeline, as the JSON object `workout` packs into
    /// the person's data export. Posts the person wrote are exported whole; on someone else's post
    /// only the person's own comments and reactions are exported, never that post's text, media
    /// or other people's comments.
    pub async fn export(
        db: &Database,
        person_uuid: &str,
    ) -> Result<serde_json::Value, BusinessError> {
        let rows = |name: &str, filter: Document| Self::documents(db.collection(name), filter);
        let posts = rows(
            "posts",
            doc! { "$or": [
                { "authorUuid": person_uuid },
                { "comments.authorUuid": person_uuid },
                { "reactions.authorId": person_uuid },
            ] },
        )
        .await?
        .into_iter()
        .map(|post| Self::own_content_only(post, person_uuid))
        .collect::<Vec<_>>();
        let evolutions = rows("evolutions", doc! { "personUuid": person_uuid }).await?;
        let workout_sessions = rows("workout_sessions", doc! { "personUuid": person_uuid }).await?;
        let notifications = rows(
            "in_app_notifications",
            doc! { "$or": [{ "recipientPersonUuid": person_uuid }, { "actorPersonUuid": person_uuid }] },
        )
        .await?;
        Ok(serde_json::json!({
            "posts": posts,
            "evolutions": evolutions,
            "workoutSessions": workout_sessions,
            "notifications": notifications,
        }))
    }

    fn own_content_only(post: Document, person_uuid: &str) -> Document {
        if post.get_str("authorUuid") == Ok(person_uuid) {
            return post;
        }
        let own = |key: &str, field: &str| -> Vec<Bson> {
            post.get_array(key)
                .map(|items| {
                    items
                        .iter()
                        .filter(|item| {
                            item.as_document()
                                .is_some_and(|d| d.get_str(field) == Ok(person_uuid))
                        })
                        .cloned()
                        .collect()
                })
                .unwrap_or_default()
        };
        doc! {
            "_id": post.get("_id").cloned().unwrap_or(Bson::Null),
            "comments": own("comments", "authorUuid"),
            "reactions": own("reactions", "authorId"),
        }
    }

    async fn documents(
        collection: Collection<Document>,
        filter: Document,
    ) -> Result<Vec<Document>, BusinessError> {
        let cursor = collection
            .find(filter)
            .await
            .map_err(|e| BusinessError::infrastructure(format!("export query failed: {e}")))?;
        cursor
            .try_collect()
            .await
            .map_err(|e| BusinessError::infrastructure(format!("export read failed: {e}")))
    }
}
