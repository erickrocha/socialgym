use std::sync::Arc;

use crate::infrastructure::mapper::{BusinessProfileAddressMapper, BusinessProfileMapper, Mapper};
use crate::proto::business_profile::business_profile_service_server::BusinessProfileService;
use crate::proto::business_profile::{
    BusinessProfile, BusinessProfileImageUploadRequest, DeleteBusinessProfileRequest, DeleteBusinessProfileResponse,
    DiscoverBusinessProfilesRequest, GetActiveBusinessProfileRequest, BusinessProfileImageUploadResponse, BusinessProfileRequestId, BusinessProfileRequestOwnerId,
    BusinessProfilesResponse, RemoveBusinessProfileAddressRequest,
    RemoveBusinessProfileAddressResponse,
};
use crate::proto::business_profile_address::BusinessProfileAddress;

use business::domain::user::User;
use business::use_cases::business_profile_address_use_case::BusinessProfileAddressUseCase;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use crate::infrastructure::utils::{business_status, locale_of, localized_business_status, localized_status, require_active_profile, require_actor, require_person_id, validate_uuid};
use business::commons::i18n::ErrorKey;
use business::domain::enums::ProfileType;
use business::use_cases::common_use_case::choose_image_type;
use tonic::Code;

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
    async fn get_business_profile_by_id(
        &self,
        request: Request<BusinessProfileRequestId>,
    ) -> Result<Response<BusinessProfile>, Status> {
        let viewer = request.extensions().get::<User>().cloned();
        let payload = request.into_inner();

        if payload.id <= 0 && payload.uuid.is_empty() {
            return Err(Status::invalid_argument(
                "either id or uuid must be informed",
            ));
        }

        if !payload.uuid.is_empty() {
            validate_uuid(&payload.uuid, "uuid")?;
            let business_profile = BusinessProfileUseCase::get_by_uuid(&self.conn, payload.uuid)
                .await
                .ok_or_else(|| Status::not_found("Business profile not found"))?;
            let shown = BusinessProfileUseCase::present(&self.conn, business_profile, viewer.as_ref()).await;
            Ok(Response::new(BusinessProfileMapper::response(shown)))
        } else {
            let profile = BusinessProfileUseCase::get_by_id(&self.conn, payload.id)
                .await
                .ok_or_else(|| Status::not_found("Business profile not found"))?;

            let shown = BusinessProfileUseCase::present(&self.conn, profile, viewer.as_ref()).await;
            Ok(Response::new(BusinessProfileMapper::response(shown)))
        }
    }

    async fn get_business_profile_by_owner_id(
        &self,
        request: Request<BusinessProfileRequestOwnerId>,
    ) -> Result<Response<BusinessProfilesResponse>, Status> {
        let viewer = request.extensions().get::<User>().cloned();
        let payload = request.into_inner();
        if payload.owner_id <= 0 && payload.owner_uuid.is_empty() {
            return Err(Status::invalid_argument(
                "either owner_id or owner_uuid must be informed",
            ));
        }

        if !payload.owner_uuid.is_empty() {
            validate_uuid(&payload.owner_uuid, "owner_uuid")?;
            let business_profiles =
                BusinessProfileUseCase::get_by_owner_uuid(&self.conn, payload.owner_uuid)
                    .await
                    .map_err(|e| Status::internal(e.message))?;
            let shown = BusinessProfileUseCase::present_all(&self.conn, business_profiles, viewer.as_ref()).await;
            let grpc_profiles = BusinessProfileMapper::response_vec(shown);
            Ok(Response::new(BusinessProfilesResponse {
                business_profiles: grpc_profiles,
            }))
        } else {
            let business_profiles =
                BusinessProfileUseCase::get_by_owner_id(&self.conn, payload.owner_id)
                    .await
                    .map_err(|e| Status::internal(e.message))?;
            let shown = BusinessProfileUseCase::present_all(&self.conn, business_profiles, viewer.as_ref()).await;
            let grpc_profiles = BusinessProfileMapper::response_vec(shown);
            Ok(Response::new(BusinessProfilesResponse {
                business_profiles: grpc_profiles,
            }))
        }
    }

    async fn add_business_profile(
        &self,
        request: Request<BusinessProfile>,
    ) -> Result<Response<BusinessProfile>, Status> {
        let actor = require_actor(&request)?;
        let payload = request.into_inner();
        let domain_profile = BusinessProfileMapper::domain(payload);
        let added_profile = BusinessProfileUseCase::add(&self.conn, domain_profile, &actor)
            .await
            .map_err(business_status)?;
        let grpc_profile = BusinessProfileMapper::response(added_profile);
        Ok(Response::new(grpc_profile))
    }

    async fn update_business_profile(
        &self,
        request: Request<BusinessProfile>,
    ) -> Result<Response<BusinessProfile>, Status> {
        let actor = require_actor(&request)?;
        let payload = request.into_inner();
        let domain_profile = BusinessProfileMapper::domain(payload);
        let updated_profile = BusinessProfileUseCase::update(&self.conn, domain_profile, &actor)
            .await
            .map_err(business_status)?;
        let grpc_profile = BusinessProfileMapper::response(updated_profile);
        Ok(Response::new(grpc_profile))
    }

    async fn add_business_profile_address(
        &self,
        request: Request<BusinessProfileAddress>,
    ) -> Result<Response<BusinessProfileAddress>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();
        let (latitude, longitude) = (payload.latitude, payload.longitude);
        let domain_address = BusinessProfileAddressMapper::domain(payload);
        let added_address = BusinessProfileAddressUseCase::save(
            &self.conn,
            domain_address,
            person_id,
            latitude,
            longitude,
        )
        .await
        .map_err(business_status)?;
        let grpc_address = BusinessProfileAddressMapper::response(added_address);
        Ok(Response::new(grpc_address))
    }

    async fn update_business_profile_address(
        &self,
        request: Request<BusinessProfileAddress>,
    ) -> Result<Response<BusinessProfileAddress>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();
        let (latitude, longitude) = (payload.latitude, payload.longitude);
        let domain_address = BusinessProfileAddressMapper::domain(payload);
        let updated_address = BusinessProfileAddressUseCase::save(
            &self.conn,
            domain_address,
            person_id,
            latitude,
            longitude,
        )
        .await
        .map_err(business_status)?;
        let grpc_address = BusinessProfileAddressMapper::response(updated_address);
        Ok(Response::new(grpc_address))
    }

    async fn remove_business_profile_address(
        &self,
        request: Request<RemoveBusinessProfileAddressRequest>,
    ) -> Result<Response<RemoveBusinessProfileAddressResponse>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();
        // By id or by uuid, as the two REST routes do; the owner check is the use case's.
        if payload.id > 0 {
            BusinessProfileAddressUseCase::delete_by_id(&self.conn, payload.id, person_id)
                .await
                .map_err(business_status)?;
        } else if !payload.uuid.is_empty() {
            validate_uuid(&payload.uuid, "uuid")?;
            BusinessProfileAddressUseCase::delete_by_uuid(&self.conn, payload.uuid, person_id)
                .await
                .map_err(business_status)?;
        } else {
            return Err(Status::invalid_argument("either id or uuid must be informed"));
        }
        Ok(Response::new(RemoveBusinessProfileAddressResponse {
            success: true,
        }))
    }

    async fn get_business_profile_image_upload_url(
        &self,
        request: Request<BusinessProfileImageUploadRequest>,
    ) -> Result<Response<BusinessProfileImageUploadResponse>, Status> {
        let locale = locale_of(&request);
        // The logo and the cover belong to the Active Business Profile of the token, which the
        // authentication layer loads (a person can only activate a profile they own).
        let profile = require_active_profile(&request)
            .ok_or_else(|| localized_status(Code::FailedPrecondition, ErrorKey::BusinessProfilePreSignedUrlNotGenerated, locale))?;
        let (Some(id), Some(uuid)) = (profile.id, profile.uuid.clone()) else {
            return Err(localized_status(Code::FailedPrecondition, ErrorKey::BusinessProfilePreSignedUrlNotGenerated, locale));
        };
        let payload = request.into_inner();
        let format = if payload.format.is_empty() { "jpg".to_string() } else { payload.format };
        let image_storage = BusinessProfileUseCase::upload_business_profile_image(
            &self.conn,
            id,
            uuid,
            choose_image_type(payload.image_type.as_str()),
            format,
        )
        .await
        .map_err(|_| localized_status(Code::InvalidArgument, ErrorKey::BusinessProfilePreSignedUrlNotGenerated, locale))?;
        Ok(Response::new(BusinessProfileImageUploadResponse {
            url: image_storage.url,
            object_key: image_storage.object_key,
            business_profile_id: id,
        }))
    }

    async fn get_active_business_profile(
        &self,
        request: Request<GetActiveBusinessProfileRequest>,
    ) -> Result<Response<BusinessProfile>, Status> {
        let locale = locale_of(&request);
        let active = require_active_profile(&request)
            .and_then(|profile| profile.id)
            .ok_or_else(|| localized_status(Code::FailedPrecondition, ErrorKey::BusinessProfileNotFound, locale))?;
        // The Active Business Profile is the caller's own, so the owner's full view applies.
        BusinessProfileUseCase::get_by_id(&self.conn, active)
            .await
            .map(|profile| Response::new(BusinessProfileMapper::response(profile)))
            .ok_or_else(|| localized_status(Code::NotFound, ErrorKey::BusinessProfileNotFound, locale))
    }

    async fn discover_business_profiles(
        &self,
        request: Request<DiscoverBusinessProfilesRequest>,
    ) -> Result<Response<BusinessProfilesResponse>, Status> {
        let locale = locale_of(&request);
        let viewer = request.extensions().get::<User>().cloned();
        let payload = request.into_inner();
        let business_type = match payload.business_type.as_deref() {
            Some("Professional") => Some(ProfileType::Professional),
            Some("Company") => Some(ProfileType::Company),
            _ => None,
        };
        let profiles = BusinessProfileUseCase::discover(
            &self.conn,
            payload.query,
            business_type,
            payload.latitude,
            payload.longitude,
            payload.radius_km,
            payload.limit.unwrap_or(50),
        )
        .await
        .map_err(|error| localized_business_status(error, ErrorKey::BusinessProfileNotFound, locale))?;
        let shown = BusinessProfileUseCase::present_all(&self.conn, profiles, viewer.as_ref()).await;
        Ok(Response::new(BusinessProfilesResponse { business_profiles: BusinessProfileMapper::response_vec(shown) }))
    }

    async fn delete_business_profile(
        &self,
        request: Request<DeleteBusinessProfileRequest>,
    ) -> Result<Response<DeleteBusinessProfileResponse>, Status> {
        let locale = locale_of(&request);
        let actor = require_actor(&request)?;
        BusinessProfileUseCase::delete(&self.conn, request.into_inner().id, &actor)
            .await
            .map_err(|error| localized_business_status(error, ErrorKey::BusinessProfileNotFound, locale))?;
        Ok(Response::new(DeleteBusinessProfileResponse {}))
    }
}
