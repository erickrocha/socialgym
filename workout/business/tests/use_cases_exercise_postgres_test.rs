mod support;

use business::domain::business_error::BusinessErrorKind as K;
use business::domain::exercise::Visibility;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use business::use_cases::friend_use_case::FriendUseCase;
use support::{acting, exercise, fresh_db, kind, register};

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn exercise_ownership_audience_and_lookup_rules() {
    let db = fresh_db().await;
    let (owner, friend, stranger) = (register(&db, 1).await, register(&db, 2).await, register(&db, 3).await);

    // create: the owner always comes from the acting identity, never from the payload
    let mut spoofed = exercise("Squat", Visibility::Private);
    spoofed.owner_id = stranger.person_id;
    spoofed.owner_uuid = stranger.person_uuid.clone();
    let private = ExerciseUseCase::persist(&db, spoofed, &owner, None).await.unwrap();
    assert_eq!((private.owner_id, private.owner_uuid.as_str()), (owner.person_id, owner.person_uuid.as_str()));
    let id = private.id.unwrap();

    // validation: description is bounded
    let mut long = exercise("Long", Visibility::Private);
    long.description = Some("x".repeat(256));
    assert!(matches!(kind(&ExerciseUseCase::persist(&db, long, &owner, None).await.unwrap_err()), K::Validation));

    // update: only the owner
    let mut edit = private.clone();
    edit.name = "Back Squat".into();
    assert!(matches!(kind(&ExerciseUseCase::persist(&db, edit.clone(), &stranger, None).await.unwrap_err()), K::Forbidden));
    assert_eq!(ExerciseUseCase::persist(&db, edit, &owner, None).await.unwrap().name, "Back Squat");

    // audience: private is owner-only and reported as not found to others
    assert!(ExerciseUseCase::is_readable(&db, &private, &acting(&owner)).await.unwrap());
    assert!(!ExerciseUseCase::is_readable(&db, &private, &acting(&stranger)).await.unwrap());
    assert!(matches!(kind(&ExerciseUseCase::ensure_readable(&db, &private, &acting(&stranger)).await.unwrap_err()), K::NotFound));
    let public = ExerciseUseCase::persist(&db, exercise("Run", Visibility::Public), &owner, None).await.unwrap();
    let friends_only = ExerciseUseCase::persist(&db, exercise("Row", Visibility::Friends), &owner, None).await.unwrap();
    assert!(ExerciseUseCase::is_readable(&db, &public, &acting(&stranger)).await.unwrap());
    assert!(!ExerciseUseCase::is_readable(&db, &friends_only, &acting(&friend)).await.unwrap(), "not friends yet");

    FriendUseCase::send_friend_request(&db, friend.person_id, owner.person_id).await.unwrap();
    FriendUseCase::accept_friend_request(&db, owner.person_id, friend.person_id).await.unwrap();
    assert!(ExerciseUseCase::is_readable(&db, &friends_only, &acting(&friend)).await.unwrap());
    assert!(!ExerciseUseCase::is_readable(&db, &friends_only, &acting(&stranger)).await.unwrap());
    assert!(!ExerciseUseCase::is_readable(&db, &private, &acting(&friend)).await.unwrap(), "friendship does not open private exercises");

    let all = vec![private.clone(), public.clone(), friends_only.clone()];
    let visible = ExerciseUseCase::retain_readable(&db, all.clone(), &acting(&friend)).await.unwrap();
    assert_eq!(visible.len(), 2);
    assert!(ExerciseUseCase::ensure_all_readable(&db, &all, &acting(&friend)).await.is_err());
    ExerciseUseCase::ensure_all_readable(&db, &visible, &acting(&friend)).await.unwrap();

    // lookups
    assert_eq!(ExerciseUseCase::get(&db, id).await.unwrap().uuid, private.uuid);
    assert_eq!(ExerciseUseCase::get_by_uuid(&db, private.uuid.clone().unwrap()).await.unwrap().id, Some(id));
    assert!(matches!(kind(&ExerciseUseCase::get(&db, 9999).await.unwrap_err()), K::NotFound));
    assert!(matches!(kind(&ExerciseUseCase::get_by_uuid(&db, "00000000-0000-0000-0000-00000000dead".into()).await.unwrap_err()), K::NotFound));

    // by visibility
    assert_eq!(ExerciseUseCase::find_by_visibility(&db, Visibility::Private, owner.person_id).await.unwrap().len(), 3, "owner's own exercises");
    assert_eq!(ExerciseUseCase::find_by_visibility(&db, Visibility::Public, stranger.person_id).await.unwrap().len(), 1);
    assert_eq!(ExerciseUseCase::find_by_visibility(&db, Visibility::Friends, friend.person_id).await.unwrap().len(), 1);
    assert!(ExerciseUseCase::find_by_visibility(&db, Visibility::Friends, stranger.person_id).await.unwrap().is_empty(), "no friends -> nothing");
    assert!(ExerciseUseCase::find_by_visibility(&db, Visibility::Professional, owner.person_id).await.unwrap().is_empty());

    // paginated search as the friend: own none, owner's public + friends-only exercises
    let (page, total, more) = ExerciseUseCase::find_by_complex_filters_paginated(&db, &acting(&friend), vec![owner.person_id], None, None, 1, 1, None).await.unwrap();
    assert_eq!((page.len(), total, more), (1, 2, true));
    let (page2, _, more2) = ExerciseUseCase::find_by_complex_filters_paginated(&db, &acting(&friend), vec![owner.person_id], None, None, 2, 1, None).await.unwrap();
    assert_eq!((page2.len(), more2), (1, false));
    let (none, total0, _) = ExerciseUseCase::find_by_complex_filters_paginated(&db, &acting(&stranger), vec![9999], None, Some("Private".into()), 1, 10, None).await.unwrap();
    assert_eq!((none.len(), total0), (0, 0), "unknown owners are ignored and private is never exposed");

    // delete: owner only; missing is not found
    assert!(matches!(kind(&ExerciseUseCase::delete_by_id(&db, id, &acting(&stranger)).await.unwrap_err()), K::Forbidden));
    assert!(matches!(kind(&ExerciseUseCase::delete_by_uuid(&db, public.uuid.clone().unwrap(), &acting(&stranger)).await.unwrap_err()), K::Forbidden));
    ExerciseUseCase::delete_by_id(&db, id, &acting(&owner)).await.unwrap();
    ExerciseUseCase::delete_by_uuid(&db, public.uuid.clone().unwrap(), &acting(&owner)).await.unwrap();
    assert!(matches!(kind(&ExerciseUseCase::delete_by_id(&db, id, &acting(&owner)).await.unwrap_err()), K::NotFound));
    assert!(matches!(kind(&ExerciseUseCase::delete_by_uuid(&db, public.uuid.clone().unwrap(), &acting(&owner)).await.unwrap_err()), K::NotFound));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn exercises_are_composed_into_workouts_with_order_and_readability_rules() {
    use business::use_cases::workout_use_case::WorkoutUseCase;
    let db = fresh_db().await;
    let (owner, stranger) = (register(&db, 1).await, register(&db, 2).await);
    let workout = WorkoutUseCase::persist(&db, support::workout("Base", Visibility::Private, vec![]), &owner, None, None).await.unwrap();
    let wid = workout.id.unwrap();

    // nothing to add is a no-op
    assert!(ExerciseUseCase::add_all_to_workout(&db, wid, vec![], &owner, None).await.unwrap().is_empty());
    assert!(ExerciseUseCase::find_all_by_workout_id(&db, wid).await.unwrap().is_empty());

    // new entries are created for the acting identity; existing ones are reused
    let mine = ExerciseUseCase::persist(&db, exercise("Mine", Visibility::Private), &owner, None).await.unwrap();
    let added = ExerciseUseCase::add_all_to_workout(&db, wid, vec![exercise("Fresh", Visibility::Private), mine.clone()], &owner, None).await.unwrap();
    assert_eq!(added.len(), 2);
    assert_eq!(added[0].owner_id, owner.person_id);
    assert_eq!(added[1].id, mine.id);
    assert_eq!(ExerciseUseCase::find_all_by_workout_id(&db, wid).await.unwrap().len(), 2);

    // somebody else's private exercise cannot be pulled in (reported as not found); an unknown id is not found
    let err = ExerciseUseCase::add_all_to_workout(&db, wid, vec![mine.clone()], &stranger, None).await.unwrap_err();
    assert!(matches!(kind(&err), K::NotFound));
    let mut ghost = mine.clone();
    ghost.id = Some(9999);
    assert!(matches!(kind(&ExerciseUseCase::add_all_to_workout(&db, wid, vec![ghost], &owner, None).await.unwrap_err()), K::NotFound));

    // single add / remove, and a duplicate association is rejected
    let extra = ExerciseUseCase::persist(&db, exercise("Extra", Visibility::Private), &owner, None).await.unwrap();
    let link = ExerciseUseCase::add_exercise_to_workout(&db, wid, extra.id.unwrap(), 5).await.unwrap();
    assert_eq!((link.workout_id, link.exercise_id, link.order_index), (wid, extra.id.unwrap(), 5));
    assert!(ExerciseUseCase::add_exercise_to_workout(&db, wid, extra.id.unwrap(), 6).await.is_err());
    assert_eq!(ExerciseUseCase::find_all_by_workout_id(&db, wid).await.unwrap().len(), 3);
    ExerciseUseCase::remove_exercise_from_workout(&db, wid, extra.id.unwrap()).await.unwrap();
    assert_eq!(ExerciseUseCase::find_all_by_workout_id(&db, wid).await.unwrap().len(), 2);
}
