//! NotificationService: the notifications of the caller (`/timeline/api/notifications`).
use crate::infrastructure::mapper::NotificationMapper;
use crate::infrastructure::utils::{business_status, with_caller};
use crate::proto::timeline::notification_service_server::NotificationService;
use crate::proto::timeline::*;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcNotificationService {
    database: Arc<Database>,
}

impl GrpcNotificationService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl NotificationService for GrpcNotificationService {
    async fn list_notifications(
        &self,
        request: Request<ListNotificationsRequest>,
    ) -> Result<Response<ListNotificationsResponse>, Status> {
        let (db, body) = (self.database.clone(), *request.get_ref());
        with_caller(&request, |user| async move {
            let limit = if body.limit == 0 {
                50
            } else {
                i64::from(body.limit).clamp(1, 100)
            };
            let notifications = MentionNotificationUseCase::list_notifications(
                &db,
                &user.person_uuid,
                body.unread_only,
                limit,
            )
            .await
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(ListNotificationsResponse {
                notifications: notifications
                    .into_iter()
                    .map(NotificationMapper::proto)
                    .collect(),
            }))
        })
        .await
    }

    async fn mark_notification_read(
        &self,
        request: Request<MarkNotificationReadRequest>,
    ) -> Result<Response<MarkNotificationReadResponse>, Status> {
        let (db, key) = (
            self.database.clone(),
            request.get_ref().idempotency_key.clone(),
        );
        with_caller(&request, |user| async move {
            if MentionNotificationUseCase::mark_as_read(&db, &user.person_uuid, &key)
                .await
                .map_err(|e| business_status(&e))?
            {
                Ok(Response::new(MarkNotificationReadResponse { read: true }))
            } else {
                // Same outcome as REST (400): the key is not one of the caller's notifications.
                Err(Status::invalid_argument(
                    "notification not found for this person",
                ))
            }
        })
        .await
    }
}
