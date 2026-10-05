use super::{PushPreferenceError, PushPreferenceGateway};

#[test]
fn preference_status_classification_matches_retry_contract() {
    for code in [
        tonic::Code::Unavailable,
        tonic::Code::DeadlineExceeded,
        tonic::Code::ResourceExhausted,
    ] {
        assert_eq!(
            PushPreferenceGateway::classify_status(code),
            PushPreferenceError::Transient
        );
    }
    assert_eq!(
        PushPreferenceGateway::classify_status(tonic::Code::NotFound),
        PushPreferenceError::NotFound
    );
    assert_eq!(
        PushPreferenceGateway::classify_status(tonic::Code::PermissionDenied),
        PushPreferenceError::Permanent
    );
}
