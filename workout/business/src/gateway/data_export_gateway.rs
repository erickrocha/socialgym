use chrono::{DateTime, Utc};
use entity::data_export_entity as export;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};

pub struct DataExportGateway;
impl DataExportGateway {
    pub async fn create(db: &DbConn, person_id: i32) -> Result<export::Model, DbErr> {
        export::ActiveModel {
            person_id: Set(person_id),
            status: Set("pending".into()),
            object_key: Set(None),
            error: Set(None),
            expires_at: Set(None),
            ..Default::default()
        }
        .insert(db)
        .await
    }
    pub async fn list_for_person(db: &DbConn, person_id: i32) -> Result<Vec<export::Model>, DbErr> {
        export::Entity::find()
            .filter(export::Column::PersonId.eq(person_id))
            .order_by_desc(export::Column::CreatedAt)
            .all(db)
            .await
    }
    pub async fn find_owned(
        db: &DbConn,
        uuid: uuid::Uuid,
        person_id: i32,
    ) -> Result<Option<export::Model>, DbErr> {
        export::Entity::find()
            .filter(export::Column::Uuid.eq(uuid))
            .filter(export::Column::PersonId.eq(person_id))
            .one(db)
            .await
    }
    pub async fn find_pending(db: &DbConn, limit: u64) -> Result<Vec<export::Model>, DbErr> {
        export::Entity::find()
            .filter(export::Column::Status.eq("pending"))
            .order_by_asc(export::Column::CreatedAt)
            .limit(limit)
            .all(db)
            .await
    }
    pub async fn find_expired(
        db: &DbConn,
        now: DateTime<Utc>,
        limit: u64,
    ) -> Result<Vec<export::Model>, DbErr> {
        export::Entity::find()
            .filter(export::Column::Status.eq("ready"))
            .filter(export::Column::ExpiresAt.lte(now))
            .limit(limit)
            .all(db)
            .await
    }
    pub async fn set_processing(db: &DbConn, id: i32) -> Result<(), DbErr> {
        export::ActiveModel {
            id: Set(id),
            status: Set("processing".into()),
            error: Set(None),
            ..Default::default()
        }
        .update(db)
        .await
        .map(|_| ())
    }
    pub async fn set_ready(
        db: &DbConn,
        id: i32,
        key: String,
        expires_at: DateTime<Utc>,
    ) -> Result<(), DbErr> {
        export::ActiveModel {
            id: Set(id),
            status: Set("ready".into()),
            object_key: Set(Some(key)),
            expires_at: Set(Some(expires_at)),
            error: Set(None),
            ..Default::default()
        }
        .update(db)
        .await
        .map(|_| ())
    }
    pub async fn set_failed(db: &DbConn, id: i32, error: String) -> Result<(), DbErr> {
        export::ActiveModel {
            id: Set(id),
            status: Set("failed".into()),
            error: Set(Some(error)),
            ..Default::default()
        }
        .update(db)
        .await
        .map(|_| ())
    }
    pub async fn set_expired(db: &DbConn, id: i32) -> Result<(), DbErr> {
        export::ActiveModel {
            id: Set(id),
            status: Set("expired".into()),
            object_key: Set(None),
            ..Default::default()
        }
        .update(db)
        .await
        .map(|_| ())
    }
}
