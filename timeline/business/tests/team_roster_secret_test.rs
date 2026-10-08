//! C-010 TC-012 step 4: the roster call `timeline` makes for a business chat carries the shared secret in
//! `x-internal-secret` and never the caller's token, and it fails without the secret configured.
use business::gateway::team_member_gateway::TeamMemberGateway;
use business::proto::proto::team_member::team_member_service_server::{TeamMemberService, TeamMemberServiceServer};
use business::proto::proto::team_member::{TeamMember, TeamMemberPageRequest, TeamMemberPageResponse, TeamMemberRequest, TeamRosterRequest, TeamRosterResponse};
use std::sync::{Arc, Mutex};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Request, Response, Status};

#[derive(Default, Clone)]
struct Fake {
    seen: Arc<Mutex<Vec<(Option<String>, Option<String>)>>>,
}

#[tonic::async_trait]
impl TeamMemberService for Fake {
    async fn get_team_roster(&self, request: Request<TeamRosterRequest>) -> Result<Response<TeamRosterResponse>, Status> {
        let metadata = request.metadata();
        let header = |name: &str| metadata.get(name).and_then(|v| v.to_str().ok()).map(str::to_string);
        self.seen.lock().unwrap().push((header("x-internal-secret"), header("authorization")));
        Ok(Response::new(TeamRosterResponse {
            business_profile_id: 7,
            business_profile_uuid: request.get_ref().business_profile_uuid.clone(),
            business_profile_name: "Gym".into(),
            business_profile_logo_object_key: String::new(),
            owner_person_uuid: "owner".into(),
            accepted_member_person_uuids: vec!["member".into()],
        }))
    }
    async fn get_team_member_page(&self, _: Request<TeamMemberPageRequest>) -> Result<Response<TeamMemberPageResponse>, Status> {
        Err(Status::unimplemented("not used"))
    }
    async fn get_team_member(&self, _: Request<TeamMemberRequest>) -> Result<Response<TeamMember>, Status> {
        Err(Status::unimplemented("not used"))
    }
    async fn send_team_member_request(&self, _: Request<TeamMemberRequest>) -> Result<Response<TeamMember>, Status> {
        Err(Status::unimplemented("not used"))
    }
    async fn accept_team_member_request(&self, _: Request<TeamMemberRequest>) -> Result<Response<TeamMember>, Status> {
        Err(Status::unimplemented("not used"))
    }
    async fn deny_team_member_request(&self, _: Request<TeamMemberRequest>) -> Result<Response<TeamMember>, Status> {
        Err(Status::unimplemented("not used"))
    }
    async fn cancel_team_member_request(&self, _: Request<TeamMemberRequest>) -> Result<Response<TeamMember>, Status> {
        Err(Status::unimplemented("not used"))
    }
}

#[tokio::test]
async fn the_roster_call_carries_the_secret_and_never_a_user_token() {
    let fake = Fake::default();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(TeamMemberServiceServer::new(fake.clone()))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );
    unsafe {
        std::env::set_var("GRPC_USE_TLS", "false");
        std::env::set_var("INTERNAL_SERVICE_SECRET", "c010-roster-secret");
    }
    let gateway = TeamMemberGateway::new(format!("http://{address}"));
    let roster = gateway.get_team_roster("profile-uuid").await.expect("roster");
    assert!(roster.allows("owner") && roster.allows("member") && !roster.allows("stranger"));
    assert_eq!(
        fake.seen.lock().unwrap().as_slice(),
        [(Some("c010-roster-secret".to_string()), None)],
        "the secret goes out, no authorization header does"
    );

    unsafe { std::env::remove_var("INTERNAL_SERVICE_SECRET") };
    assert!(gateway.get_team_roster("profile-uuid").await.is_err(), "no secret configured: no call is made");
    assert_eq!(fake.seen.lock().unwrap().len(), 1);
}
