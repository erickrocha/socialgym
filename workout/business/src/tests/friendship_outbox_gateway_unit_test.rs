use super::FriendshipOutboxGateway;

#[test]
fn retry_delay_is_bounded() {
    assert_eq!(FriendshipOutboxGateway::retry_delay_seconds(1), 2);
    assert_eq!(FriendshipOutboxGateway::retry_delay_seconds(5), 32);
    assert_eq!(FriendshipOutboxGateway::retry_delay_seconds(100), 256);
}
