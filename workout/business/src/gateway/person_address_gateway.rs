use crate::commons::entity_mapper::EntityMapper;
use crate::domain::person_address::{PersonAddress, PersonAddressEntityMapper};
use entity::person_address;
use entity::person_address::{ActiveModel, Model};
use entity::prelude::PersonAddress as PersonAddressQuery;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbBackend, DbConn, DbErr, EntityTrait, QueryFilter, Statement,
    UpdateResult,
};

pub struct PersonAddressGateway {}

impl PersonAddressGateway {
    pub async fn get_current_address(
        db: &DbConn,
        person_id: i32,
        current: bool,
    ) -> Result<Option<Model>, DbErr> {
        PersonAddressQuery::find()
            .filter(person_address::Column::PersonId.eq(person_id))
            .filter(person_address::Column::Current.eq(current))
            .one(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        PersonAddressQuery::find()
            .filter(person_address::Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn find_all_by_person_id(db: &DbConn, person_id: i32) -> Vec<Model> {
        PersonAddressQuery::find()
            .filter(person_address::Column::PersonId.eq(person_id))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn persist(db: &DbConn, domain: PersonAddress) -> Result<ActiveModel, DbErr> {
        let active_model = PersonAddressEntityMapper::build_active_model(domain);
        active_model.save(db).await
    }

    pub async fn find_all_within_radius(
        db: &DbConn,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<Vec<person_address::Model>, DbErr> {
        PersonAddressQuery::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"SELECT pa.*
                   FROM person_address AS pa
                   WHERE pa.current = TRUE
                     AND pa.location IS NOT NULL
                     AND ST_DWithin(
                         pa.location,
                         ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography,
                         $3
                     )"#,
                [
                    longitude.into(),
                    latitude.into(),
                    (radius_km * 1_000.0).into(),
                ],
            ))
            .all(db)
            .await
    }

    pub async fn delete(db: &DbConn, person_address_id: i32) -> Result<(), DbErr> {
        let address = PersonAddressQuery::find()
            .filter(person_address::Column::Id.eq(person_address_id))
            .one(db)
            .await?;

        if let Some(address) = address {
            let active_model: person_address::ActiveModel = address.into();
            active_model.delete(db).await?;
        }

        Ok(())
    }

    pub async fn set_all_addresses_not_current(
        db: &DbConn,
        person_id: i32,
    ) -> Result<UpdateResult, DbErr> {
        PersonAddressQuery::update_many()
            .col_expr(
                person_address::Column::Current,
                sea_orm::sea_query::Expr::value(false),
            )
            .filter(person_address::Column::PersonId.eq(person_id))
            .exec(db)
            .await
    }
}
