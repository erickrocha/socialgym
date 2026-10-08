use business::domain::enums::InviteStatus;
use business::use_cases::team_member_use_case::TeamMemberUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::infrastructure::utils::{require_active_profile, require_person_id};

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

	/// The active business profile of the caller (from the validated token) must be
	/// `business_profile_id`: a business profile invites, cancels and reads its own team.
	fn require_acting_profile<T>(request: &Request<T>, business_profile_id: i32) -> Result<(), Status> {
		let active = require_active_profile(request)
			.and_then(|profile| profile.id)
			.ok_or_else(|| Status::permission_denied("An active business profile is required"))?;
		if active != business_profile_id {
			return Err(Status::permission_denied(
				"Not the active business profile of this team",
			));
		}
		Ok(())
	}

	/// The invited person (the caller) accepts and denies, and reads their own teams.
	fn require_caller_person<T>(request: &Request<T>, person_id: i32) -> Result<(), Status> {
		if require_person_id(request)? != person_id {
			return Err(Status::permission_denied("Not the invited person"));
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
		let caller_person_id = require_person_id(&request)?;
		let viewer = request.extensions().get::<business::domain::user::User>().cloned();
		let acting_profile_id = require_active_profile(&request).and_then(|profile| profile.id);
		let payload = request.into_inner();

		if payload.business_profile_id <= 0 && payload.person_id <= 0 {
			return Err(Status::invalid_argument(
				"either business_profile_id or person_id must be informed",
			));
		}
		// Each side is only readable by its owner: the team by its active business
		// profile, the invitations and memberships by the person they belong to.
		if payload.business_profile_id > 0 && acting_profile_id != Some(payload.business_profile_id) {
			return Err(Status::permission_denied(
				"Not the active business profile of this team",
			));
		}
		if payload.person_id > 0 && payload.person_id != caller_person_id {
			return Err(Status::permission_denied("Not the invited person"));
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
			// These are other people's businesses: the tax id is for the owner only.
			teams: BusinessProfileMapper::response_vec(
				teams.into_iter().map(|p| p.for_viewer(viewer.as_ref())).collect(),
			),
			received_requests: BusinessProfileMapper::response_vec(
				received_requests
					.into_iter()
					.map(|p| p.for_viewer(viewer.as_ref()))
					.collect(),
			),
		}))
	}

	async fn get_team_member(
		&self,
		request: Request<TeamMemberRequest>,
	) -> Result<Response<TeamMember>, Status> {
		let payload = request.get_ref().clone();
		Self::validate(&payload)?;
		Self::require_acting_profile(&request, payload.business_profile_id)?;

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
		let payload = request.get_ref().clone();
		Self::validate(&payload)?;
		Self::require_acting_profile(&request, payload.business_profile_id)?;

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
		let payload = request.get_ref().clone();
		Self::validate(&payload)?;
		Self::require_caller_person(&request, payload.person_id)?;

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
		let payload = request.get_ref().clone();
		Self::validate(&payload)?;
		Self::require_caller_person(&request, payload.person_id)?;

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
		let payload = request.get_ref().clone();
		Self::validate(&payload)?;
		Self::require_acting_profile(&request, payload.business_profile_id)?;

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
				.map_err(crate::infrastructure::utils::business_status)?;

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

#[cfg(test)]
mod tests {
	use super::*;
	use business::domain::business_profile::BusinessProfile;
	use business::domain::enums::ProfileType;
	use business::domain::user::User;

	/// A request from `person_id`, optionally acting as the business profile `profile_id`.
	fn request(person_id: i32, profile_id: Option<i32>) -> Request<()> {
		let mut request = Request::new(());
		request.extensions_mut().insert(User::new(
			Some("Caller".to_string()),
			"caller@example.test".to_string(),
			"hashed".to_string(),
			person_id,
			"00000000-0000-0000-0000-000000000001".to_string(),
		));
		if let Some(id) = profile_id {
			let mut profile = BusinessProfile::new(
				person_id,
				"00000000-0000-0000-0000-000000000001".to_string(),
				"tax".to_string(),
				"Gym".to_string(),
				ProfileType::Professional,
				None,
			);
			profile.id = Some(id);
			request.extensions_mut().insert(profile);
		}
		request
	}

	#[test]
	fn a_team_is_only_managed_by_its_active_business_profile() {
		assert!(GrpcTeamMemberService::require_acting_profile(&request(1, Some(7)), 7).is_ok());
		// Another profile's id, or no active profile at all, is denied.
		let other = GrpcTeamMemberService::require_acting_profile(&request(1, Some(7)), 8)
			.unwrap_err();
		assert_eq!(other.code(), tonic::Code::PermissionDenied);
		let none = GrpcTeamMemberService::require_acting_profile(&request(1, None), 7).unwrap_err();
		assert_eq!(none.code(), tonic::Code::PermissionDenied);
	}

	#[test]
	fn an_invitation_is_only_answered_by_the_invited_person() {
		assert!(GrpcTeamMemberService::require_caller_person(&request(2, None), 2).is_ok());
		let forged = GrpcTeamMemberService::require_caller_person(&request(2, None), 3).unwrap_err();
		assert_eq!(forged.code(), tonic::Code::PermissionDenied);
	}
}
