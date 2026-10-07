//! Handlers of the post, feed, notification, check-in and push-device services. Each one takes
//! the acting person from the token and calls the use case REST calls; nothing here decides who
//! may see or change what.
use super::auth::with_caller;
use super::convert::*;
use super::rate_limit::enforce;
use super::status::status_from_business;
use crate::authentication::rate_limit::content_limiter;
use crate::http::feed_controller::build_signed_url_cache;
use crate::infrastructure::data_tools::opt_naive_to_bson_datetime;
use crate::infrastructure::mapper::{CommentMapper, EvolutionCheckInMapper, Mapper, NotificationMapper, PostMapper};
use business::gateway::consent_gateway::ConsentGateway;
use business::gateway::evolution_check_in_gateway::EvolutionCheckInGateway;
use business::proto::proto::timeline::evolution_check_in_service_server::EvolutionCheckInService;
use business::proto::proto::timeline::feed_service_server::FeedService;
use business::proto::proto::timeline::notification_service_server::NotificationService;
use business::proto::proto::timeline::post_service_server::PostService;
use business::proto::proto::timeline::push_device_service_server::PushDeviceService;
use business::proto::proto::timeline::*;
use business::use_cases::evolution_check_in_use_case::EvolutionCheckInUseCase;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use business::use_cases::post_use_case::PostUseCase;
use business::use_cases::push_device_use_case::PushDeviceUseCase;
use chrono::{Duration, Utc};
use domain::business_error::BusinessError;
use domain::post::Post as DomainPost;
use crate::AppState;
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct Services {
    pub state: AppState,
}

impl Services {
    pub(crate) fn db(&self) -> Arc<Database> {
        self.state.database.clone()
    }
}

fn failed(error: BusinessError) -> Status {
    status_from_business(&error)
}

/// Posts as REST returns them in a feed: with avatar and media URLs signed.
async fn feed_response(posts: Vec<DomainPost>) -> Result<Response<FeedResponse>, Status> {
    let cache = build_signed_url_cache(&posts).await.map_err(failed)?;
    Ok(Response::new(FeedResponse {
        posts: posts.into_iter().map(|p| post_to(PostMapper::json_with_avatars(p, &cache))).collect(),
    }))
}

#[tonic::async_trait]
impl PostService for Services {
    async fn create_post(&self, request: Request<CreatePostRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let db = self.db();
        let body = request.get_ref().clone();
        with_caller(&request, |user| async move {
            let json = post_from(body);
            let consent = json.third_party_consent_confirmed;
            let post = PostUseCase::create_with_consent(&db, &user, PostMapper::domain(json), consent)
                .await
                .map_err(failed)?;
            Ok(Response::new(post_to(PostMapper::json(post))))
        })
        .await
    }

    async fn delete_post(&self, request: Request<DeletePostRequest>) -> Result<Response<DeletePostResponse>, Status> {
        let (db, post_uuid) = (self.db(), request.get_ref().post_uuid.clone());
        with_caller(&request, |user| async move {
            PostUseCase::delete_owned(&db, post_uuid, user.person_id, &user.person_uuid).await.map_err(failed)?;
            Ok(Response::new(DeletePostResponse {}))
        })
        .await
    }

    async fn add_comment(&self, request: Request<AddCommentRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let post_uuid = body.post_uuid.clone();
            let mut comment = CommentMapper::domain(comment_from(body));
            comment.post_uuid = post_uuid.clone();
            let post = PostUseCase::add_comment(&db, &user, post_uuid, comment).await.map_err(failed)?;
            Ok(Response::new(post_to(PostMapper::json(post))))
        })
        .await
    }

    async fn add_reaction(&self, request: Request<AddReactionRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let reaction_type = PostUseCase::parse_reaction_type(&body.reaction_type).map_err(failed)?;
            let reaction = domain::reaction::Reaction::new(
                uuid::Uuid::new_v4().to_string(),
                String::new(),
                String::new(),
                reaction_type,
            );
            let post = PostUseCase::add_reaction(&db, &user, body.post_uuid, reaction).await.map_err(failed)?;
            Ok(Response::new(post_to(PostMapper::json(post))))
        })
        .await
    }

    async fn remove_reaction(&self, request: Request<RemoveReactionRequest>) -> Result<Response<Post>, Status> {
        enforce(&content_limiter(), &request)?;
        let (db, post_uuid) = (self.db(), request.get_ref().post_uuid.clone());
        with_caller(&request, |user| async move {
            let post = PostUseCase::remove_reaction(&db, post_uuid, user.person_id, user.person_uuid)
                .await
                .map_err(failed)?;
            Ok(Response::new(post_to(PostMapper::json(post))))
        })
        .await
    }
}

