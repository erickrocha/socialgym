use std::sync::Arc;

use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use business::use_cases::business_profile_address_use_case::BusinessProfileAddressUseCase;
use crate::infrastructure::mapper::{BusinessProfileAddressMapper, BusinessProfileMapper, Mapper};
use crate::proto::business_profile::business_profile_service_server::BusinessProfileService;
use crate::proto::business_profile::{BusinessProfile, BusinessProfileRequestId, BusinessProfileRequestOwnerId, BusinessProfilesResponse, RemoveBusinessProfileAddressRequest, RemoveBusinessProfileAddressResponse};
use crate::proto::business_profile_address::BusinessProfileAddress;
use crate::service::validate_uuid;

pub struct GrpcBusinessProfileService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcBusinessProfileService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[tonic::async_trait]
impl BusinessProfileService for GrpcBusinessProfileService {
    async fn get_business_profile_by_id(&self,request: Request<BusinessProfileRequestId>) -> Result<Response<BusinessProfile>, Status> {
        log::info!("Getting Business Profile by id or Uuid");
        let payload = request.into_inner();

        if payload.id <= 0 && payload.uuid.is_empty() {
            return Err(Status::invalid_argument("either id or uuid must be informed"));
        }

        if !payload.uuid.is_empty() {
            validate_uuid(&payload.uuid, "uuid")?;
            let business_profile = BusinessProfileUseCase::get_by_uuid(&self.conn, payload.uuid)
                .await
                .ok_or_else(|| Status::internal("Business profile not found"))?;
            let grpc_profile = BusinessProfileMapper::response(business_profile);
            Ok(Response::new(grpc_profile))
        }else {
            let profile = BusinessProfileUseCase::get_by_id(&self.conn, payload.id)
                .await
                .ok_or_else(|| Status::internal("Business profile not found"))?;

            let grpc_profile = BusinessProfileMapper::response(profile);
            Ok(Response::new(grpc_profile))
        }
    }

    async fn get_business_profile_by_owner_id(&self, request: Request<BusinessProfileRequestOwnerId>) -> Result<Response<BusinessProfilesResponse>, Status> {
        log::info!("Getting Business Profile by owner id or owner Uuid");
        let payload = request.into_inner();

        if payload.owner_id <= 0 && payload.owner_uuid.is_empty() {
            return Err(Status::invalid_argument("either owner_id or owner_uuid must be informed"));
        }

        if !payload.owner_uuid.is_empty() {
            validate_uuid(&payload.owner_uuid, "owner_uuid")?;
            let business_profiles = BusinessProfileUseCase::get_by_owner_uuid(&self.conn, payload.owner_uuid)
                .await
                .map_err(|e| Status::internal(e.message))?;
            let grpc_profiles = BusinessProfileMapper::response_vec(business_profiles);
            Ok(Response::new(BusinessProfilesResponse { business_profiles: grpc_profiles }))
        } else {
            let business_profiles = BusinessProfileUseCase::get_by_owner_id(&self.conn, payload.owner_id)
                .await
                .map_err(|e| Status::internal(e.message))?;
            let grpc_profiles = BusinessProfileMapper::response_vec(business_profiles);
            Ok(Response::new(BusinessProfilesResponse { business_profiles: grpc_profiles }))
        }
    }

    async fn add_business_profile(&self, request: Request<BusinessProfile>) -> Result<Response<BusinessProfile>, Status> {
        log::info!("Adding Business Profile");
        let payload = request.into_inner();
        let domain_profile = BusinessProfileMapper::domain(payload);
        let added_profile = BusinessProfileUseCase::add(&self.conn, domain_profile)
            .await
            .map_err(|e| Status::internal(e.message))?;
        let grpc_profile = BusinessProfileMapper::response(added_profile);
        Ok(Response::new(grpc_profile))
    }

    async fn update_business_profile(&self, request: Request<BusinessProfile>) -> Result<Response<BusinessProfile>, Status> {
        log::info!("Updating Business Profile");
        let payload = request.into_inner();
        let domain_profile = BusinessProfileMapper::domain(payload);
        let updated_profile = BusinessProfileUseCase::update(&self.conn, domain_profile)
            .await
            .map_err(|e| Status::internal(e.message))?;
        let grpc_profile = BusinessProfileMapper::response(updated_profile);
        Ok(Response::new(grpc_profile))
    }

    async fn add_business_profile_address(&self, request: Request<BusinessProfileAddress>) -> Result<Response<BusinessProfileAddress>, Status> {
        log::info!("Adding Business Profile Address");
        let payload = request.into_inner();
        let domain_address = BusinessProfileAddressMapper::domain(payload);
        let added_address = BusinessProfileAddressUseCase::save(&self.conn, domain_address)
            .await
            .map_err(|e| Status::internal(e.message))?;
        let grpc_address = BusinessProfileAddressMapper::response(added_address);
        Ok(Response::new(grpc_address))
    }

    async fn update_business_profile_address(&self, request: Request<BusinessProfileAddress>) -> Result<Response<BusinessProfileAddress>, Status> {
        log::info!("Updating Business Profile Address");
        let payload = request.into_inner();
        let domain_address = BusinessProfileAddressMapper::domain(payload);
        let updated_address = BusinessProfileAddressUseCase::save(&self.conn, domain_address)
            .await
            .map_err(|e| Status::internal(e.message))?;
        let grpc_address = BusinessProfileAddressMapper::response(updated_address);
        Ok(Response::new(grpc_address))
    }

    async fn remove_business_profile_address(&self, request: Request<RemoveBusinessProfileAddressRequest>) -> Result<Response<RemoveBusinessProfileAddressResponse>, Status> {
        log::info!("Removing Business Profile Address");
        let payload = request.into_inner();
        BusinessProfileAddressUseCase::delete_by_id(&self.conn, payload.id)
            .await
            .map_err(|e| Status::internal(e.message))?;
        Ok(Response::new(RemoveBusinessProfileAddressResponse { success: true }))
    }
}
