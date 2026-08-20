use std::sync::Arc;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::resource_use_case::ResourceUseCase;
use business::use_cases::setings_use_case::SettingsUseCase;
use crate::infrastructure::mapper::{CountryMapper, Mapper, SettingsMapper};
use crate::infrastructure::utils::business_status;
use crate::proto::resource::resource_service_server::ResourceService;
use crate::proto::resource::{ResourceRequest, ResourceResponse};
use crate::proto::resource::resource_request::Identifier;

pub struct GrpcResourceService{
    conn: Arc<DatabaseConnection>
}

impl GrpcResourceService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[tonic::async_trait]
impl ResourceService for GrpcResourceService {
    async fn get_resource(&self, request: Request<ResourceRequest>) -> Result<Response<ResourceResponse>, Status> {
        let req = request.into_inner();
        let countries_response = ResourceUseCase::get_countries(&self.conn).await.map_err(business_status)?;
        match req.identifier {
            Some(Identifier::UserId(id)) => {
                let setting_use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));
                let settings_response = setting_use_case.get_by_owner_id(id).await.map_err(business_status)?;
                let countries = CountryMapper::response_vec(countries_response);
                let setting = SettingsMapper::response_option(Some(settings_response));
                Ok(Response::new(ResourceResponse{
                    countries,
                    setting
                }))
            }
            Some(Identifier::OwnerUuid(uuid)) => {
                let setting_use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));
                let settings_response = setting_use_case.get_by_owner_uuid(uuid).await.map_err(business_status)?;
                let countries = CountryMapper::response_vec(countries_response);
                let setting = SettingsMapper::response_option(Some(settings_response));
                Ok(Response::new(ResourceResponse{
                    countries,
                    setting
                }))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }
}