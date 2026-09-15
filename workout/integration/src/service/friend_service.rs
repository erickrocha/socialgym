use std::sync::Arc;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use business::domain::business_error::{BusinessError, BusinessErrorKind};
use business::domain::user::User;
use business::use_cases::friend_use_case::FriendUseCase;
use business::use_cases::person_use_case::PersonUseCase;

use crate::proto::friend::friend_service_server::FriendService;
use crate::proto::friend::{
	Friend, FriendPageRequest, FriendPageResponse, FriendRequestRequest, FriendsRequest,
	FriendsResponse, RemoveFriendResponse, SearchFriendsRequest, SearchFriendsResponse,
};
use crate::infrastructure::mapper::{Mapper, FriendMapper, PersonMapper};

pub struct GrpcFriendService {
	conn: Arc<DatabaseConnection>,
}

impl GrpcFriendService {
	pub fn new(conn: Arc<DatabaseConnection>) -> Self {
		Self { conn }
	}

	/// Resolve the authenticated caller's `person_id` from the request
	/// extensions populated by `GrpcAuthLayer` — the gRPC equivalent of the
	/// REST controller's `current_user.person_id`.
	fn caller_person_id<T>(request: &Request<T>) -> Result<i32, Status> {
		request
			.extensions()
			.get::<User>()
			.map(|user| user.person_id)
			.filter(|id| *id > 0)
			.ok_or_else(|| Status::unauthenticated("authenticated user missing"))
	}
}

/// Translate a `BusinessError` into the closest gRPC status so clients get the
/// same signal REST callers get from `ExceptionResponse::from_business`.
fn to_status(error: BusinessError) -> Status {
	match error.kind {
		BusinessErrorKind::Validation => Status::invalid_argument(error.message),
		BusinessErrorKind::Unauthorized => Status::unauthenticated(error.message),
		BusinessErrorKind::Forbidden => Status::permission_denied(error.message),
		BusinessErrorKind::NotFound => Status::not_found(error.message),
		BusinessErrorKind::Conflict => Status::already_exists(error.message),
		BusinessErrorKind::Locked => Status::failed_precondition(error.message),
		BusinessErrorKind::Infrastructure => Status::internal(error.message),
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
		// `person_id` in the body is an optional override; normally the caller's
		// token identifies them, mirroring REST's `GET /workout/api/friends`.
		let person_id = if request.get_ref().person_id > 0 {
			request.get_ref().person_id
		} else {
			Self::caller_person_id(&request)?
		};
		let payload = request.into_inner();
		let radius_km = payload.radius_km.unwrap_or(200.0);

		let suggestions = PersonUseCase::get_suggestions(
			&self.conn,
			person_id,
			radius_km,
			payload.latitude,
			payload.longitude,
		)
		.await;
		let friends = PersonUseCase::get_all_friends(&self.conn, person_id).await;
		let receive_requests =
			PersonUseCase::get_all_received_requests(&self.conn, person_id).await;
		let sent_requests = PersonUseCase::get_all_sent_requests(&self.conn, person_id).await;

		Ok(Response::new(FriendPageResponse {
			suggestions: PersonMapper::response_vec(suggestions),
			friends: PersonMapper::response_vec(friends),
			receive_requests: PersonMapper::response_vec(receive_requests),
			sent_requests: PersonMapper::response_vec(sent_requests),
		}))
	}

	async fn send_friend_request(
		&self,
		request: Request<FriendRequestRequest>,
	) -> Result<Response<Friend>, Status> {
		let sender_id = Self::caller_person_id(&request)?;
		let target_id = request.into_inner().person_id;
		if target_id <= 0 {
			return Err(Status::invalid_argument("person_id must be informed"));
		}

		let friend = FriendUseCase::send_friend_request(&self.conn, sender_id, target_id)
			.await
			.map_err(to_status)?;

		Ok(Response::new(FriendMapper::response(friend)))
	}

	async fn accept_friend_request(
		&self,
		request: Request<FriendRequestRequest>,
	) -> Result<Response<Friend>, Status> {
		let person_id = Self::caller_person_id(&request)?;
		let sender_id = request.into_inner().person_id;
		if sender_id <= 0 {
			return Err(Status::invalid_argument("person_id must be informed"));
		}

		let friend = FriendUseCase::accept_friend_request(&self.conn, person_id, sender_id)
			.await
			.map_err(to_status)?;

		Ok(Response::new(FriendMapper::response(friend)))
	}

	async fn deny_friend_request(
		&self,
		request: Request<FriendRequestRequest>,
	) -> Result<Response<Friend>, Status> {
		let person_id = Self::caller_person_id(&request)?;
		let sender_id = request.into_inner().person_id;
		if sender_id <= 0 {
			return Err(Status::invalid_argument("person_id must be informed"));
		}

		let friend = FriendUseCase::deny_friend_request(&self.conn, person_id, sender_id)
			.await
			.map_err(to_status)?;

		Ok(Response::new(FriendMapper::response(friend)))
	}

	async fn cancel_friend_request(
		&self,
		request: Request<FriendRequestRequest>,
	) -> Result<Response<Friend>, Status> {
		let person_id = Self::caller_person_id(&request)?;
		let receiver_id = request.into_inner().person_id;
		if receiver_id <= 0 {
			return Err(Status::invalid_argument("person_id must be informed"));
		}

		let friend = FriendUseCase::cancel_friend_request(&self.conn, person_id, receiver_id)
			.await
			.map_err(to_status)?;

		Ok(Response::new(FriendMapper::response(friend)))
	}

	async fn remove_friend(
		&self,
		request: Request<FriendRequestRequest>,
	) -> Result<Response<RemoveFriendResponse>, Status> {
		let person_id = Self::caller_person_id(&request)?;
		let friend_id = request.into_inner().person_id;
		if friend_id <= 0 {
			return Err(Status::invalid_argument("person_id must be informed"));
		}

		FriendUseCase::remove_friend(&self.conn, person_id, friend_id)
			.await
			.map_err(to_status)?;

		Ok(Response::new(RemoveFriendResponse { success: true }))
	}

	async fn search_friends(
		&self,
		request: Request<SearchFriendsRequest>,
	) -> Result<Response<SearchFriendsResponse>, Status> {
		let person_id = Self::caller_person_id(&request)?;
		let payload = request.into_inner();
		let query = {
			let trimmed = payload.query.trim();
			(!trimmed.is_empty()).then(|| trimmed.to_string())
		};
		let limit = if payload.limit > 0 { payload.limit } else { 50 };

		let people = PersonUseCase::find_friends(
			&self.conn,
			person_id,
			query,
			payload.latitude,
			payload.longitude,
			payload.radius_km,
			limit,
		)
		.await;

		Ok(Response::new(SearchFriendsResponse {
			people: PersonMapper::response_vec(people),
		}))
	}
}