#[tonic::async_trait]
impl FeedService for Services {
    async fn get_feed(&self, request: Request<GetFeedRequest>) -> Result<Response<FeedResponse>, Status> {
        let (db, page) = (self.db(), request.get_ref().page);
        let posts = with_caller(&request, |user| async move {
            PostUseCase::get_feed(&db, user.person_id, user.person_uuid, page).await.map_err(failed)
        })
        .await?;
        feed_response(posts).await
    }

    async fn get_feed_by_author(&self, request: Request<GetFeedByAuthorRequest>) -> Result<Response<FeedResponse>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
        let posts = with_caller(&request, |_| async move {
            PostUseCase::get_business_feed(&db, body.author_uuid, body.page).await.map_err(failed)
        })
        .await?;
        feed_response(posts).await
    }
}

#[tonic::async_trait]
impl NotificationService for Services {
    async fn list_notifications(&self, request: Request<ListNotificationsRequest>) -> Result<Response<ListNotificationsResponse>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let limit = if body.limit == 0 { 50 } else { i64::from(body.limit).clamp(1, 100) };
            let notifications =
                MentionNotificationUseCase::list_notifications(&db, &user.person_uuid, body.unread_only, limit)
                    .await
                    .map_err(failed)?;
            Ok(Response::new(ListNotificationsResponse {
                notifications: notifications.into_iter().map(|n| notification_to(NotificationMapper::json(n))).collect(),
            }))
        })
        .await
    }

    async fn mark_notification_read(&self, request: Request<MarkNotificationReadRequest>) -> Result<Response<MarkNotificationReadResponse>, Status> {
        let (db, key) = (self.db(), request.get_ref().idempotency_key.clone());
        with_caller(&request, |user| async move {
            if MentionNotificationUseCase::mark_as_read(&db, &user.person_uuid, &key).await.map_err(failed)? {
                Ok(Response::new(MarkNotificationReadResponse { read: true }))
            } else {
                // Same outcome as REST (400): the key is not one of the caller's notifications.
                Err(Status::invalid_argument("notification not found for this person"))
            }
        })
        .await
    }
}

#[tonic::async_trait]
impl EvolutionCheckInService for Services {
    async fn add_evolution_check_in(&self, request: Request<AddEvolutionCheckInRequest>) -> Result<Response<EvolutionCheckIn>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let json = check_in_from(body, &user.person_uuid)?;
            let use_case = EvolutionCheckInUseCase::new(EvolutionCheckInGateway::new(&db), ConsentGateway);
            let saved = use_case
                .add(EvolutionCheckInMapper::domain(json), &user.person_uuid)
                .await
                .map_err(failed)?;
            Ok(Response::new(check_in_to(EvolutionCheckInMapper::json(saved))))
        })
        .await
    }

    async fn list_evolution_check_ins(&self, request: Request<ListEvolutionCheckInsRequest>) -> Result<Response<ListEvolutionCheckInsResponse>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let end = optional_text_to_date(body.end_date.as_deref())?.unwrap_or_else(|| Utc::now().naive_utc());
            let start = optional_text_to_date(body.start_date.as_deref())?.unwrap_or_else(|| end - Duration::days(7));
            let use_case = EvolutionCheckInUseCase::new(EvolutionCheckInGateway::new(&db), ConsentGateway);
            let found = use_case
                .find_all_by_owner(
                    user.person_uuid,
                    opt_naive_to_bson_datetime(start).unwrap(),
                    opt_naive_to_bson_datetime(end).unwrap(),
                )
                .await;
            Ok(Response::new(ListEvolutionCheckInsResponse {
                check_ins: found.into_iter().map(|c| check_in_to(EvolutionCheckInMapper::json(c))).collect(),
            }))
        })
        .await
    }
}

#[tonic::async_trait]
impl PushDeviceService for Services {
    async fn register_push_device(&self, request: Request<RegisterPushDeviceRequest>) -> Result<Response<RegisterPushDeviceResponse>, Status> {
        let (db, body) = (self.db(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            PushDeviceUseCase::register(&db, &user.person_uuid, &body.device_uuid, &body.platform, &body.registration_token)
                .await
                .map_err(failed)?;
            Ok(Response::new(RegisterPushDeviceResponse {}))
        })
        .await
    }

    async fn remove_push_device(&self, request: Request<RemovePushDeviceRequest>) -> Result<Response<RemovePushDeviceResponse>, Status> {
        let (db, device) = (self.db(), request.get_ref().device_uuid.clone());
        with_caller(&request, |user| async move {
            PushDeviceUseCase::remove(&db, &user.person_uuid, &device).await.map_err(failed)?;
            Ok(Response::new(RemovePushDeviceResponse {}))
        })
        .await
    }
}
