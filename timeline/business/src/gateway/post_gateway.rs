use crate::repositories::repository::Repository;
use async_trait::async_trait;
use domain::business_error::BusinessError;
use domain::comment::Comment;
use domain::post::Post;
use domain::reaction::Reaction;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, to_bson};
use mongodb::{Collection, Database};

const COLLECTION_NAME: &str = "posts";

pub struct PostGateway {
    collection: Collection<Post>,
}

impl PostGateway {
    pub async fn remove_comment_for_moderation(
        &self,
        post_id: &str,
        comment_id: &str,
    ) -> Result<(), BusinessError> {
        self.collection
            .update_one(
                doc! { "_id": post_id },
                doc! { "$pull": { "comments": { "_id": comment_id } } },
            )
            .await
            .map(|_| ())
            .map_err(|e| BusinessError::infrastructure(format!("Failed to remove comment: {e}")))
    }
    pub async fn remove_media_for_moderation(
        &self,
        post_id: &str,
        media_id: &str,
    ) -> Result<(), BusinessError> {
        self.collection
            .update_one(
                doc! { "_id": post_id },
                doc! { "$pull": { "media": { "uuid": media_id } } },
            )
            .await
            .map(|_| ())
            .map_err(|e| BusinessError::infrastructure(format!("Failed to remove media: {e}")))
    }
    pub fn new(db: &Database) -> PostGateway {
        Self {
            collection: db.collection::<Post>(COLLECTION_NAME),
        }
    }
}

#[async_trait]
impl Repository<Post, String> for PostGateway {
    async fn persist(&self, entity: Post) -> Result<Post, BusinessError> {
        let result = self.collection.insert_one(entity).await;
        if let Err(e) = result {
            log::error!("Error inserting post: {:?}", e);
            return Err(BusinessError::new("Failed to insert post".to_string()));
        }
        let inserted_id = result.unwrap().inserted_id;
        self.collection
            .find_one(doc! { "_id": inserted_id })
            .await
            .map_err(|e| {
                log::error!("Error fetching inserted post: {:?}", e);
                BusinessError::new("Failed to retrieve inserted post".to_string())
            })?
            .ok_or_else(|| BusinessError::new("Inserted post not found".to_string()))
    }

    async fn find_by_id(&self, id: String) -> Option<Post> {
        self.collection
            .find_one(doc! { "_id": &id })
            .await
            .unwrap_or(None)
    }

    async fn update(&self, id: String, entity: Post) -> Result<Post, BusinessError> {
        let result = self
            .collection
            .replace_one(doc! { "_id": id.clone() }, entity)
            .await;
        match result {
            Ok(r) if r.matched_count > 0 => self
                .find_by_id(id)
                .await
                .ok_or_else(|| BusinessError::new("Post not found after update".to_string())),
            Ok(_) => Err(BusinessError::new("No post found to update".to_string())),
            Err(e) => {
                log::error!("Error updating post: {:?}", e);
                Err(BusinessError::new("Failed to update post".to_string()))
            }
        }
    }

    async fn delete(&self, id: String) -> Result<bool, BusinessError> {
        self.collection
            .delete_one(doc! { "_id": id })
            .await
            .map(|r| r.deleted_count > 0)
            .map_err(|e| {
                log::error!("Error deleting post: {:?}", e);
                BusinessError::new("Failed to delete post".to_string())
            })
    }
}

