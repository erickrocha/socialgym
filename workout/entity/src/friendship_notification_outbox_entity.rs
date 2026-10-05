use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

pub type FriendshipNotificationOutboxEntity = Model;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "friendship_notification_outbox")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub event_uuid: Uuid,
    pub friendship_uuid: Uuid,
    pub event_type: String,
    pub actor_person_uuid: Uuid,
    pub recipient_person_uuid: Uuid,
    pub occurred_at: DateTimeUtc,
    pub attempt_count: i32,
    pub next_attempt_at: DateTimeUtc,
    pub last_error: Option<String>,
    pub published_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            if matches!(&self.event_uuid, sea_orm::ActiveValue::NotSet) {
                self.event_uuid = Set(Uuid::new_v4());
            }
            let now = Utc::now();
            if matches!(&self.occurred_at, sea_orm::ActiveValue::NotSet) {
                self.occurred_at = Set(now);
            }
            if matches!(&self.next_attempt_at, sea_orm::ActiveValue::NotSet) {
                self.next_attempt_at = Set(now);
            }
            self.attempt_count = Set(0);
            self.created_at = Set(now);
        }
        self.updated_at = Set(Utc::now());
        Ok(self)
    }
}
