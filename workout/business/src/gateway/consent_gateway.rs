use chrono::{DateTime, Utc};
use entity::consent_entity as consent;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};

pub struct ConsentGateway;

impl ConsentGateway {
    pub async fn accept(
        db: &DbConn,
        person_id: i32,
        document: &str,
        version: &str,
        ip: &str,
    ) -> Result<consent::Model, DbErr> {
        let model = consent::ActiveModel {
            person_id: Set(person_id),
            document: Set(document.to_string()),
            version: Set(version.to_string()),
            accepted_at: Set(Utc::now()),
            ip: Set(ip.to_string()),
            revoked_at: Set(None),
            ..Default::default()
        };
        model.insert(db).await
    }

    pub async fn list(db: &DbConn, person_id: i32) -> Result<Vec<consent::Model>, DbErr> {
        consent::Entity::find()
            .filter(consent::Column::PersonId.eq(person_id))
            .all(db)
            .await
    }

    pub async fn has_active(
        db: &DbConn,
        person_id: i32,
        document: &str,
        version: &str,
    ) -> Result<bool, DbErr> {
        Ok(consent::Entity::find()
            .filter(consent::Column::PersonId.eq(person_id))
            .filter(consent::Column::Document.eq(document))
            .filter(consent::Column::Version.eq(version))
            .filter(consent::Column::RevokedAt.is_null())
            .one(db)
            .await?
            .is_some())
    }

    pub async fn revoke_active(
        db: &DbConn,
        person_id: i32,
        document: &str,
        at: DateTime<Utc>,
    ) -> Result<u64, DbErr> {
        let result = consent::Entity::update_many()
            .col_expr(
                consent::Column::RevokedAt,
                sea_orm::sea_query::Expr::value(Some(at)),
            )
            .filter(consent::Column::PersonId.eq(person_id))
            .filter(consent::Column::Document.eq(document))
            .filter(consent::Column::RevokedAt.is_null())
            .exec(db)
            .await?;
        Ok(result.rows_affected)
    }
}
