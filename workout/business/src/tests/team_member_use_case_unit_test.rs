use super::TeamMemberUseCase;
use crate::domain::enums::InviteStatus;

#[test]
fn open_and_honoured_requests_are_not_duplicated() {
    assert!(TeamMemberUseCase::reject_duplicated_request(&InviteStatus::Pending).is_err());
    assert!(TeamMemberUseCase::reject_duplicated_request(&InviteStatus::Accepted).is_err());
    assert!(TeamMemberUseCase::reject_duplicated_request(&InviteStatus::Rejected).is_ok());
    assert!(TeamMemberUseCase::reject_duplicated_request(&InviteStatus::Cancelled).is_ok());
}

#[test]
fn only_pending_requests_can_be_answered() {
    assert!(TeamMemberUseCase::reject_closed_request(
        &InviteStatus::Pending,
        &InviteStatus::Accepted
    )
    .is_ok());
    assert!(TeamMemberUseCase::reject_closed_request(
        &InviteStatus::Rejected,
        &InviteStatus::Accepted
    )
    .is_err());
    assert!(TeamMemberUseCase::reject_closed_request(
        &InviteStatus::Cancelled,
        &InviteStatus::Accepted
    )
    .is_err());
}

#[test]
fn an_accepted_membership_can_only_be_cancelled() {
    assert!(TeamMemberUseCase::reject_closed_request(
        &InviteStatus::Accepted,
        &InviteStatus::Cancelled
    )
    .is_ok());
    assert!(TeamMemberUseCase::reject_closed_request(
        &InviteStatus::Accepted,
        &InviteStatus::Accepted
    )
    .is_err());
}
