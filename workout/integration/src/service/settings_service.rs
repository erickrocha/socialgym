use crate::proto::settings::settings_service_server::SettingsService;
use crate::proto::settings::{Setting, SettingIdRequest, SettingOwnerIdRequest};
use crate::service::validate_uuid;
use business::domain::enums::Position;
use business::domain::settings::Settings;
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::setings_use_case::SettingsUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcSettingService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcSettingService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    fn domain_to_proto(settings: Settings) -> Setting {
        Setting {
            id: settings.id.unwrap_or(0),
            uuid: settings.uuid.unwrap_or_default(),
            owner_id: settings.person_id,
            owner_uuid: settings.person_uuid,
            language: settings.language,
            theme: settings.theme,
            notifications_enabled: settings.notifications_enabled,
            context_menu_position: settings.context_menu_position.to_string(),
            home_page: settings.home_page,
            created_at: settings
                .created_at
                .map(|dt| dt.to_string())
                .unwrap_or_default(),
            updated_at: settings
                .updated_at
                .map(|dt| dt.to_string())
                .unwrap_or_default(),
        }
    }

    fn proto_to_domain(setting: Setting) -> Settings {
        Settings {
            id: if setting.id > 0 {
                Some(setting.id)
            } else {
                None
            },
            uuid: if setting.uuid.is_empty() {
                None
            } else {
                Some(setting.uuid)
            },
            person_id: setting.owner_id,
            person_uuid: setting.owner_uuid,
            language: setting.language,
            theme: setting.theme,
            notifications_enabled: setting.notifications_enabled,
            context_menu_position: Position::from_string(&setting.context_menu_position),
            home_page: setting.home_page,
            created_at: None, // Will be set by database
            updated_at: None, // Will be set by database
        }
    }
}

#[tonic::async_trait]
impl SettingsService for GrpcSettingService {
    async fn get_by_id(
        &self,
        request: Request<SettingIdRequest>,
    ) -> Result<Response<Setting>, Status> {
        let req = request.into_inner();
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        let result = use_case.get_by_id(req.id).await;
        match result {
            Ok(settings) => {
                let proto_setting = Self::domain_to_proto(settings);
                Ok(Response::new(proto_setting))
            }
            Err(_e) => Err(Status::not_found("Settings not found")),
        }
    }

    async fn persist_settings(
        &self,
        request: Request<Setting>,
    ) -> Result<Response<Setting>, Status> {
        let setting = request.into_inner();
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        let domain_settings = Self::proto_to_domain(setting);
        let result = use_case.persist(domain_settings).await;

        match result {
            Ok(settings) => {
                let proto_setting = Self::domain_to_proto(settings);
                Ok(Response::new(proto_setting))
            }
            Err(_e) => Err(Status::internal("Error persisting settings")),
        }
    }

    async fn get_by_uuid(
        &self,
        request: Request<SettingIdRequest>,
    ) -> Result<Response<Setting>, Status> {
        let req = request.into_inner();
        validate_uuid(&req.uuid, "uuid")?;
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        let result = use_case.get_by_uuid(req.uuid).await;
        match result {
            Ok(settings) => {
                let proto_setting = Self::domain_to_proto(settings);
                Ok(Response::new(proto_setting))
            }
            Err(_e) => Err(Status::not_found("Settings not found")),
        }
    }

    async fn get_by_owner_ids(
        &self,
        request: Request<SettingOwnerIdRequest>,
    ) -> Result<Response<Setting>, Status> {
        let req = request.into_inner();
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        let result = use_case.get_by_owner_id(req.owner_id).await;
        match result {
            Ok(settings) => {
                let proto_setting = Self::domain_to_proto(settings);
                Ok(Response::new(proto_setting))
            }
            Err(_e) => Err(Status::not_found("Settings not found")),
        }
    }
}