impl PostGateway {
    /// Returns posts for the given authors, sorted newest-first and paginated.
    pub async fn find_feed(
        &self,
        author_ids: Vec<String>,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Post>, BusinessError> {
        if author_ids.is_empty() {
            return Ok(Vec::new());
        }

        let filter = doc! {
            "authorUuid": { "$in": author_ids }
        };

        let mut cursor = self
            .collection
            .find(filter)
            .sort(doc! { "createdAt": -1 })
            .skip(skip)
            .limit(i64::try_from(limit).unwrap_or(i64::MAX))
            .await
            .map_err(|e| {
                log::error!("Error fetching feed: {:?}", e);
                BusinessError::new("Failed to fetch feed".to_string())
            })?;

        let mut posts = Vec::new();
        while let Some(post) = cursor.try_next().await.map_err(|e| {
            log::error!("Error iterating post cursor: {:?}", e);
            BusinessError::new("Failed to iterate posts".to_string())
        })? {
            posts.push(post);
        }
        Ok(posts)
    }

    /// Pushes a comment (or reply) into the post's comments array.
    pub async fn add_comment(
        &self,
        post_id: &str,
        comment: Comment,
    ) -> Result<Post, BusinessError> {
        let comment_bson = to_bson(&comment).map_err(|e| {
            log::error!("Error serializing comment: {:?}", e);
            BusinessError::new("Failed to serialize comment".to_string())
        })?;

        self.collection
            .update_one(
                doc! { "_id": post_id },
                doc! { "$push": { "comments": comment_bson } },
            )
            .await
            .map_err(|e| {
                log::error!("Error adding comment: {:?}", e);
                BusinessError::new("Failed to add comment".to_string())
            })?;

        self.find_by_id(post_id.to_string())
            .await
            .ok_or_else(|| BusinessError::new("Post not found after adding comment".to_string()))
    }

    /// Upserts a reaction: removes any existing reaction from the same person,
    /// then pushes the new one (one reaction type per person, like Facebook).
    pub async fn add_reaction(
        &self,
        post_id: &str,
        reaction: Reaction,
    ) -> Result<Post, BusinessError> {
        let reaction_bson = to_bson(&reaction).map_err(|e| {
            log::error!("Error serializing reaction: {:?}", e);
            BusinessError::new("Failed to serialize reaction".to_string())
        })?;

        self.collection
            .update_one(
                doc! { "_id": post_id },
                doc! { "$push": { "reactions": reaction_bson } },
            )
            .await
            .map_err(|e| {
                log::error!("Error adding reaction: {:?}", e);
                BusinessError::new("Failed to add reaction".to_string())
            })?;

        self.find_by_id(post_id.to_string())
            .await
            .ok_or_else(|| BusinessError::new("Post not found after adding reaction".to_string()))
    }

    /// Removes the caller's reaction from the post.
    pub async fn remove_reaction(
        &self,
        post_id: &str,
        person_uuid: &str,
    ) -> Result<Post, BusinessError> {
        self.collection
            .update_one(
                doc! { "_id": post_id },
                // `Reaction` serialises the reactor as `authorId`; matching on
                // `personId` here never removed anything.
                doc! { "$pull": { "reactions": { "authorId": person_uuid } } },
            )
            .await
            .map_err(|e| {
                log::error!("Error removing reaction: {:?}", e);
                BusinessError::new("Failed to remove reaction".to_string())
            })?;

        self.find_by_id(post_id.to_string())
            .await
            .ok_or_else(|| BusinessError::new("Post not found after removing reaction".to_string()))
    }

    /// Account-deletion cascade: deletes every post authored by this person
    /// (also removes their embedded comments/reactions on those posts, and any
    /// comments/reactions others left there, since the whole post is gone).
    pub async fn delete_all_by_author(&self, person_uuid: &str) -> Result<(), BusinessError> {
        self.collection
            .delete_many(doc! { "authorUuid": person_uuid })
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!("Error deleting posts by author: {:?}", e);
                BusinessError::new("Failed to delete posts".to_string())
            })
    }

    /// Account-deletion cascade: strips this person's reactions from every
    /// *other* post they reacted to (their own posts are deleted wholesale by
    /// `delete_all_by_author`, so this only matters for posts they don't own).
    pub async fn pull_reactions_by_author(&self, person_uuid: &str) -> Result<(), BusinessError> {
        self.collection
            .update_many(
                doc! {},
                doc! { "$pull": { "reactions": { "authorId": person_uuid } } },
            )
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!("Error pulling reactions by author: {:?}", e);
                BusinessError::new("Failed to remove reactions".to_string())
            })
    }

    /// Account-deletion cascade: strips this person's comments from every
    /// *other* post they commented on.
    pub async fn pull_comments_by_author(&self, person_uuid: &str) -> Result<(), BusinessError> {
        self.collection
            .update_many(
                doc! {},
                doc! { "$pull": { "comments": { "authorUuid": person_uuid } } },
            )
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!("Error pulling comments by author: {:?}", e);
                BusinessError::new("Failed to remove comments".to_string())
            })
    }
}
