use business::domain::enums::InviteStatus;
use business::use_cases::team_member_use_case::TeamMemberUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::infrastructure::mapper::{
    BusinessProfileMapper, Mapper, PersonMapper, TeamMemberMapper,
};
use crate::proto::team_member::team_member_service_server::TeamMemberService;
use crate::proto::team_member::{
    TeamMember, TeamMemberPageRequest, TeamMemberPageResponse, TeamMemberRequest, TeamRosterRequest,
    TeamRosterResponse,
};

pub struct GrpcTeamMemberService {
	conn: Arc<DatabaseConnection>,
}

impl GrpcTeamMemberService {
	pub fn new(conn: Arc<DatabaseConnection>) -> Self {
		Self { conn }
	}
}

impl GrpcTeamMemberService {
	fn validate(payload: &TeamMemberRequest) -> Result<(), Status> {
		if payload.business_profile_id <= 0 || payload.person_id <= 0 {
			return Err(Status::invalid_argument(
				"business_profile_id and person_id must be informed",
			));
		}
		Ok(())
	}
}

#[tonic::async_trait]
impl TeamMemberService for GrpcTeamMemberService {
	async fn get_team_member_page(
		&self,
		request: Request<TeamMemberPageRequest>,
	) -> Result<Response<TeamMemberPageResponse>, Status> {
		let payload = request.into_inner();

		if payload.business_profile_id <= 0 && payload.person_id <= 0 {
			return Err(Status::invalid_argument(
				"either business_profile_id or person_id must be informed",
			));
		}

		let (members, sent_requests) = if payload.business_profile_id > 0 {
			(
				TeamMemberUseCase::find_all_persons(
					&self.conn,
					payload.business_profile_id,
					InviteStatus::Accepted,
				)
				.await,
				TeamMemberUseCase::find_all_persons(
					&self.conn,
					payload.business_profile_id,
					InviteStatus::Pending,
				)
				.await,
			)
		} else {
			(Vec::new(), Vec::new())
		};

		let (teams, received_requests) = if payload.person_id > 0 {
			(
				TeamMemberUseCase::find_all_business_profiles(
					&self.conn,
					payload.person_id,
					InviteStatus::Accepted,
				)
				.await,
				TeamMemberUseCase::find_all_business_profiles(
					&self.conn,
					payload.person_id,
					InviteStatus::Pending,
				)
				.await,
			)
		} else {
			(Vec::new(), Vec::new())
		};

		Ok(Response::new(TeamMemberPageResponse {
			members: PersonMapper::response_vec(members),
			sent_requests: PersonMapper::response_vec(sent_requests),
			teams: BusinessProfileMapper::response_vec(teams),
			received_requests: BusinessProfileMapper::response_vec(received_requests),
		}))
	}

	async fn get_team_member(
		&self,
		request: Request<TeamMemberRequest>,
	) -> Result<Response<TeamMember>, Status> {
		let payload = request.into_inner();
		Self::validate(&payload)?;

		let team_member = TeamMemberUseCase::find_membership(
			&self.conn,
			payload.business_profile_id,
			payload.person_id,
		)
		.await
		.map_err(|e| Status::not_found(e.message))?;

		Ok(Response::new(TeamMemberMapper::response(team_member)))
	}

	async fn send_team_member_request(
		&self,
		request: Request<TeamMemberRequest>,
	) -> Result<Response<TeamMember>, Status> {
		let payload = request.into_inner();
		Self::validate(&payload)?;

		let team_member = TeamMemberUseCase::send_team_member_request(
			&self.conn,
			payload.business_profile_id,
			payload.person_id,
		)
		.await
		.map_err(|e| Status::invalid_argument(e.message))?;

		Ok(Response::new(TeamMemberMapper::response(team_member)))
	}

	async fn accept_team_member_request(
		&self,
		request: Request<TeamMemberRequest>,
	) -> Result<Response<TeamMember>, Status> {
		let payload = request.into_inner();
		Self::validate(&payload)?;

		let team_member = TeamMemberUseCase::accept_team_member_request(
			&self.conn,
			payload.business_profile_id,
			payload.person_id,
		)
		.await
		.map_err(|e| Status::invalid_argument(e.message))?;

		Ok(Response::new(TeamMemberMapper::response(team_member)))
	}

	async fn deny_team_member_request(
		&self,
		request: Request<TeamMemberRequest>,
	) -> Result<Response<TeamMember>, Status> {
		let payload = request.into_inner();
		Self::validate(&payload)?;

		let team_member = TeamMemberUseCase::deny_team_member_request(
			&self.conn,
			payload.business_profile_id,
			payload.person_id,
		)
		.await
		.map_err(|e| Status::invalid_argument(e.message))?;

		Ok(Response::new(TeamMemberMapper::response(team_member)))
	}

	async fn cancel_team_member_request(
		&self,
		request: Request<TeamMemberRequest>,
	) -> Result<Response<TeamMember>, Status> {
		let payload = request.into_inner();
		Self::validate(&payload)?;

		let team_member = TeamMemberUseCase::cancel_team_member_request(
			&self.conn,
			payload.business_profile_id,
			payload.person_id,
		)
		.await
		.map_err(|e| Status::invalid_argument(e.message))?;

		Ok(Response::new(TeamMemberMapper::response(team_member)))
	}

	async fn get_team_roster(
		&self,
		request: Request<TeamRosterRequest>,
	) -> Result<Response<TeamRosterResponse>, Status> {
		let payload = request.into_inner();

		if payload.business_profile_uuid.trim().is_empty() {
			return Err(Status::invalid_argument(
				"business_profile_uuid must be informed",
			));
		}

		let roster =
			TeamMemberUseCase::find_roster(&self.conn, payload.business_profile_uuid.trim())
				.await
				.map_err(|e| Status::not_found(e.message))?;

		Ok(Response::new(TeamRosterResponse {
			business_profile_id: roster.business_profile_id,
			business_profile_uuid: roster.business_profile_uuid,
			business_profile_name: roster.business_profile_name,
			business_profile_logo_object_key: roster
				.business_profile_logo_object_key
				.unwrap_or_default(),
			owner_person_uuid: roster.owner_person_uuid,
			accepted_member_person_uuids: roster.accepted_member_person_uuids,
		}))
	}
}
