mod support;

use business::gateway::content_report_gateway::ContentReportGateway;
use business::gateway::evolution_check_in_gateway::{EvolutionCheckInGateway, EvolutionCheckInGatewayPort};
use business::gateway::post_gateway::PostGateway;
use business::use_cases::account_data_deletion_use_case::AccountDataDeletionUseCase;
use business::use_cases::content_report_use_case::ContentReportUseCase;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use business::use_cases::post_use_case::PostUseCase;
use business::use_cases::workout_use_case::WorkoutSessionUseCase;
use domain::business_error::BusinessErrorKind;
use domain::comment::Comment;
use domain::enums::{ReactionType, Visibility};
use domain::evolution_check_in::EvolutionCheckIn;
use domain::in_app_notification::InAppNotification;
use domain::mention_notification_event::MentionNotificationEvent;
use domain::post::Post;
use domain::reaction::Reaction;
use domain::user::User;
use domain::workout_session::WorkoutSession;
use mongodb::bson::{DateTime, doc, from_document};
use mongodb::Database;

fn user(n: i32) -> User {
    User::new(format!("Person {n}"), format!("p{n}@t.test"), format!("u{n}"), n, format!("person-{n}"), String::new(), None)
}

async fn seed_post(db: &Database, author: &User, text: &str) -> Post {
    PostUseCase::create(db, author, Post::new(0, String::new(), String::new(), None, None, text.into(), vec![], vec![]))
        .await
        .unwrap()
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn content_report_validates_persists_and_resolves_moderation() {
    let db = support::database().await;
    let stub = support::start_stub().await;
    let (author, reporter) = (user(1), user(2));
    let post = seed_post(&db, &author, "hello").await;
    let commented = PostUseCase::add_comment(
        &db, &reporter, post.uuid.clone(),
        Comment::new("c1".into(), post.uuid.clone(), String::new(), String::new(), None, None, "rude".into(), None, vec![]),
    ).await.unwrap();
    assert_eq!(commented.comments.len(), 1);

    // create: validation, missing post, success
    for (tt, tid, pid, reason) in [("video", "x", post.uuid.as_str(), "spam"), ("post", "", post.uuid.as_str(), "spam"), ("post", "x", "", "spam"), ("post", "x", post.uuid.as_str(), "")] {
        let e = ContentReportUseCase::create(&db, "person-2", tt.into(), tid.into(), pid.into(), reason.into(), None).await.unwrap_err();
        assert!(matches!(e.kind, BusinessErrorKind::Validation), "{tt}/{tid}/{pid}/{reason}");
    }
    let e = ContentReportUseCase::create(&db, "person-2", "post".into(), "x".into(), "missing".into(), "spam".into(), None).await.unwrap_err();
    assert!(matches!(e.kind, BusinessErrorKind::NotFound));
    let on_comment = ContentReportUseCase::create(&db, "person-2", "comment".into(), "c1".into(), post.uuid.clone(), "abuse".into(), Some("details".into())).await.unwrap();
    let on_post = ContentReportUseCase::create(&db, "person-3", "post".into(), post.uuid.clone(), post.uuid.clone(), "spam".into(), None).await.unwrap();
    assert_ne!(on_comment.status, "resolved");

    // moderator role is required to list and decide
    assert!(matches!(ContentReportUseCase::list(&db, None).await.unwrap_err().kind, BusinessErrorKind::Forbidden));
    assert!(matches!(ContentReportUseCase::decide(&db, &on_comment.uuid, "mod", "removed", "why").await.unwrap_err().kind, BusinessErrorKind::Forbidden));
    *stub.moderator.lock().unwrap() = true;
    assert_eq!(ContentReportUseCase::list(&db, None).await.unwrap().len(), 2);
    assert_eq!(ContentReportUseCase::list(&db, Some("resolved")).await.unwrap().len(), 0);

    // decide: invalid decision/reason, unknown report
    for (decision, reason) in [("banned", "why"), ("removed", "  ")] {
        let e = ContentReportUseCase::decide(&db, &on_comment.uuid, "mod", decision, reason).await.unwrap_err();
        assert!(matches!(e.kind, BusinessErrorKind::Validation));
    }
    let e = ContentReportUseCase::decide(&db, "nope", "mod", "removed", "why").await.unwrap_err();
    assert!(matches!(e.kind, BusinessErrorKind::NotFound));

    // removing a comment pulls it from the post and records history
    let resolved = ContentReportUseCase::decide(&db, &on_comment.uuid, "mod-1", "removed", "abusive").await.unwrap();
    assert_eq!((resolved.status.as_str(), resolved.decision.as_deref()), ("resolved", Some("removed")));
    assert_eq!(resolved.assigned_moderator_uuid.as_deref(), Some("mod-1"));
    assert_eq!(resolved.history.len(), 1);
    assert!(PostGateway::new(&db).find_by_id_result(&post.uuid).await.unwrap().unwrap().comments.is_empty());
    assert_eq!(ContentReportUseCase::list(&db, Some("resolved")).await.unwrap().len(), 1);

    // dismissing keeps content; removing a post deletes it
    let dismissed = ContentReportUseCase::decide(&db, &on_post.uuid, "mod-1", "dismissed", "fine").await.unwrap();
    assert_eq!(dismissed.decision.as_deref(), Some("dismissed"));
    assert!(PostGateway::new(&db).find_by_id_result(&post.uuid).await.unwrap().is_some());
    let on_post2 = ContentReportUseCase::create(&db, "person-3", "post".into(), post.uuid.clone(), post.uuid.clone(), "spam".into(), None).await.unwrap();
    ContentReportUseCase::decide(&db, &on_post2.uuid, "mod-1", "removed", "spam").await.unwrap();
    assert!(PostGateway::new(&db).find_by_id_result(&post.uuid).await.unwrap().is_none());
    assert!(ContentReportGateway::new(&db).find(&on_post2.uuid).await.unwrap().is_some());
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn workout_session_use_case_scopes_sessions_to_their_owner() {
    let db = support::database().await;
    let session = |id: &str, started: i64| -> WorkoutSession {
        from_document(doc! { "_id": id, "personUuid": "spoofed", "duration": 30, "startedAt": DateTime::from_millis(started), "totalVolume": 1.0_f64, "totalSets": 1 }).unwrap()
    };
    let saved = WorkoutSessionUseCase::add(&db, session("w1", 1_000), "person-1").await.unwrap();
    assert_eq!(saved.person_uuid.as_deref(), Some("person-1"), "body personUuid is overwritten");
    WorkoutSessionUseCase::add(&db, session("w2", 50_000), "person-1").await.unwrap();
    assert!(WorkoutSessionUseCase::add(&db, session("w1", 1_000), "person-1").await.is_err(), "duplicate id fails");

    assert_eq!(WorkoutSessionUseCase::find_by_id(&db, "w1".into(), "person-1").await.unwrap().uuid, "w1");
    let e = WorkoutSessionUseCase::find_by_id(&db, "w1".into(), "person-2").await.unwrap_err();
    assert!(matches!(e.kind, BusinessErrorKind::Forbidden));
    let e = WorkoutSessionUseCase::find_by_id(&db, "missing".into(), "person-1").await.unwrap_err();
    assert!(matches!(e.kind, BusinessErrorKind::NotFound));

    let all = WorkoutSessionUseCase::find_all_by_person(&db, "person-1".into(), DateTime::from_millis(0), DateTime::from_millis(100_000)).await;
    assert_eq!(all.len(), 2);
    let narrow = WorkoutSessionUseCase::find_all_by_person(&db, "person-1".into(), DateTime::from_millis(0), DateTime::from_millis(10_000)).await;
    assert_eq!(narrow.len(), 1);
    assert!(WorkoutSessionUseCase::find_all_by_person(&db, "person-2".into(), DateTime::from_millis(0), DateTime::from_millis(100_000)).await.is_empty());
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn account_deletion_removes_every_record_of_the_person_and_keeps_the_rest() {
    let db = support::database().await;
    let _stub = support::start_stub().await;
    let (a, b) = (user(1), user(2));
    let post_a = seed_post(&db, &a, "from a").await;
    let post_b = seed_post(&db, &b, "from b").await;
    PostUseCase::add_comment(&db, &b, post_a.uuid.clone(), Comment::new("cb".into(), post_a.uuid.clone(), String::new(), String::new(), None, None, "b says".into(), None, vec![])).await.unwrap();
    PostUseCase::add_comment(&db, &a, post_a.uuid.clone(), Comment::new("ca".into(), post_a.uuid.clone(), String::new(), String::new(), None, None, "a says".into(), None, vec![])).await.unwrap();
    PostUseCase::add_reaction(&db, &b, post_a.uuid.clone(), Reaction::new("r".into(), String::new(), String::new(), ReactionType::Like)).await.unwrap();
    EvolutionCheckInGateway::new(&db)
        .persist_check_in(EvolutionCheckIn::new("e1".into(), "person-2".into(), DateTime::from_millis(5), None, Visibility::Private, None, None))
        .await.unwrap();
    ContentReportUseCase::create(&db, "person-2", "post".into(), post_a.uuid.clone(), post_a.uuid.clone(), "spam".into(), None).await.unwrap();

    AccountDataDeletionUseCase::delete_all_for_person(&db, "person-2").await.unwrap();

    assert!(PostGateway::new(&db).find_by_id_result(&post_b.uuid).await.unwrap().is_none(), "own post deleted");
    let kept = PostGateway::new(&db).find_by_id_result(&post_a.uuid).await.unwrap().unwrap();
    assert_eq!(kept.comments.iter().map(|c| c.author_uuid.as_str()).collect::<Vec<_>>(), vec!["person-1"]);
    assert!(kept.reactions.is_empty(), "reaction left on another post is pulled");
    assert!(EvolutionCheckInGateway::new(&db).find_all_by_person_uuid("person-2".into(), DateTime::from_millis(0), DateTime::from_millis(10)).await.is_empty());
    assert!(ContentReportUseCase::list(&db, None).await.is_err(), "no moderator -> forbidden, proves reports not listable");
    assert!(ContentReportGateway::new(&db).list(None).await.unwrap().is_empty(), "reports filed by the person are deleted");
    // idempotent
    AccountDataDeletionUseCase::delete_all_for_person(&db, "person-2").await.unwrap();
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn mention_notification_use_case_enqueues_processes_and_scopes_reads() {
    let db = support::database().await;
    MentionNotificationUseCase::enqueue(&db, vec![]).await.unwrap();
    assert_eq!(MentionNotificationUseCase::process_pending(&db, 10).await.unwrap(), 0);

    let event = |id: &str| MentionNotificationEvent::new(id.into(), "post".into(), "p1".into(), Some("p1".into()), None, 1, "person-1".into(), "Person 1".into(), "person-2".into(), "hi".into());
    MentionNotificationUseCase::enqueue(&db, vec![event("m1"), event("m2")]).await.unwrap();
    assert_eq!(MentionNotificationUseCase::process_pending(&db, 10).await.unwrap(), 2);
    assert_eq!(MentionNotificationUseCase::process_pending(&db, 10).await.unwrap(), 0, "processed events are not picked up again");

    let list = MentionNotificationUseCase::list_notifications(&db, "person-2", true, 10).await.unwrap();
    assert_eq!(list.len(), 2);
    assert!(MentionNotificationUseCase::list_notifications(&db, "person-3", false, 10).await.unwrap().is_empty());
    assert!(MentionNotificationUseCase::mark_as_read(&db, "person-2", "m1").await.unwrap());
    assert!(!MentionNotificationUseCase::mark_as_read(&db, "person-3", "m2").await.unwrap(), "another recipient cannot mark it read");
    assert_eq!(MentionNotificationUseCase::list_notifications(&db, "person-2", true, 10).await.unwrap().len(), 1);

    let friendship = || InAppNotification::from_friendship_event("fe1".into(), "FriendRequest".into(), "person-2".into(), "person-1".into(), "fr1".into(), "request".into());
    MentionNotificationUseCase::persist_friendship_notification(&db, friendship()).await.unwrap();
    MentionNotificationUseCase::persist_friendship_notification(&db, friendship()).await.unwrap();
    assert_eq!(MentionNotificationUseCase::list_notifications(&db, "person-2", false, 10).await.unwrap().len(), 3, "friendship notification is idempotent");
}
