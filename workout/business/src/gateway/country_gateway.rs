use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::country::{Country, CountryEntityMapper};
use entity::prelude::CountryEntity as CountryQuery;
use sea_orm::{DbConn, DbErr, EntityTrait};

#[allow(async_fn_in_trait)]
pub trait CountryGatewayPort: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Country>, BusinessError>;
}

pub struct CountryGateway {
    db: DbConn,
}

impl CountryGateway {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }

    pub async fn find_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<entity::country_entity::CountryEntity>, DbErr> {
        CountryQuery::find_by_id(id).one(db).await
    }
}

impl CountryGatewayPort for CountryGateway {
    async fn find_all(&self) -> Result<Vec<Country>, BusinessError> {
        let models = CountryQuery::find().all(&self.db).await.map_err(|error| {
            log::error!("Failed to get countries: {error}");
            BusinessError::new("Failed to get countries".to_string())
        })?;
        Ok(CountryEntityMapper::from_models(models))
    }
}
