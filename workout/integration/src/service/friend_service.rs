use std::sync::Arc;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use business::use_cases::friend_use_case::FriendUseCase;
use business::use_cases::person_use_case::PersonUseCase;

use crate::proto::friend::friend_service_server::FriendService;
use crate::proto::friend::{FriendsRequest, FriendsResponse, FriendPageRequest, FriendPageResponse};
use crate::infrastructure::mapper::{Mapper, FriendMapper, PersonMapper};

pub struct GrpcFriendService {
	conn: Arc<DatabaseConnection>,
}

impl GrpcFriendService {
	pub fn new(conn: Arc<DatabaseConnection>) -> Self {
		Self { conn }
	}
}

#[tonic::async_trait]
impl FriendService for GrpcFriendService {
	async fn get_friends(
		&self,
		request: Request<FriendsRequest>,
	) -> Result<Response<FriendsResponse>, Status> {
		let payload = request.into_inner();

		if payload.id <= 0 && payload.uuid.is_empty() {
			return Err(Status::invalid_argument("either id or uuid must be informed"));
		}

		let friends = FriendUseCase::find_all_friend(&self.conn, payload.id)
			.await
			.map_err(|e| Status::internal(e.message))?;

		let grpc_friends = FriendMapper::response_vec(friends);

		Ok(Response::new(FriendsResponse { friends: grpc_friends }))
	}

	async fn get_friend_page(
		&self,
		request: Request<FriendPageRequest>,
	) -> Result<Response<FriendPageResponse>, Status> {
		let payload = request.into_inner();

		if payload.person_id <= 0 {
			return Err(Status::invalid_argument("person_id must be informed"));
		}

		let suggestions =
			PersonUseCase::get_suggestions(&self.conn, payload.person_id, 200.0, None, None).await;
		let friends = PersonUseCase::get_all_friends(&self.conn, payload.person_id).await;
		let receive_requests =
			PersonUseCase::get_all_received_requests(&self.conn, payload.person_id).await;
		let sent_requests = PersonUseCase::get_all_sent_requests(&self.conn, payload.person_id).await;

		Ok(Response::new(FriendPageResponse {
			suggestions: PersonMapper::response_vec(suggestions),
			friends: PersonMapper::response_vec(friends),
			receive_requests: PersonMapper::response_vec(receive_requests),
			sent_requests: PersonMapper::response_vec(sent_requests),
		}))
	}
}