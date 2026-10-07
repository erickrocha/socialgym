use super::*;
use tonic::Code;

#[test]
fn every_business_error_kind_has_its_documented_status() {
    let cases = [
        (BusinessError::validation("x"), Code::InvalidArgument),
        (BusinessError::unauthorized("x"), Code::Unauthenticated),
        (BusinessError::forbidden("x"), Code::PermissionDenied),
        (BusinessError::not_found("x"), Code::NotFound),
        (BusinessError::conflict("x"), Code::AlreadyExists),
        (BusinessError::locked("x"), Code::FailedPrecondition),
        (BusinessError::infrastructure("x"), Code::Unavailable),
    ];
    for (error, code) in cases {
        assert_eq!(business_status(&error).code(), code, "{:?}", error.kind);
    }
}

#[test]
fn dependency_failures_do_not_leak_their_message() {
    let status = business_status(&BusinessError::infrastructure("mongodb://user:pw@host"));
    assert!(!status.message().contains("pw"));
}
