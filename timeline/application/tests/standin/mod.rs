//! The in-process stand-in for the `workout` gRPC service shared by the C-008 acceptance tests:
//! consent is always active, nobody holds a role, no Business Profile exists unless listed, and friendships come
//! from a fixed map.
#![allow(dead_code)]
use business::proto::proto::business_profile::business_profile_service_server::BusinessProfileService;
use business::proto::proto::business_profile::*;
use business::proto::proto::business_profile_address::BusinessProfileAddress;
use business::proto::proto::friend::friend_service_server::FriendService;
use business::proto::proto::friend::*;
use business::proto::proto::person::person_service_server::PersonService;
use business::proto::proto::person::*;
use std::collections::HashMap;
use tonic::{Request as GrpcRequest, Response, Status};

// ---------------------------------------------------------------- workout stand-in

/// Consent is always active, nobody holds a role, and friends come from a fixed map.
#[derive(Default)]
pub struct Workout {
    pub friends: HashMap<String, Vec<String>>,
    /// Access tokens of people whose Terms or Privacy consent is not current.
    pub denied_tokens: Vec<String>,
    /// Access tokens of people who hold the moderator role.
    pub moderator_tokens: Vec<String>,
    /// Access tokens whose role lookup fails with an error (the role service misbehaves).
    pub role_error_tokens: Vec<String>,
    /// Uuids that are Business Profiles; every other lookup is NOT_FOUND.
    pub business_profiles: Vec<String>,
    /// The Business Profile lookup fails as if `workout` were down.
    pub business_lookup_down: bool,
}

macro_rules! person_stub {
    ($($name:ident($req:ty) -> $res:ty;)*) => {
        #[tonic::async_trait]
        impl PersonService for Workout {
            async fn has_active_consent(
                &self,
                request: GrpcRequest<ConsentStatusRequest>,
            ) -> Result<Response<ConsentStatusResponse>, Status> {
                let token = request
                    .metadata()
                    .get("authorization")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.strip_prefix("Bearer "))
                    .unwrap_or_default()
                    .to_string();
                let active = !self.denied_tokens.contains(&token);
                Ok(Response::new(ConsentStatusResponse { active, version: "test".into() }))
            }
            async fn has_role(
                &self,
                request: GrpcRequest<RoleStatusRequest>,
            ) -> Result<Response<RoleStatusResponse>, Status> {
                let token = request
                    .metadata()
                    .get("authorization")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.strip_prefix("Bearer "))
                    .unwrap_or_default()
                    .to_string();
                if self.role_error_tokens.contains(&token) {
                    return Err(Status::internal("role lookup failed"));
                }
                Ok(Response::new(RoleStatusResponse { active: self.moderator_tokens.contains(&token) }))
            }
            $(async fn $name(&self, _: GrpcRequest<$req>) -> Result<Response<$res>, Status> {
                Err(Status::unimplemented("not used by the C-008 gap tests"))
            })*
        }
    };
}

person_stub! {
    get_person(PersonIdRequest) -> PersonResponse;
    get_me(GetMeRequest) -> PersonResponse;
    search_mentionable_friends(SearchMentionableFriendsRequest) -> PeopleResponse;
    update_person(Person) -> PersonResponse;
    search_persons(PersonParams) -> PeopleResponse;
    update_person_info(business::proto::proto::person_info::PersonInfo) -> business::proto::proto::person_info::PersonInfo;
    add_person_address(business::proto::proto::person_address::PersonAddress) -> business::proto::proto::person_address::PersonAddress;
    update_person_address(business::proto::proto::person_address::PersonAddress) -> business::proto::proto::person_address::PersonAddress;
    remove_person_address(RemovePersonAddressRequest) -> RemovePersonAddressResponse;
    get_person_image_upload_url(PersonImageUploadRequest) -> PersonImageUploadResponse;
    delete_person_image(PersonImageRequest) -> DeletePersonImageResponse;
}

