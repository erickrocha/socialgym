mod support;

use business::proto::proto::team_member::TeamRosterResponse;
use business::use_cases::chat_use_case::ChatUseCase;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use domain::business_error::BusinessErrorKind;
use domain::conversation::{
    CONVERSATION_TYPE_BUSINESS_DIRECT, CONVERSATION_TYPE_BUSINESS_TEAM_GROUP,
    CONVERSATION_TYPE_DIRECT_PERSON,
};
use domain::message::{SENDER_KIND_BUSINESS_PROFILE, SENDER_KIND_PERSON};
use domain::user::User;

fn user(n: i32, active_business: Option<&str>) -> User {
    User::new(
        format!("Person {n}"),
        format!("p{n}@t.test"),
        format!("u{n}"),
        n,
        format!("person-{n}"),
        String::new(),
        active_business.map(str::to_string),
    )
}

fn roster(owner: &str, members: &[&str]) -> TeamRosterResponse {
    TeamRosterResponse {
        business_profile_id: 7,
        business_profile_uuid: "bp-1".into(),
        business_profile_name: "Gym".into(),
        business_profile_logo_object_key: String::new(),
        owner_person_uuid: owner.into(),
        accepted_member_person_uuids: members.iter().map(|m| m.to_string()).collect(),
    }
}

