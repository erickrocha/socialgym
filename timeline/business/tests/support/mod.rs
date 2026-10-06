//! Shared fixtures for use-case acceptance tests: a disposable MongoDB database and a
//! plaintext gRPC stand-in for the workout service (roles, friends, team rosters).
#![allow(dead_code)]
use business::proto::proto::business_profile::business_profile_service_server::{BusinessProfileService, BusinessProfileServiceServer};
use business::proto::proto::business_profile::*;
use business::proto::proto::friend::friend_service_server::{FriendService, FriendServiceServer};
use business::proto::proto::friend::*;
use business::proto::proto::person::person_service_server::{PersonService, PersonServiceServer};
use business::proto::proto::person::*;
use business::proto::proto::team_member::team_member_service_server::{
    TeamMemberService, TeamMemberServiceServer,
};
use business::proto::proto::team_member::*;
use mongodb::{Client, Database};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct StubState {
    pub moderator: Mutex<bool>,
    pub friends: Mutex<Vec<String>>,
    /// uuids the stand-in reports as Business Profiles; any other uuid is NOT_FOUND.
    pub business_profiles: Mutex<Vec<String>>,
    pub roster: Mutex<Option<TeamRosterResponse>>,
}

#[derive(Clone)]
pub struct Stub(pub Arc<StubState>);

pub async fn database() -> Database {
    let url = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(url.contains("/timeline_test"), "refusing to run against a non-test database");
    let db = Client::with_uri_str(url).await.unwrap().database("timeline_test");
    for name in db.list_collection_names().await.unwrap() {
        db.collection::<mongodb::bson::Document>(&name).delete_many(mongodb::bson::doc! {}).await.unwrap();
    }
    db
}

/// Starts the stub on an ephemeral port and points the gRPC client config at it.
pub async fn start_stub() -> Arc<StubState> {
    let state = Arc::new(StubState::default());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let stub = Stub(state.clone());
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(PersonServiceServer::new(stub.clone()))
            .add_service(FriendServiceServer::new(stub.clone()))
            .add_service(BusinessProfileServiceServer::new(stub.clone()))
            .add_service(TeamMemberServiceServer::new(stub))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    unsafe {
        std::env::set_var("GRPC_PROTOCOL", "http");
        std::env::set_var("GRPC_HOST", "127.0.0.1");
        std::env::set_var("GRPC_PORT", port.to_string());
        std::env::set_var("GRPC_USE_TLS", "false");
    }
    state
}

macro_rules! service_impl {
    ($trait:ident { $($real:tt)* } $($name:ident($req:ty) -> $res:ty;)*) => {
        #[tonic::async_trait]
        impl $trait for Stub {
            $($real)*
            $(async fn $name(&self, _: Request<$req>) -> Result<Response<$res>, Status> {
                Err(Status::unimplemented("not used by the use-case tests"))
            })*
        }
    };
}

service_impl! { PersonService {
    async fn has_role(&self, _: Request<RoleStatusRequest>) -> Result<Response<RoleStatusResponse>, Status> {
        Ok(Response::new(RoleStatusResponse { active: *self.0.moderator.lock().unwrap() }))
    }
    async fn has_active_consent(&self, _: Request<ConsentStatusRequest>) -> Result<Response<ConsentStatusResponse>, Status> {
        Ok(Response::new(ConsentStatusResponse { active: true, version: "test".into() }))
    }
}
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

service_impl! { FriendService {
    async fn get_friends(&self, req: Request<FriendsRequest>) -> Result<Response<FriendsResponse>, Status> {
        let me = req.into_inner().uuid;
        let friends = self
            .0
            .friends
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, other)| Friend {
                id: i as i32,
                uuid: format!("f{i}"),
                person_id: 0,
                person_uuid: me.clone(),
                friend_id: 0,
                friend_uuid: other.clone(),
                status: "Accepted".into(),
            })
            .collect();
        Ok(Response::new(FriendsResponse { friends }))
    }
}
        get_friend_page(FriendPageRequest) -> FriendPageResponse;
        search_friends(SearchFriendsRequest) -> SearchFriendsResponse;
        send_friend_request(FriendRequestRequest) -> Friend;
        accept_friend_request(FriendRequestRequest) -> Friend;
        deny_friend_request(FriendRequestRequest) -> Friend;
        cancel_friend_request(FriendRequestRequest) -> Friend;
        remove_friend(FriendRequestRequest) -> RemoveFriendResponse;
}

service_impl! { BusinessProfileService {
    async fn get_business_profile_by_id(&self, req: Request<BusinessProfileRequestId>) -> Result<Response<BusinessProfile>, Status> {
        let uuid = req.into_inner().uuid;
        if self.0.business_profiles.lock().unwrap().contains(&uuid) {
            Ok(Response::new(BusinessProfile { uuid, ..Default::default() }))
        } else {
            Err(Status::not_found("Business profile not found"))
        }
    }
}
        get_business_profile_by_owner_id(BusinessProfileRequestOwnerId) -> BusinessProfilesResponse;
        add_business_profile(BusinessProfile) -> BusinessProfile;
        update_business_profile(BusinessProfile) -> BusinessProfile;
        add_business_profile_address(business::proto::proto::business_profile_address::BusinessProfileAddress) -> business::proto::proto::business_profile_address::BusinessProfileAddress;
        update_business_profile_address(business::proto::proto::business_profile_address::BusinessProfileAddress) -> business::proto::proto::business_profile_address::BusinessProfileAddress;
        remove_business_profile_address(RemoveBusinessProfileAddressRequest) -> RemoveBusinessProfileAddressResponse;
}

service_impl! { TeamMemberService {
    async fn get_team_roster(&self, _: Request<TeamRosterRequest>) -> Result<Response<TeamRosterResponse>, Status> {
        self.0
            .roster
            .lock()
            .unwrap()
            .clone()
            .map(Response::new)
            .ok_or_else(|| Status::not_found("no roster"))
    }
}
        get_team_member_page(TeamMemberPageRequest) -> TeamMemberPageResponse;
        get_team_member(TeamMemberRequest) -> TeamMember;
        send_team_member_request(TeamMemberRequest) -> TeamMember;
        accept_team_member_request(TeamMemberRequest) -> TeamMember;
        deny_team_member_request(TeamMemberRequest) -> TeamMember;
        cancel_team_member_request(TeamMemberRequest) -> TeamMember;
}