#[tonic::async_trait]
impl FriendService for Workout {
    async fn get_friends(
        &self,
        request: GrpcRequest<FriendsRequest>,
    ) -> Result<Response<FriendsResponse>, Status> {
        let me = request.into_inner().uuid;
        let friends = self
            .friends
            .get(&me)
            .into_iter()
            .flatten()
            .map(|other| Friend {
                person_uuid: me.clone(),
                friend_uuid: other.clone(),
                ..Default::default()
            })
            .collect();
        Ok(Response::new(FriendsResponse { friends }))
    }
    async fn get_friend_page(&self, _: GrpcRequest<FriendPageRequest>) -> Result<Response<FriendPageResponse>, Status> { Err(Status::unimplemented("")) }
    async fn search_friends(&self, _: GrpcRequest<SearchFriendsRequest>) -> Result<Response<SearchFriendsResponse>, Status> { Err(Status::unimplemented("")) }
    async fn send_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn accept_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn deny_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn cancel_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn remove_friend(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<RemoveFriendResponse>, Status> { Err(Status::unimplemented("")) }
    async fn get_friend_profile(&self, _: GrpcRequest<FriendProfileRequest>) -> Result<Response<FriendProfileResponse>, Status> { Err(Status::unimplemented("")) }
}

/// The cast has no Business Profiles: every lookup is NOT_FOUND, like workout's real answer.
#[tonic::async_trait]
impl BusinessProfileService for Workout {
    async fn get_business_profile_by_id(&self, request: GrpcRequest<BusinessProfileRequestId>) -> Result<Response<BusinessProfile>, Status> {
        if self.business_lookup_down {
            return Err(Status::unavailable("workout is down"));
        }
        if self.business_profiles.contains(&request.into_inner().uuid) {
            return Ok(Response::new(BusinessProfile::default()));
        }
        Err(Status::not_found("Business profile not found"))
    }
    async fn get_business_profile_by_owner_id(&self, _: GrpcRequest<BusinessProfileRequestOwnerId>) -> Result<Response<BusinessProfilesResponse>, Status> { Err(Status::unimplemented("")) }
    async fn add_business_profile(&self, _: GrpcRequest<BusinessProfile>) -> Result<Response<BusinessProfile>, Status> { Err(Status::unimplemented("")) }
    async fn update_business_profile(&self, _: GrpcRequest<BusinessProfile>) -> Result<Response<BusinessProfile>, Status> { Err(Status::unimplemented("")) }
    async fn add_business_profile_address(&self, _: GrpcRequest<BusinessProfileAddress>) -> Result<Response<BusinessProfileAddress>, Status> { Err(Status::unimplemented("")) }
    async fn update_business_profile_address(&self, _: GrpcRequest<BusinessProfileAddress>) -> Result<Response<BusinessProfileAddress>, Status> { Err(Status::unimplemented("")) }
    async fn remove_business_profile_address(&self, _: GrpcRequest<RemoveBusinessProfileAddressRequest>) -> Result<Response<RemoveBusinessProfileAddressResponse>, Status> { Err(Status::unimplemented("")) }
    async fn get_active_business_profile(&self, _: GrpcRequest<GetActiveBusinessProfileRequest>) -> Result<Response<BusinessProfile>, Status> { Err(Status::unimplemented("")) }
    async fn discover_business_profiles(&self, _: GrpcRequest<DiscoverBusinessProfilesRequest>) -> Result<Response<BusinessProfilesResponse>, Status> { Err(Status::unimplemented("")) }
    async fn delete_business_profile(&self, _: GrpcRequest<DeleteBusinessProfileRequest>) -> Result<Response<DeleteBusinessProfileResponse>, Status> { Err(Status::unimplemented("")) }
    async fn get_business_profile_image_upload_url(&self, _: GrpcRequest<BusinessProfileImageUploadRequest>) -> Result<Response<BusinessProfileImageUploadResponse>, Status> { Err(Status::unimplemented("")) }
}

