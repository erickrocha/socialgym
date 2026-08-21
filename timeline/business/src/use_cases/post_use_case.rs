use crate::commons::grpc_config::GrpcConfig;
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::post_gateway::PostGateway;
use crate::repositories::repository::Repository;
use crate::use_cases::media_use_case::MediaUseCase;
use crate::commons::authorization::ensure_owns;
use domain::business_error::BusinessError;
use domain::user::User;
use domain::comment::Comment;
use domain::post::Post;
use domain::reaction::Reaction;
use futures::stream::StreamExt;
use mongodb::Database;
use crate::use_cases::mention_use_case::MentionUseCase;

pub struct PostUseCase {}

const DEFAULT_FEED_PAGE_SIZE: u64 = 20;

impl PostUseCase {
    /// Publish a post authored by `author`. Author identity comes from the
    /// authenticated principal, never from the request body — otherwise content
    /// could be attributed to anyone.
    /// MongoDB imposes no column width, so unlike the Postgres side this is the
    /// only thing standing between a client and an unbounded post/comment.
    const MAX_CONTENT_LEN: usize = 5000;

    pub async fn create(db: &Database, author: &User, mut post: Post) -> Result<Post, BusinessError> {
        if post.content.len() > Self::MAX_CONTENT_LEN {
            return Err(BusinessError::validation(format!(
                "content must be at most {} characters",
                Self::MAX_CONTENT_LEN
            )));
        }
        let author_person_id = author.person_id;
        post.author_id = author_person_id;
        post.author_uuid = author.person_uuid.clone();
        post.author_name = author.name.clone();
        log::info!("Creating post by author: {}", post.author_id);
        let persisted = PostGateway::new(db).persist(post).await?;

        if let Err(e) = MentionUseCase::enqueue_mentions_from_post(db, author_person_id, &persisted).await {
            log::error!("Failed to enqueue post mention notifications: {}",e.message);
        }

        Ok(persisted)
    }

    pub async fn delete(db: &Database, uuid: String,) -> Result<(), BusinessError> {
        log::info!("Deleting post by id: {}", uuid);
        let deleted_post = PostGateway::new(db).delete(uuid).await;
        if let Err(e) = deleted_post {
            log::error!("Failed to delete post: {}", e.message);
        }
        Ok(())
    }

    // pub async fn delete_comment(db: &Database, post_uuid: String, comment_uuid: String) -> Result<Post, BusinessError> {
    //     log::info!("Deleting comment {} from post {}", comment_uuid, post_uuid);
    //     PostGateway::new(db).delete_comment(&post_uuid, &comment_uuid).await
    // }

    pub async fn find_by_id(db: &Database, id: String) -> Option<Post> {
        log::info!("Finding post by id: {}", id);
        PostGateway::new(db).find_by_id(id).await
    }
    pub async fn get_feed(db: &Database, person_id: i32, person_uuid: String, page: u32) -> Result<Vec<Post>, BusinessError> {
        log::info!("Fetching friend feed for person_uuid: {}, page: {}",person_uuid,page);

        let endpoint = GrpcConfig::build_endpoint();

        let gateway = FriendGateway::new(endpoint);
        let mut friend_uuids = gateway.find_friend_uuids(person_id, &person_uuid).await?;
        friend_uuids.push(person_uuid.clone());
        if friend_uuids.is_empty() {
            return Ok(Vec::new());
        }

        let gateway = PostGateway::new(db);
        let posts = gateway.find_feed(friend_uuids, Self::feed_skip(page, DEFAULT_FEED_PAGE_SIZE), DEFAULT_FEED_PAGE_SIZE).await?;
        let response = Self::fill_posts(posts).await;
        Ok(response)
    }

