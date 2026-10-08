use crate::infrastructure::mapper::{CountryMapper, Mapper, SettingsMapper};
use crate::infrastructure::utils::{business_status, require_actor, validate_uuid};
use business::commons::authorization::ensure_owns;
use crate::proto::resource::resource_request::Identifier;
use crate::proto::resource::resource_service_server::ResourceService;
use crate::proto::resource::{ResourceRequest, ResourceResponse};
use business::gateway::country_gateway::CountryGateway;
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::resource_use_case::ResourceUseCase;
use business::use_cases::setings_use_case::SettingsUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcResourceService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcResourceService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[tonic::async_trait]
impl ResourceService for GrpcResourceService {
    async fn get_resource(
        &self,
        request: Request<ResourceRequest>,
    ) -> Result<Response<ResourceResponse>, Status> {
        // The settings that come with the countries are the caller's own, as over REST: another person's id
        // or uuid is refused, not served (C-010 W22).
        let actor = require_actor(&request)?;
        let req = request.into_inner();
        match &req.identifier {
            Some(Identifier::UserId(id)) => ensure_owns(*id, actor.person_id).map_err(business_status)?,
            Some(Identifier::OwnerUuid(uuid)) => {
                validate_uuid(uuid, "owner_uuid")?;
                if *uuid != actor.person_uuid {
                    return Err(Status::permission_denied("Not the owner of this resource"));
                }
            }
            None => {}
        }
        let resource_use_case = ResourceUseCase::new(CountryGateway::new((*self.conn).clone()));
        let countries_response = resource_use_case
            .get_countries()
            .await
            .map_err(business_status)?;
        match req.identifier {
            Some(Identifier::UserId(id)) => {
                let setting_use_case =
                    SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));
                let settings_response = setting_use_case
                    .get_by_owner_id(id)
                    .await
                    .map_err(business_status)?;
                let countries = CountryMapper::response_vec(countries_response);
                let setting = SettingsMapper::response_option(Some(settings_response));
                Ok(Response::new(ResourceResponse { countries, setting }))
            }
            Some(Identifier::OwnerUuid(uuid)) => {
                let setting_use_case =
                    SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));
                let settings_response = setting_use_case
                    .get_by_owner_uuid(uuid)
                    .await
                    .map_err(business_status)?;
                let countries = CountryMapper::response_vec(countries_response);
                let setting = SettingsMapper::response_option(Some(settings_response));
                Ok(Response::new(ResourceResponse { countries, setting }))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }
}
