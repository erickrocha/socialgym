use super::*;
use tonic::Code;

fn executed_set(started: Option<&str>, owner: Option<&str>) -> pb::ExecutedSet {
    pb::ExecutedSet {
        owner_name: owner.map(String::from),
        started_at: started.map(String::from),
        completed_at: started.map(String::from),
        ..Default::default()
    }
}

fn session(started: Option<&str>, sets: Vec<pb::ExecutedSet>) -> pb::WorkoutSession {
    pb::WorkoutSession {
        uuid: Some("client-chosen".into()),
        person_uuid: "someone-else".into(),
        started_at: started.map(String::from),
        completed_at: started.map(String::from),
        executed_sets: sets,
        ..Default::default()
    }
}

#[test]
fn a_date_travels_as_the_text_rest_puts_in_json() {
    let date = text_to_date("2026-10-06T10:00:00").unwrap();
    assert_eq!(date_to_text(date), "2026-10-06T10:00:00");
    assert_eq!(text_to_date("not a date").unwrap_err().code(), Code::InvalidArgument);
}

#[test]
fn a_session_without_dates_or_set_owners_is_invalid() {
    let day = Some("2026-10-06T10:00:00");
    assert_eq!(WorkoutSessionMapper::domain(session(None, vec![])).unwrap_err().code(), Code::InvalidArgument);
    let no_set_dates = session(day, vec![executed_set(None, Some("Alice"))]);
    assert_eq!(WorkoutSessionMapper::domain(no_set_dates).unwrap_err().code(), Code::InvalidArgument);
    let no_owner = session(day, vec![executed_set(day, None)]);
    assert_eq!(WorkoutSessionMapper::domain(no_owner).unwrap_err().code(), Code::InvalidArgument);
}

#[test]
fn a_bad_date_is_reported_as_such_even_when_the_session_is_also_incomplete() {
    let bad = session(Some("yesterday"), vec![executed_set(None, None)]);
    let status = WorkoutSessionMapper::domain(bad).unwrap_err();
    assert_eq!(status.message(), "invalid date, expected ISO-8601");
}

#[test]
fn a_complete_session_gets_its_own_id_and_keeps_the_sets() {
    let day = Some("2026-10-06T10:00:00");
    let built = WorkoutSessionMapper::domain(session(day, vec![executed_set(day, Some("Alice"))])).unwrap();
    assert_ne!(built.uuid, "client-chosen", "the id the client sent is ignored");
    assert_eq!(built.exercises.len(), 1);
    assert_eq!(built.exercises[0].owner_name, "Alice");
}

#[test]
fn a_new_post_carries_no_author_and_a_server_id() {
    let post = PostMapper::domain(pb::CreatePostRequest { content: "hi".into(), ..Default::default() });
    assert!(post.author_uuid.is_empty(), "the use case fills the author from the token");
    assert!(!post.uuid.is_empty());
}
