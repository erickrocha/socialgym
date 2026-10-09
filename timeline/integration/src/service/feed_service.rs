//! FeedService: the main feed and the feed of a Business Profile (`/timeline/api/feed`).
use crate::infrastructure::mapper::PostMapper;
use crate::infrastructure::utils::{business_status, with_caller};
use crate::proto::timeline::feed_service_server::FeedService;
use crate::proto::timeline::*;
use business::use_cases::post_use_case::PostUseCase;
use domain::post::Post as DomainPost;
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcFeedService {
    database: Arc<Database>,
}

impl GrpcFeedService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// Posts as REST returns them in a feed: with avatar and media URLs signed.
    async fn feed_response(posts: Vec<DomainPost>) -> Result<Response<FeedResponse>, Status> {
        let url_cache = PostUseCase::signed_urls_for(&posts)
            .await
            .map_err(|e| business_status(&e))?;
        Ok(Response::new(FeedResponse {
            posts: posts
                .into_iter()
                .map(|post| PostMapper::proto(PostUseCase::with_signed_urls(post, &url_cache)))
                .collect(),
        }))
    }
}

#[tonic::async_trait]
impl FeedService for GrpcFeedService {
    async fn get_feed(
        &self,
        request: Request<GetFeedRequest>,
    ) -> Result<Response<FeedResponse>, Status> {
        let (db, page) = (self.database.clone(), request.get_ref().page);
        let posts = with_caller(&request, |user| async move {
            PostUseCase::get_feed(&db, user.person_id, user.person_uuid, page)
                .await
                .map_err(|e| business_status(&e))
        })
        .await?;
        Self::feed_response(posts).await
    }

    async fn get_feed_by_author(
        &self,
        request: Request<GetFeedByAuthorRequest>,
    ) -> Result<Response<FeedResponse>, Status> {
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        let posts = with_caller(&request, |_| async move {
            PostUseCase::get_business_feed(&db, body.author_uuid, body.page)
                .await
                .map_err(|e| business_status(&e))
        })
        .await?;
        Self::feed_response(posts).await
    }
}