    pub async fn get_business_feed(db: &Database, business_profile_uuid: String, page: u32) -> Result<Vec<Post>, BusinessError> {
        log::info!("Fetching business feed for business_profile_uuid: {}, page: {}",business_profile_uuid,page);

        let uuids = vec![business_profile_uuid.clone()];

        let gateway = PostGateway::new(db);
        let posts = gateway.find_feed(uuids, Self::feed_skip(page, DEFAULT_FEED_PAGE_SIZE), DEFAULT_FEED_PAGE_SIZE).await?;
        let response = Self::fill_posts(posts).await;
        Ok(response)
    }


    pub async fn add_comment(db: &Database, author: &User, post_id: String, mut comment: Comment) -> Result<Post, BusinessError> {
        if comment.content.len() > Self::MAX_CONTENT_LEN {
            return Err(BusinessError::validation(format!(
                "content must be at most {} characters",
                Self::MAX_CONTENT_LEN
            )));
        }
        let author_person_id = author.person_id;
        comment.author_uuid = author.person_uuid.clone();
        comment.author_name = author.name.clone();
        log::info!("Adding comment to post: {}", post_id);
        let uuid = comment.uuid.clone();
        let persisted_post = PostGateway::new(db).add_comment(&post_id, comment).await?;

        if let Some(persisted_comment) = persisted_post
            .comments
            .iter()
            .find(|c| c.uuid == uuid.as_str())
            && let Err(e) = MentionUseCase::enqueue_mentions_from_comment(db, author_person_id, &persisted_post, persisted_comment).await
        {
            log::error!("Failed to enqueue comment mention notifications: {}",e.message);
        }
        Ok(persisted_post)
    }
    pub async fn add_reaction(db: &Database, author: &User, post_id: String, mut reaction: Reaction) -> Result<Post, BusinessError> {
        reaction.author_id = author.person_uuid.clone();
        reaction.author_name = author.name.clone();
        log::info!("Adding reaction to post: {}", post_id);
        PostGateway::new(db).add_reaction(&post_id, reaction).await
    }
    pub async fn remove_reaction(db: &Database, post_id: String, person_uuid: String) -> Result<Post, BusinessError> {
        log::info!("Removing reaction from post: {}", post_id);
        PostGateway::new(db).remove_reaction(&post_id, &person_uuid).await
    }

    /// Only the author may delete their own post.
    pub async fn delete_owned(db: &Database, uuid: String, acting_person_uuid: &str) -> Result<(), BusinessError> {
        let post = Self::find_by_id(db, uuid.clone())
            .await
            .ok_or_else(|| BusinessError::not_found("Post not found"))?;
        ensure_owns(&post.author_uuid, acting_person_uuid)?;
        PostGateway::new(db).delete(uuid).await.map(|_| ())
    }

    async fn fill_posts(posts: Vec<Post>) -> Vec<Post> {
        const PER_POST_CONCURRENCY: usize = 6;

        let mut posts = posts;

        for post in &mut posts {
            if post.media.is_empty() {
                continue;
            }

            let keys: Vec<String> = post.media.iter().map(|m| m.object_key.clone()).collect();

            let results = futures::stream::iter(keys.into_iter().enumerate())
                .map(|(idx, key)| async move {
                    let res = MediaUseCase::generate_cloud_front_signed_url(&key).await;
                    (idx, res)
                })
                .buffer_unordered(PER_POST_CONCURRENCY)
                .collect::<Vec<(usize, Result<String, BusinessError>)>>()
                .await;

            for (idx, res) in results {
                match res {
                    Ok(signed_url) => {
                        if let Some(media) = post.media.get_mut(idx) {
                            media.url = signed_url;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to generate signed url for media idx {} in post {:?}: {:?}",idx,post.uuid,e);
                    }
                }
            }
        }
        posts
    }

    fn feed_skip(page: u32, page_size: u64) -> u64 {
        u64::from(page).saturating_mul(page_size)
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::post_use_case::PostUseCase;

    #[test]
    fn page_zero_has_no_skip() {
        assert_eq!(PostUseCase::feed_skip(0, 20), 0);
    }

    #[test]
    fn page_one_skips_one_full_page() {
        assert_eq!(PostUseCase::feed_skip(1, 20), 20);
    }

}

