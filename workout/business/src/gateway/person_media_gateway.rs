use crate::commons::entity_mapper::EntityMapper;
use crate::domain::person_media::{PersonMedia, PersonMediaEntityMapper};
use entity::person_media::{Column, Entity, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

pub struct PersonMediaGateway {}

impl PersonMediaGateway {
    pub async fn save(
        db: &DbConn,
        media: PersonMedia,
    ) -> Result<entity::person_media::ActiveModel, DbErr> {
        let active_model = PersonMediaEntityMapper::build_active_model(media);
        active_model.save(db).await
    }

    pub async fn find_by_person_and_album(
        db: &DbConn,
        person_id: i32,
        album: String,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::PersonId.eq(person_id))
            .filter(Column::Album.eq(album))
            .all(db)
            .await
    }

    pub async fn find_by_s3_key(db: &DbConn, s3_key: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::S3Key.eq(s3_key))
            .one(db)
            .await
    }

    pub async fn delete_by_s3_key(db: &DbConn, s3_key: &str) -> Result<(), DbErr> {
        Entity::delete_many()
            .filter(Column::S3Key.eq(s3_key))
            .exec(db)
            .await?;
        Ok(())
    }
}
