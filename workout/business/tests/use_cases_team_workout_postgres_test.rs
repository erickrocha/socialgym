mod support;

use business::domain::business_error::BusinessErrorKind as K;
use business::domain::enums::{InviteStatus, ProfileType, Visibility};
use business::use_cases::team_member_use_case::TeamMemberUseCase;
use business::use_cases::workout_use_case::WorkoutUseCase;
use support::{acting, business_profile, exercise, fresh_db, kind, register, workout};

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn team_membership_requires_the_persons_consent_and_follows_the_invite_lifecycle() {
    let db = fresh_db().await;
    let (owner, member, other) = (
        register(&db, 1).await,
        register(&db, 2).await,
        register(&db, 3).await,
    );
    let profile = business_profile(&db, &owner, "Gym").await;
    let pid = profile.id.unwrap();

    // invalid invites
    assert!(
        TeamMemberUseCase::send_team_member_request(&db, 9999, member.person_id)
            .await
            .is_err(),
        "unknown profile"
    );
    assert!(
        TeamMemberUseCase::send_team_member_request(&db, pid, owner.person_id)
            .await
            .is_err(),
        "owner cannot join own team"
    );
    assert!(
        TeamMemberUseCase::send_team_member_request(&db, pid, 9999)
            .await
            .is_err(),
        "unknown person"
    );
    assert!(
        TeamMemberUseCase::find_membership(&db, pid, member.person_id)
            .await
            .is_err()
    );
    assert!(
        TeamMemberUseCase::accept_team_member_request(&db, pid, member.person_id)
            .await
            .is_err(),
        "nothing to accept"
    );

    // invite -> pending; a duplicate pending invite is rejected; no authority until accepted
    let invite = TeamMemberUseCase::send_team_member_request(&db, pid, member.person_id)
        .await
        .unwrap();
    assert_eq!(invite.status, InviteStatus::Pending);
    assert!(
        TeamMemberUseCase::send_team_member_request(&db, pid, member.person_id)
            .await
            .is_err()
    );
    assert!(matches!(
        kind(
            &TeamMemberUseCase::ensure_accepted_member(&db, pid, member.person_id)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert_eq!(
        TeamMemberUseCase::find_all_persons(&db, pid, InviteStatus::Pending)
            .await
            .len(),
        1
    );
    assert_eq!(
        TeamMemberUseCase::find_all_business_profiles(&db, member.person_id, InviteStatus::Pending)
            .await
            .len(),
        1
    );

    // accept -> member; closed requests cannot be re-answered
    assert_eq!(
        TeamMemberUseCase::accept_team_member_request(&db, pid, member.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Accepted
    );
    TeamMemberUseCase::ensure_accepted_member(&db, pid, member.person_id)
        .await
        .unwrap();
    assert!(
        TeamMemberUseCase::deny_team_member_request(&db, pid, member.person_id)
            .await
            .is_err()
    );
    assert!(
        TeamMemberUseCase::send_team_member_request(&db, pid, member.person_id)
            .await
            .is_err(),
        "already a member"
    );
    assert_eq!(
        TeamMemberUseCase::find_membership(&db, pid, member.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Accepted
    );

    // roster read model
    let roster = TeamMemberUseCase::find_roster(&db, profile.uuid.as_deref().unwrap())
        .await
        .unwrap();
    assert_eq!(roster.owner_person_uuid, owner.person_uuid);
    assert_eq!(
        roster.accepted_member_person_uuids,
        vec![member.person_uuid.clone()]
    );
    assert_eq!(roster.business_profile_name, "Gym");
    assert!(
        TeamMemberUseCase::find_roster(&db, "00000000-0000-0000-0000-00000000dead")
            .await
            .is_err()
    );

    // an accepted membership is revoked by cancelling it
    assert_eq!(
        TeamMemberUseCase::cancel_team_member_request(&db, pid, member.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Cancelled
    );
    assert!(matches!(
        kind(
            &TeamMemberUseCase::ensure_accepted_member(&db, pid, member.person_id)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(
        TeamMemberUseCase::find_roster(&db, profile.uuid.as_deref().unwrap())
            .await
            .unwrap()
            .accepted_member_person_uuids
            .is_empty()
    );

    // a denied or cancelled invite can be sent again
    TeamMemberUseCase::send_team_member_request(&db, pid, other.person_id)
        .await
        .unwrap();
    assert_eq!(
        TeamMemberUseCase::deny_team_member_request(&db, pid, other.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Rejected
    );
    assert_eq!(
        TeamMemberUseCase::send_team_member_request(&db, pid, other.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Pending
    );
    assert_eq!(
        TeamMemberUseCase::send_team_member_request(&db, pid, member.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Pending,
        "re-invite after cancel"
    );
    let _ = ProfileType::Company;
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn workout_ownership_audience_and_assignment_rules() {
    let db = fresh_db().await;
    let (owner, member, stranger) = (
        register(&db, 1).await,
        register(&db, 2).await,
        register(&db, 3).await,
    );
    let profile = business_profile(&db, &owner, "Gym").await;
    let pid = profile.id.unwrap();

    // create: owner comes from the acting identity; the composed new exercise is owned by the actor
    let mut spoofed = workout(
        "Legs",
        Visibility::Private,
        vec![exercise("Squat", Visibility::Private)],
    );
    spoofed.owner_id = stranger.person_id;
    spoofed.owner_uuid = stranger.person_uuid.clone();
    let created = WorkoutUseCase::persist(&db, spoofed, &owner, None, None)
        .await
        .unwrap();
    assert_eq!(
        (created.owner_id, created.status),
        (owner.person_id, InviteStatus::Accepted)
    );
    let id = created.id.unwrap();
    let loaded = WorkoutUseCase::get(&db, id).await.unwrap();
    assert_eq!(loaded.exercises.len(), 1);
    assert_eq!(loaded.exercises[0].owner_id, owner.person_id);
    assert_eq!(
        WorkoutUseCase::get_by_uuid(&db, loaded.uuid.clone().unwrap())
            .await
            .unwrap()
            .id,
        Some(id)
    );
    assert!(matches!(
        kind(&WorkoutUseCase::get(&db, 9999).await.unwrap_err()),
        K::NotFound
    ));
    assert!(matches!(
        kind(
            &WorkoutUseCase::get_by_uuid(&db, "00000000-0000-0000-0000-00000000dead".into())
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));

    // validation
    let mut long = workout("Long", Visibility::Private, vec![]);
    long.description = Some("x".repeat(5001));
    assert!(matches!(
        kind(
            &WorkoutUseCase::persist(&db, long, &owner, None, None)
                .await
                .unwrap_err()
        ),
        K::Validation
    ));

    // update: only the owner; a stranger cannot read the private workout, so for them it does not exist
    let mut edit = loaded.clone();
    edit.name = "Leg Day".into();
    assert!(matches!(
        kind(
            &WorkoutUseCase::persist(&db, edit.clone(), &stranger, None, None)
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    // an edit re-sends the exercises the workout already has: they are kept once, new ones are appended
    edit.exercises.push(exercise("Lunge", Visibility::Private));
    assert_eq!(
        WorkoutUseCase::persist(&db, edit.clone(), &owner, None, None)
            .await
            .unwrap()
            .name,
        "Leg Day"
    );
    let after_edit = WorkoutUseCase::get(&db, id).await.unwrap();
    assert_eq!(after_edit.name, "Leg Day");
    assert_eq!(
        after_edit
            .exercises
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>(),
        vec!["Squat", "Lunge"]
    );
    assert_eq!(
        WorkoutUseCase::persist(&db, edit.clone(), &owner, None, None)
            .await
            .unwrap()
            .name,
        "Leg Day",
        "saving the same edit twice is idempotent"
    );
    assert_eq!(
        WorkoutUseCase::get(&db, id).await.unwrap().exercises.len(),
        3,
        "the new Lunge was created again by the second save, the existing ones were not duplicated"
    );

    // an existing exercise can only be composed by someone allowed to read it
    let others_private = support::exercise("Secret", Visibility::Private);
    let secret = business::use_cases::exercise_use_case::ExerciseUseCase::persist(
        &db,
        others_private,
        &owner,
        None,
    )
    .await
    .unwrap();
    let steal = workout("Steal", Visibility::Private, vec![secret.clone()]);
    assert!(
        WorkoutUseCase::persist(&db, steal, &stranger, None, None)
            .await
            .is_err(),
        "private exercise of another person cannot be pulled in"
    );

    // assignment rules
    let target = member.person_uuid.as_str();
    assert!(
        matches!(
            kind(
                &WorkoutUseCase::persist(&db, edit, &owner, Some(&profile), Some(target))
                    .await
                    .unwrap_err()
            ),
            K::Validation
        ),
        "existing workout cannot be assigned"
    );
    assert!(
        matches!(
            kind(
                &WorkoutUseCase::persist(
                    &db,
                    workout("A", Visibility::Private, vec![]),
                    &owner,
                    None,
                    Some(target)
                )
                .await
                .unwrap_err()
            ),
            K::Validation
        ),
        "needs a business profile"
    );
    assert!(matches!(
        kind(
            &WorkoutUseCase::persist(
                &db,
                workout("A", Visibility::Private, vec![]),
                &owner,
                Some(&profile),
                Some("00000000-0000-0000-0000-00000000dead")
            )
            .await
            .unwrap_err()
        ),
        K::NotFound
    ));
    assert!(
        matches!(
            kind(
                &WorkoutUseCase::persist(
                    &db,
                    workout("A", Visibility::Private, vec![]),
                    &owner,
                    Some(&profile),
                    Some(target)
                )
                .await
                .unwrap_err()
            ),
            K::Forbidden
        ),
        "not a team member yet"
    );

    TeamMemberUseCase::send_team_member_request(&db, pid, member.person_id)
        .await
        .unwrap();
    TeamMemberUseCase::accept_team_member_request(&db, pid, member.person_id)
        .await
        .unwrap();
    let assigned = WorkoutUseCase::persist(
        &db,
        workout(
            "Plan",
            Visibility::Private,
            vec![exercise("Row", Visibility::Private)],
        ),
        &owner,
        Some(&profile),
        Some(target),
    )
    .await
    .unwrap();
    assert_eq!(
        (
            assigned.owner_id,
            assigned.status,
            assigned.assigned_by_profile_id
        ),
        (member.person_id, InviteStatus::Pending, Some(pid))
    );
    let uuid = assigned.uuid.clone().unwrap();

    // a pending assignment cannot be edited; only the assignee answers; only the assigner cancels
    let mut pending_edit = WorkoutUseCase::get_by_uuid(&db, uuid.clone())
        .await
        .unwrap();
    pending_edit.name = "x".into();
    assert!(matches!(
        kind(
            &WorkoutUseCase::persist(&db, pending_edit, &member, None, None)
                .await
                .unwrap_err()
        ),
        K::Validation
    ));
    assert!(matches!(
        kind(
            &WorkoutUseCase::accept_assignment(&db, uuid.clone(), stranger.person_id)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &WorkoutUseCase::cancel_assignment(&db, uuid.clone(), pid + 100)
                .await
                .unwrap_err()
        ),
        K::Forbidden
    ));
    assert!(matches!(
        kind(
            &WorkoutUseCase::accept_assignment(
                &db,
                "00000000-0000-0000-0000-00000000dead".into(),
                member.person_id
            )
            .await
            .unwrap_err()
        ),
        K::NotFound
    ));
    assert_eq!(
        WorkoutUseCase::find_all_assigned_by_profile(&db, pid)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        WorkoutUseCase::accept_assignment(&db, uuid.clone(), member.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Accepted
    );
    assert!(
        matches!(
            kind(
                &WorkoutUseCase::reject_assignment(&db, uuid.clone(), member.person_id)
                    .await
                    .unwrap_err()
            ),
            K::Validation
        ),
        "no longer pending"
    );

    let second = WorkoutUseCase::persist(
        &db,
        workout("Plan 2", Visibility::Private, vec![]),
        &owner,
        Some(&profile),
        Some(target),
    )
    .await
    .unwrap();
    assert_eq!(
        WorkoutUseCase::reject_assignment(&db, second.uuid.clone().unwrap(), member.person_id)
            .await
            .unwrap()
            .status,
        InviteStatus::Rejected
    );
    let third = WorkoutUseCase::persist(
        &db,
        workout("Plan 3", Visibility::Private, vec![]),
        &owner,
        Some(&profile),
        Some(target),
    )
    .await
    .unwrap();
    assert_eq!(
        WorkoutUseCase::cancel_assignment(&db, third.uuid.clone().unwrap(), pid)
            .await
            .unwrap()
            .status,
        InviteStatus::Cancelled
    );
    assert_eq!(
        WorkoutUseCase::find_all_assigned_by_profile(&db, pid)
            .await
            .unwrap()
            .len(),
        3
    );

    // audience: a public workout hides the owner's private exercises from others
    let public = WorkoutUseCase::persist(
        &db,
        workout(
            "Open",
            Visibility::Public,
            vec![secret.clone(), exercise("Plank", Visibility::Public)],
        ),
        &owner,
        None,
        None,
    )
    .await
    .unwrap();
    let seen = WorkoutUseCase::find_readable_by_person_id(&db, owner.person_id, &acting(&stranger))
        .await
        .unwrap();
    assert_eq!(seen.len(), 1, "only the public workout is readable");
    assert_eq!(
        seen[0].exercises.len(),
        1,
        "the private exercise is redacted"
    );
    assert_eq!(seen[0].exercises[0].name, "Plank");
    assert!(
        WorkoutUseCase::find_readable_by_person_id(&db, 9999, &acting(&stranger))
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        WorkoutUseCase::find_all_by_owner_id(&db, owner.person_id)
            .await
            .unwrap()
            .len(),
        2
    );
    assert!(
        WorkoutUseCase::ensure_readable(&db, &loaded, &acting(&stranger))
            .await
            .is_err()
    );
    WorkoutUseCase::ensure_readable(&db, &public, &acting(&stranger))
        .await
        .unwrap();
    assert!(WorkoutUseCase::ensure_owner_uuid(&owner.person_uuid, &owner.person_uuid).is_ok());
    assert!(WorkoutUseCase::ensure_owner_uuid(&owner.person_uuid, &stranger.person_uuid).is_err());

    // delete: owner only. The private workout does not exist for a stranger (C-010 decision 2026-10-07); a reader
    // who is not the owner is told they may not.
    assert!(matches!(
        kind(
            &WorkoutUseCase::delete_by_id(&db, id, &acting(&stranger))
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    assert!(matches!(
        kind(
            &WorkoutUseCase::delete_by_uuid(&db, uuid.clone(), &acting(&stranger))
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    let public_uuid = public.uuid.clone().expect("saved workout has a uuid");
    assert!(
        matches!(
            kind(
                &WorkoutUseCase::delete_by_uuid(&db, public_uuid, &acting(&stranger))
                    .await
                    .unwrap_err()
            ),
            K::Forbidden
        ),
        "a public workout can be read, so the refusal says so"
    );
    WorkoutUseCase::delete_by_id(&db, id, &acting(&owner))
        .await
        .unwrap();
    WorkoutUseCase::delete_by_uuid(&db, uuid.clone(), &acting(&member))
        .await
        .unwrap();
    assert!(matches!(
        kind(
            &WorkoutUseCase::delete_by_id(&db, id, &acting(&owner))
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
    assert!(matches!(
        kind(
            &WorkoutUseCase::delete_by_uuid(&db, uuid, &acting(&member))
                .await
                .unwrap_err()
        ),
        K::NotFound
    ));
}
