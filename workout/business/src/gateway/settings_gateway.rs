use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::parse_uuid;
use crate::commons::gateway::Gateway;
use crate::domain::settings::{Settings, SettingsEntityMapper};
use entity::prelude::SettingsEntity as SettingsQuery;
use entity::settings_entity::{ActiveModel, SettingsEntity};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

pub struct SettingsGateway {
    db: DbConn,
}

impl SettingsGateway {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }

    pub async fn find_by_owner_ids(
        &self,
        owner_id: Option<i32>,
        owner_uuid: Option<String>,
    ) -> Result<Option<SettingsEntity>, DbErr> {
        let mut query = SettingsQuery::find();
        if let Some(id) = owner_id {
            query = query.filter(entity::settings_entity::Column::PersonId.eq(id));
        }
        if let Some(uuid) = owner_uuid {
            let uuid = parse_uuid(&uuid).map_err(|e| DbErr::Type(e.to_string()))?;
            query = query.filter(entity::settings_entity::Column::PersonUuid.eq(uuid));
        }
        query.one(&self.db).await
    }

    pub async fn find_by_owner_id(&self, owner_id: i32) -> Result<Option<SettingsEntity>, DbErr> {
        let mut query = SettingsQuery::find();
        query = query.filter(entity::settings_entity::Column::PersonId.eq(owner_id));
        query.one(&self.db).await
    }
}

#[async_trait]
impl Gateway<Settings, SettingsEntity, ActiveModel> for SettingsGateway {
    async fn persist(&self, domain: Settings) -> Result<ActiveModel, DbErr> {
        let active_model = SettingsEntityMapper::build_active_model(domain);
        active_model.save(&self.db).await
    }

    async fn delete(&self, domain: Settings) -> Result<(), DbErr> {
        let active_model = SettingsEntityMapper::build_active_model(domain);
        active_model.delete(&self.db).await.map(|_| ())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<SettingsEntity>, DbErr> {
        SettingsQuery::find()
            .filter(entity::settings_entity::Column::Id.eq(id))
            .one(&self.db)
            .await
    }

    async fn find_by_uuid(&self, uuid: String) -> Result<Option<SettingsEntity>, DbErr> {
        let uuid = parse_uuid(&uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        SettingsQuery::find()
            .filter(entity::settings_entity::Column::Uuid.eq(uuid))
            .one(&self.db)
            .await
    }

    async fn find_all(&self) -> Result<Vec<SettingsEntity>, DbErr> {
        SettingsQuery::find().all(&self.db).await
    }
}
