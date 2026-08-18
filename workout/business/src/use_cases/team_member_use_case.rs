use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::uuid_to_string;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::{BusinessProfile, BusinessProfileEntityMapper};
use crate::domain::person::{Person, PersonEntityMapper};
use crate::domain::team_member::{TeamMember, TeamMemberMapper, TeamMemberStatus};
use crate::gateway::business_profile_gateway::BusinessProfileGateway;
use crate::gateway::person_gateway::PersonGateway;
use crate::gateway::team_member_gateway::TeamMemberGateway;
use sea_orm::DbConn;

pub struct TeamMemberUseCase {}

impl TeamMemberUseCase {
    /// A business profile invites a person to its team. The person is the one who accepts or
    /// denies it — a profile never grants itself authority over a person.
    pub async fn send_team_member_request(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
    ) -> Result<TeamMember, BusinessError> {
        log::info!(
            "Attempting to send team member request from business_profile_id: {:?} to person_id: {:?}",
            business_profile_id,
            person_id
        );

        let business_profile = BusinessProfileGateway::find_by_id(db, business_profile_id)
            .await
            .ok_or_else(|| BusinessError::new("Business profile not found".to_string()))?;

        if business_profile.owner_id == person_id {
            log::error!(
                "Cannot invite the owner of business profile {:?} to its own team",
                business_profile_id
            );
            return Err(BusinessError::new(
                "Cannot invite the owner to its own team".to_string(),
            ));
        }

        let person = PersonGateway::find_by_id(db, person_id)
            .await
            .ok_or_else(|| BusinessError::new("Person not found".to_string()))?;

        let existing = TeamMemberGateway::find_membership(db, business_profile_id, person_id)
            .await
            .map_err(|e| {
                BusinessError::new(format!(
                    "Error checking existing team member request: {:?}",
                    e
                ))
            })?;

        if let Some(existing_request) = existing {
            let status = TeamMemberStatus::from_string(&existing_request.status);
            Self::reject_duplicated_request(&status)?;

            log::info!(
                "Previous request was '{}'. Reopening it for business profile {:?} and person {:?}.",
                existing_request.status,
                business_profile_id,
                person_id
            );
            let mut request = TeamMemberMapper::from_model(existing_request);
            request.status = TeamMemberStatus::Pending;
            return Self::persist_update(db, request, "reopening team member request").await;
        }

        let request = TeamMember::new(
            business_profile_id,
            uuid_to_string(business_profile.uuid),
            person_id,
            uuid_to_string(person.uuid),
            TeamMemberStatus::Pending,
        );

        let result = TeamMemberGateway::persist(db, request).await.map_err(|e| {
            log::error!("Error sending team member request: {:?}", e);
            BusinessError::new("Error sending team member request".to_string())
        })?;

        log::info!("Team member request sent successfully: {:?}", result);
        Ok(TeamMemberMapper::from_active_model(result))
    }

    /// An open request can only be created when no other one is already open or already honoured.
    fn reject_duplicated_request(status: &TeamMemberStatus) -> Result<(), BusinessError> {
        match status {
            TeamMemberStatus::Pending => Err(BusinessError::new(
                "A pending team member request already exists".to_string(),
            )),
            TeamMemberStatus::Accepted => Err(BusinessError::new(
                "This person is already a team member".to_string(),
            )),
            TeamMemberStatus::Rejected | TeamMemberStatus::Cancelled => Ok(()),
        }
    }

    pub async fn accept_team_member_request(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
    ) -> Result<TeamMember, BusinessError> {
        Self::update_team_member_request(
            db,
            business_profile_id,
            person_id,
            TeamMemberStatus::Accepted,
        )
        .await
    }

    pub async fn deny_team_member_request(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
    ) -> Result<TeamMember, BusinessError> {
        Self::update_team_member_request(
            db,
            business_profile_id,
            person_id,
            TeamMemberStatus::Rejected,
        )
        .await
    }

    pub async fn cancel_team_member_request(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
    ) -> Result<TeamMember, BusinessError> {
        Self::update_team_member_request(
            db,
            business_profile_id,
            person_id,
            TeamMemberStatus::Cancelled,
        )
        .await
    }

    async fn update_team_member_request(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
        status: TeamMemberStatus,
    ) -> Result<TeamMember, BusinessError> {
        log::info!(
            "Attempting to move team member request of business profile {:?} and person {:?} to {:?}",
            business_profile_id,
            person_id,
            status
        );

        let request = TeamMemberGateway::find_membership(db, business_profile_id, person_id)
            .await
            .map_err(|e| {
                log::error!("Error finding team member request: {:?}", e);
                BusinessError::new("Error finding team member request".to_string())
            })?
            .ok_or_else(|| {
                log::error!(
                    "Team member request not found for business profile {:?} and person {:?}",
                    business_profile_id,
                    person_id
                );
                BusinessError::new("Team member request not found".to_string())
            })?;

        Self::reject_closed_request(&TeamMemberStatus::from_string(&request.status), &status)?;

        let mut request = TeamMemberMapper::from_model(request);
        request.status = status;
        Self::persist_update(db, request, "updating team member request").await
    }

