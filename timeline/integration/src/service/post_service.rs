//! PostService: the post operations of `/timeline/api/posts`. The acting person comes from the
//! token and the use case REST calls decides who may see or change what.
use crate::infrastructure::mapper::{CommentMapper, PostMapper};
use crate::infrastructure::utils::{business_status, enforce, with_caller};
use crate::proto::timeline::post_service_server::PostService;
use crate::proto::timeline::*;
use business::commons::rate_limit::content_limiter;
use business::use_cases::post_use_case::PostUseCase;
use domain::reaction::Reaction;
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcPostService {
    database: Arc<Database>,
}

impl GrpcPostService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl PostService for GrpcPostService {
    async fn create_post(&self, request: Request<CreatePostRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let consent = body.third_party_consent_confirmed;
            let post = PostUseCase::create_with_consent(&db, &user, PostMapper::domain(body), consent)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(PostMapper::proto(post)))
        })
        .await
    }

    async fn delete_post(&self, request: Request<DeletePostRequest>) -> Result<Response<DeletePostResponse>, Status> {
        let (db, post_uuid) = (self.database.clone(), request.get_ref().post_uuid.clone());
        with_caller(&request, |user| async move {
            PostUseCase::delete_owned(&db, post_uuid, user.person_id, &user.person_uuid)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(DeletePostResponse {}))
        })
        .await
    }

    async fn add_comment(&self, request: Request<AddCommentRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let post_uuid = body.post_uuid.clone();
            let post = PostUseCase::add_comment(&db, &user, post_uuid, CommentMapper::domain(body))
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(PostMapper::proto(post)))
        })
        .await
    }

    async fn add_reaction(&self, request: Request<AddReactionRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let reaction_type = PostUseCase::parse_reaction_type(&body.reaction_type).map_err(|e| business_status(&e))?;
            let reaction = Reaction::new(uuid::Uuid::new_v4().to_string(), String::new(), String::new(), reaction_type);
            let post = PostUseCase::add_reaction(&db, &user, body.post_uuid, reaction)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(PostMapper::proto(post)))
        })
        .await
    }

    async fn remove_reaction(&self, request: Request<RemoveReactionRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, post_uuid) = (self.database.clone(), request.get_ref().post_uuid.clone());
        with_caller(&request, |user| async move {
            let post = PostUseCase::remove_reaction(&db, post_uuid, user.person_id, user.person_uuid)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(PostMapper::proto(post)))
        })
        .await
    }
}