fn kind(e: &domain::business_error::BusinessError) -> &BusinessErrorKind {
    &e.kind
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn direct_conversation_requires_friendship_and_is_idempotent() {
    let db = support::database().await;
    let stub = support::start_stub().await;
    let (a, b) = (user(1, None), user(2, None));

    assert!(matches!(
        kind(
            &ChatUseCase::get_or_create_direct(&db, &a, "  ")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Validation
    ));
    assert!(matches!(
        kind(
            &ChatUseCase::get_or_create_direct(&db, &a, "person-1")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Validation
    ));
    assert!(
        matches!(
            kind(
                &ChatUseCase::get_or_create_direct(&db, &a, "person-2")
                    .await
                    .unwrap_err()
            ),
            BusinessErrorKind::Forbidden
        ),
        "not friends"
    );

    *stub.friends.lock().unwrap() = vec!["person-2".into()];
    let created = ChatUseCase::get_or_create_direct(&db, &a, "person-2")
        .await
        .unwrap();
    assert_eq!(created.conversation_type, CONVERSATION_TYPE_DIRECT_PERSON);
    assert_eq!(created.participant_person_uuids.len(), 2);
    let again = ChatUseCase::get_or_create_direct(&db, &a, "person-2")
        .await
        .unwrap();
    assert_eq!(again.uuid, created.uuid);

    *stub.friends.lock().unwrap() = vec!["person-1".into()];
    let from_b = ChatUseCase::get_or_create_direct(&db, &b, "person-1")
        .await
        .unwrap();
    assert_eq!(
        from_b.uuid, created.uuid,
        "the key does not depend on who starts the chat"
    );
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn direct_messages_are_validated_stored_notified_read_and_listed() {
    let db = support::database().await;
    let stub = support::start_stub().await;
    let (a, b, c) = (user(1, None), user(2, None), user(3, None));
    *stub.friends.lock().unwrap() = vec!["person-2".into()];
    let conversation = ChatUseCase::get_or_create_direct(&db, &a, "person-2")
        .await
        .unwrap();

    // validation, lookup, membership
    assert!(matches!(
        kind(
            &ChatUseCase::send_message(&db, &a, &conversation.uuid, "hi", vec![], " ")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Validation
    ));
    assert!(matches!(
        kind(
            &ChatUseCase::send_message(&db, &a, &conversation.uuid, "  ", vec![], "m0")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Validation
    ));
    assert!(matches!(
        kind(
            &ChatUseCase::send_message(&db, &a, "missing", "hi", vec![], "m0")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::NotFound
    ));
    assert!(matches!(
        kind(
            &ChatUseCase::send_message(&db, &c, &conversation.uuid, "hi", vec![], "m0")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Forbidden
    ));

    // send as a person; the other participant is the recipient and gets an in-app notification
    let sent = ChatUseCase::send_message(&db, &a, &conversation.uuid, "  hello  ", vec![], "m1")
        .await
        .unwrap();
    assert_eq!(sent.message.body, "hello");
    assert_eq!(sent.message.sender_kind, SENDER_KIND_PERSON);
    assert_eq!(sent.conversation_type, CONVERSATION_TYPE_DIRECT_PERSON);
    assert!(sent.recipients.contains(&"person-2".to_string()));
    assert_eq!(
        MentionNotificationUseCase::list_notifications(&db, "person-2", true, 10)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        MentionNotificationUseCase::list_notifications(&db, "person-1", true, 10)
            .await
            .unwrap()
            .is_empty(),
        "no self notification"
    );

    // conversation list: unread for the recipient only
    let for_b = ChatUseCase::list_conversations(&db, &b, 0).await.unwrap();
    assert_eq!((for_b.len(), for_b[0].unread), (1, true));
    let for_a = ChatUseCase::list_conversations(&db, &a, 0).await.unwrap();
    assert!(!for_a[0].unread);
    assert!(
        ChatUseCase::list_conversations(&db, &c, 0)
            .await
            .unwrap()
            .is_empty()
    );

    // history
    let page = ChatUseCase::list_messages(&db, &b, &conversation.uuid, 0)
        .await
        .unwrap();
    assert_eq!(page.len(), 1);
    assert!(matches!(
        kind(
            &ChatUseCase::list_messages(&db, &c, &conversation.uuid, 0)
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Forbidden
    ));
    assert_eq!(
        ChatUseCase::list_messages_since(&db, &b, &conversation.uuid, 0)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        ChatUseCase::list_messages_since(&db, &b, &conversation.uuid, i64::MAX / 2)
            .await
            .unwrap()
            .is_empty()
    );

    // mark read
    assert!(matches!(
        kind(
            &ChatUseCase::mark_read(&db, &b, &conversation.uuid, " ")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Validation
    ));
    assert!(matches!(
        kind(
            &ChatUseCase::mark_read(&db, &c, &conversation.uuid, &sent.message.uuid)
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Forbidden
    ));
    let read = ChatUseCase::mark_read(&db, &b, &conversation.uuid, &sent.message.uuid)
        .await
        .unwrap();
    assert_eq!(
        (read.reader_person_uuid.as_str(), read.recipients.clone()),
        ("person-2", vec!["person-1".to_string()])
    );
    assert!(!ChatUseCase::list_conversations(&db, &b, 0).await.unwrap()[0].unread);

    // the friendship is re-checked on every send
    *stub.friends.lock().unwrap() = vec![];
    let e = ChatUseCase::send_message(&db, &a, &conversation.uuid, "still there?", vec![], "m2")
        .await
        .unwrap_err();
    assert!(matches!(kind(&e), BusinessErrorKind::Forbidden));
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn business_team_group_follows_the_roster_and_attributes_owner_messages() {
    let db = support::database().await;
    let stub = support::start_stub().await;
    let (owner, member, outsider) = (user(1, None), user(2, None), user(3, None));
    let owner_as_business = user(1, Some("bp-1"));

    assert!(matches!(
        kind(
            &ChatUseCase::get_or_create_business_team_group(&db, &owner, " ")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Validation
    ));
    assert!(
        ChatUseCase::get_or_create_business_team_group(&db, &owner, "bp-1")
            .await
            .is_err(),
        "roster lookup failure is propagated"
    );

    *stub.roster.lock().unwrap() = Some(roster("person-1", &["person-2"]));
    assert!(matches!(
        kind(
            &ChatUseCase::get_or_create_business_team_group(&db, &outsider, "bp-1")
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Forbidden
    ));
    let group = ChatUseCase::get_or_create_business_team_group(&db, &owner, "bp-1")
        .await
        .unwrap();
    assert_eq!(
        group.conversation_type,
        CONVERSATION_TYPE_BUSINESS_TEAM_GROUP
    );
    assert_eq!(group.participant_person_uuids.len(), 2);

    // a new accepted member joins on the next get-or-create (participants are synced)
    *stub.roster.lock().unwrap() = Some(roster("person-1", &["person-2", "person-3"]));
    let synced = ChatUseCase::get_or_create_business_team_group(&db, &owner, "bp-1")
        .await
        .unwrap();
    assert_eq!(synced.uuid, group.uuid);
    assert_eq!(synced.participant_person_uuids.len(), 3);

    // owner acting as the business is attributed to it; acting as a person is not; a member is a person
    let as_business = ChatUseCase::send_message(
        &db,
        &owner_as_business,
        &group.uuid,
        "welcome",
        vec![],
        "b1",
    )
    .await
    .unwrap();
    assert_eq!(
        as_business.message.sender_kind,
        SENDER_KIND_BUSINESS_PROFILE
    );
    assert_eq!(as_business.message.sender_display_name, "Gym");
    assert_eq!(
        as_business.message.sender_business_profile_uuid.as_deref(),
        Some("bp-1")
    );
    let as_person = ChatUseCase::send_message(&db, &owner, &group.uuid, "hi", vec![], "b2")
        .await
        .unwrap();
    assert_eq!(as_person.message.sender_kind, SENDER_KIND_PERSON);
    let from_member = ChatUseCase::send_message(&db, &member, &group.uuid, "thanks", vec![], "b3")
        .await
        .unwrap();
    assert_eq!(from_member.message.sender_kind, SENDER_KIND_PERSON);
    assert_eq!(from_member.recipients.len(), 3);

    // a member removed from the roster can no longer send
    *stub.roster.lock().unwrap() = Some(roster("person-1", &["person-3"]));
    let e = ChatUseCase::send_message(&db, &member, &group.uuid, "still here", vec![], "b4")
        .await
        .unwrap_err();
    assert!(matches!(kind(&e), BusinessErrorKind::Forbidden));
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL"]
async fn business_direct_conversation_resolves_the_member_from_the_roster() {
    let db = support::database().await;
    let stub = support::start_stub().await;
    let (owner, member, outsider) = (user(1, None), user(2, None), user(3, None));
    *stub.roster.lock().unwrap() = Some(roster("person-1", &["person-2"]));

    assert!(matches!(
        kind(
            &ChatUseCase::get_or_create_business_direct(&db, &owner, " ", None)
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Validation
    ));
    assert!(
        matches!(
            kind(
                &ChatUseCase::get_or_create_business_direct(&db, &owner, "bp-1", None)
                    .await
                    .unwrap_err()
            ),
            BusinessErrorKind::Validation
        ),
        "owner must name the member"
    );
    assert!(matches!(
        kind(
            &ChatUseCase::get_or_create_business_direct(&db, &owner, "bp-1", Some("person-9"))
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Forbidden
    ));
    assert!(matches!(
        kind(
            &ChatUseCase::get_or_create_business_direct(&db, &outsider, "bp-1", None)
                .await
                .unwrap_err()
        ),
        BusinessErrorKind::Forbidden
    ));

    let by_owner =
        ChatUseCase::get_or_create_business_direct(&db, &owner, "bp-1", Some("person-2"))
            .await
            .unwrap();
    assert_eq!(
        by_owner.conversation_type,
        CONVERSATION_TYPE_BUSINESS_DIRECT
    );
    let by_member =
        ChatUseCase::get_or_create_business_direct(&db, &member, "bp-1", Some("ignored"))
            .await
            .unwrap();
    assert_eq!(
        by_member.uuid, by_owner.uuid,
        "member and owner reach the same thread"
    );

    let sent = ChatUseCase::send_message(&db, &member, &by_owner.uuid, "question", vec![], "d1")
        .await
        .unwrap();
    assert_eq!(sent.message.sender_kind, SENDER_KIND_PERSON);
    assert_eq!(sent.conversation_type, CONVERSATION_TYPE_BUSINESS_DIRECT);
}