    /// Only a request still waiting on its answer can be answered — an accepted membership is
    /// revoked by cancelling it, never by re-answering a closed request.
    fn reject_closed_request(
        current: &TeamMemberStatus,
        next: &TeamMemberStatus,
    ) -> Result<(), BusinessError> {
        match (current, next) {
            (TeamMemberStatus::Pending, _) => Ok(()),
            (TeamMemberStatus::Accepted, TeamMemberStatus::Cancelled) => Ok(()),
            _ => Err(BusinessError::new(format!(
                "Team member request is already '{}'",
                current.as_str()
            ))),
        }
    }

    async fn persist_update(
        db: &DbConn,
        request: TeamMember,
        action: &str,
    ) -> Result<TeamMember, BusinessError> {
        let result = TeamMemberGateway::update(db, request).await.map_err(|e| {
            log::error!("Error {}: {:?}", action, e);
            BusinessError::new(format!("Error {}", action))
        })?;
        log::info!("Team member request updated successfully: {:?}", result);
        Ok(TeamMemberMapper::from_model(result))
    }

    pub async fn find_membership(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
    ) -> Result<TeamMember, BusinessError> {
        TeamMemberGateway::find_membership(db, business_profile_id, person_id)
            .await
            .map_err(|e| {
                log::error!("Error finding team member: {:?}", e);
                BusinessError::new("Error finding team member".to_string())
            })?
            .map(TeamMemberMapper::from_model)
            .ok_or_else(|| BusinessError::new("Team member not found".to_string()))
    }

    /// Persons of a business profile's team, by membership status.
    pub async fn find_all_persons(
        db: &DbConn,
        business_profile_id: i32,
        status: TeamMemberStatus,
    ) -> Vec<Person> {
        let memberships = TeamMemberGateway::find_all_by_business_profile_and_status(
            db,
            business_profile_id,
            status,
        )
        .await
        .unwrap_or_else(|e| {
            log::error!("Error finding team members: {:?}", e);
            Vec::new()
        });

        let person_ids: Vec<i32> = memberships.into_iter().map(|m| m.person_id).collect();
        PersonEntityMapper::from_models(PersonGateway::find_all_by_id_in(db, person_ids).await)
    }

    /// Business profiles a person belongs to (or was invited by), by membership status.
    pub async fn find_all_business_profiles(
        db: &DbConn,
        person_id: i32,
        status: TeamMemberStatus,
    ) -> Vec<BusinessProfile> {
        let memberships = TeamMemberGateway::find_all_by_person_and_status(db, person_id, status)
            .await
            .unwrap_or_else(|e| {
                log::error!("Error finding teams of person {:?}: {:?}", person_id, e);
                Vec::new()
            });

        let business_profile_ids: Vec<i32> = memberships
            .into_iter()
            .map(|m| m.business_profile_id)
            .collect();
        BusinessProfileEntityMapper::from_models(
            BusinessProfileGateway::find_all_by_ids(db, business_profile_ids).await,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TeamMemberUseCase;
    use crate::domain::team_member::TeamMemberStatus;

    #[test]
    fn open_and_honoured_requests_are_not_duplicated() {
        assert!(TeamMemberUseCase::reject_duplicated_request(&TeamMemberStatus::Pending).is_err());
        assert!(TeamMemberUseCase::reject_duplicated_request(&TeamMemberStatus::Accepted).is_err());
        assert!(TeamMemberUseCase::reject_duplicated_request(&TeamMemberStatus::Rejected).is_ok());
        assert!(TeamMemberUseCase::reject_duplicated_request(&TeamMemberStatus::Cancelled).is_ok());
    }

    #[test]
    fn only_pending_requests_can_be_answered() {
        assert!(TeamMemberUseCase::reject_closed_request(
            &TeamMemberStatus::Pending,
            &TeamMemberStatus::Accepted
        )
        .is_ok());
        assert!(TeamMemberUseCase::reject_closed_request(
            &TeamMemberStatus::Rejected,
            &TeamMemberStatus::Accepted
        )
        .is_err());
        assert!(TeamMemberUseCase::reject_closed_request(
            &TeamMemberStatus::Cancelled,
            &TeamMemberStatus::Accepted
        )
        .is_err());
    }

    #[test]
    fn an_accepted_membership_can_only_be_cancelled() {
        assert!(TeamMemberUseCase::reject_closed_request(
            &TeamMemberStatus::Accepted,
            &TeamMemberStatus::Cancelled
        )
        .is_ok());
        assert!(TeamMemberUseCase::reject_closed_request(
            &TeamMemberStatus::Accepted,
            &TeamMemberStatus::Accepted
        )
        .is_err());
    }
}
