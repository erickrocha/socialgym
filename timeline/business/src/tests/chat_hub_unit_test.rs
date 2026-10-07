use super::*;

#[test]
fn online_among_reports_only_connected_people() {
    let hub = ChatHub::new();
    let (conn, _rx_a) = hub.register("person-a");
    let (_, _rx_b) = hub.register("person-b");

    let mut online = hub.online_among(&[
        "person-a".to_string(),
        "person-b".to_string(),
        "person-c".to_string(),
    ]);
    online.sort();
    assert_eq!(online, vec!["person-a".to_string(), "person-b".to_string()]);

    // Last connection closing takes the person offline.
    hub.unregister("person-a", conn);
    assert_eq!(hub.online_among(&["person-a".to_string()]), Vec::<String>::new());

    // A second device keeps them online until every connection is gone.
    let (first, _rx1) = hub.register("person-d");
    let (_second, _rx2) = hub.register("person-d");
    hub.unregister("person-d", first);
    assert_eq!(hub.online_among(&["person-d".to_string()]), vec!["person-d".to_string()]);
}

#[test]
fn publish_reaches_every_connection_of_a_recipient_and_only_them() {
    let hub = ChatHub::new();
    let (_, mut first) = hub.register("person-a");
    let (_, mut second) = hub.register("person-a");
    let (_, mut other) = hub.register("person-b");

    hub.publish(&["person-a".to_string()], &ChatEvent::Pong);

    assert!(matches!(first.try_recv(), Ok(ChatEvent::Pong)));
    assert!(matches!(second.try_recv(), Ok(ChatEvent::Pong)));
    assert!(other.try_recv().is_err(), "a person who is not a recipient gets nothing");
}
