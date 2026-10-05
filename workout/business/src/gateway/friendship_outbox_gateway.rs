use crate::commons::functions::parse_uuid;
use crate::domain::business_error::BusinessError;
use crate::domain::friendship_outbox_event::FriendshipOutboxEvent;
use chrono::Utc;
use entity::friendship_notification_outbox_entity as outbox;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

pub struct FriendshipOutboxGateway;

impl FriendshipOutboxGateway {
    pub async fn find_pending<C: ConnectionTrait>(
        db: &C,
        limit: u64,
    ) -> Result<Vec<outbox::Model>, BusinessError> {
        outbox::Entity::find()
            .filter(outbox::Column::PublishedAt.is_null())
            .filter(outbox::Column::NextAttemptAt.lte(Utc::now()))
            .order_by_asc(outbox::Column::OccurredAt)
            .order_by_asc(outbox::Column::Id)
            .limit(limit)
            .all(db)
            .await
            .map_err(|error| {
                BusinessError::new(format!("Failed to load pending friendship events: {error}"))
            })
    }

    pub async fn has_earlier_unpublished<C: ConnectionTrait>(
        db: &C,
        event: &outbox::Model,
    ) -> Result<bool, BusinessError> {
        let earlier = outbox::Entity::find()
            .filter(outbox::Column::FriendshipUuid.eq(event.friendship_uuid))
            .filter(outbox::Column::PublishedAt.is_null())
            .filter(
                Condition::any()
                    .add(outbox::Column::OccurredAt.lt(event.occurred_at))
                    .add(
                        Condition::all()
                            .add(outbox::Column::OccurredAt.eq(event.occurred_at))
                            .add(outbox::Column::Id.lt(event.id)),
                    ),
            )
            .one(db)
            .await
            .map_err(|error| {
                BusinessError::new(format!("Failed to check friendship event order: {error}"))
            })?;
        Ok(earlier.is_some())
    }

    pub async fn persist<C: ConnectionTrait>(
        db: &C,
        event: FriendshipOutboxEvent,
    ) -> Result<(), BusinessError> {
        let now = Utc::now();
        outbox::ActiveModel {
            event_uuid: Set(parse_uuid(&event.event_uuid).map_err(Self::map_uuid_error)?),
            friendship_uuid: Set(parse_uuid(&event.friendship_uuid).map_err(Self::map_uuid_error)?),
            event_type: Set(event.event_type),
            actor_person_uuid: Set(
                parse_uuid(&event.actor_person_uuid).map_err(Self::map_uuid_error)?
            ),
            recipient_person_uuid: Set(
                parse_uuid(&event.recipient_person_uuid).map_err(Self::map_uuid_error)?
            ),
            occurred_at: Set(event.occurred_at),
            attempt_count: Set(0),
            next_attempt_at: Set(now),
            last_error: Set(None),
            published_at: Set(None),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|error| {
            BusinessError::new(format!("Failed to persist friendship event: {error}"))
        })?;

        Ok(())
    }

    pub async fn mark_published<C: ConnectionTrait>(
        db: &C,
        event: outbox::Model,
    ) -> Result<(), BusinessError> {
        let mut active: outbox::ActiveModel = event.into();
        active.published_at = Set(Some(Utc::now()));
        active.last_error = Set(None);
        active.update(db).await.map_err(|error| {
            BusinessError::new(format!(
                "Failed to mark friendship event published: {error}"
            ))
        })?;
        Ok(())
    }

    pub async fn mark_failed<C: ConnectionTrait>(
        db: &C,
        event: outbox::Model,
        error: &str,
    ) -> Result<i32, BusinessError> {
        let attempts = event.attempt_count + 1;
        let delay_seconds = Self::retry_delay_seconds(attempts);
        let mut active: outbox::ActiveModel = event.into();
        active.attempt_count = Set(attempts);
        active.next_attempt_at = Set(Utc::now() + chrono::Duration::seconds(delay_seconds));
        active.last_error = Set(Some(error.to_string()));
        active.update(db).await.map_err(|db_error| {
            BusinessError::new(format!(
                "Failed to schedule friendship event retry: {db_error}"
            ))
        })?;
        Ok(attempts)
    }

    pub fn retry_delay_seconds(attempts: i32) -> i64 {
        2_i64.pow(attempts.clamp(1, 8) as u32).min(300)
    }

    fn map_uuid_error(error: impl std::fmt::Display) -> BusinessError {
        BusinessError::new(format!("Invalid friendship event UUID: {error}"))
    }
}

#[cfg(test)]
#[path = "../tests/friendship_outbox_gateway_unit_test.rs"]
mod tests;
