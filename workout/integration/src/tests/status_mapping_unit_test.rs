use super::{business_status, rate_limited};
use business::domain::business_error::{BusinessError, BusinessErrorKind};
use tonic::Code;

fn code(kind: BusinessErrorKind) -> Code {
    business_status(BusinessError {
        kind,
        message: "m".into(),
    })
    .code()
}

#[test]
fn every_business_error_kind_maps_to_its_grpc_status() {
    assert_eq!(code(BusinessErrorKind::Validation), Code::InvalidArgument);
    assert_eq!(code(BusinessErrorKind::Unauthorized), Code::Unauthenticated);
    assert_eq!(code(BusinessErrorKind::Forbidden), Code::PermissionDenied);
    assert_eq!(code(BusinessErrorKind::NotFound), Code::NotFound);
    assert_eq!(code(BusinessErrorKind::Conflict), Code::AlreadyExists);
    assert_eq!(code(BusinessErrorKind::Locked), Code::FailedPrecondition);
    assert_eq!(
        code(BusinessErrorKind::Infrastructure),
        Code::Unavailable,
        "a dependency outage is retryable"
    );
}

#[test]
fn the_rate_limit_is_resource_exhausted() {
    assert_eq!(rate_limited().code(), Code::ResourceExhausted);
}
