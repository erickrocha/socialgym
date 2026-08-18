use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::string_to_uuid;
use crate::commons::gateway::Gateway;
use crate::domain::settings::{Settings, SettingsEntityMapper};
use entity::prelude::Settings as SettingsQuery;
use entity::settings::{ActiveModel, Model};
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
    ) -> Result<Option<Model>, DbErr> {
        let mut query = SettingsQuery::find();
        if let Some(id) = owner_id {
            query = query.filter(entity::settings::Column::PersonId.eq(id));
        }
        if let Some(uuid) = owner_uuid {
            query = query
                .filter(entity::settings::Column::PersonUuid.eq(string_to_uuid(uuid.as_str())));
        }
        query.one(&self.db).await
    }

    pub async fn find_by_owner_id(&self, owner_id: i32) -> Result<Option<Model>, DbErr> {
        let mut query = SettingsQuery::find();
        query = query.filter(entity::settings::Column::PersonId.eq(owner_id));
        query.one(&self.db).await
    }
}

#[async_trait]
impl Gateway<Settings, Model, ActiveModel> for SettingsGateway {
    async fn persist(&self, domain: Settings) -> Result<ActiveModel, DbErr> {
        let active_model = SettingsEntityMapper::build_active_model(domain);
        active_model.save(&self.db).await
    }

    async fn delete(&self, domain: Settings) -> Result<(), DbErr> {
        let active_model = SettingsEntityMapper::build_active_model(domain);
        active_model.delete(&self.db).await.map(|_| ())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        SettingsQuery::find()
            .filter(entity::settings::Column::Id.eq(id))
            .one(&self.db)
            .await
    }

    async fn find_by_uuid(&self, uuid: String) -> Result<Option<Model>, DbErr> {
        SettingsQuery::find()
            .filter(entity::settings::Column::Uuid.eq(string_to_uuid(uuid.as_str())))
            .one(&self.db)
            .await
    }

    async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        SettingsQuery::find().all(&self.db).await
    }
}
