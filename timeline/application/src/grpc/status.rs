use domain::business_error::{BusinessError, BusinessErrorKind};
use tonic::Status;

/// The one place a use-case error becomes a gRPC status (icd.md section 4). `Infrastructure`
/// is the kind every dependency failure (workout, MongoDB, SQS) is raised with, so it maps to
/// `UNAVAILABLE`: the only intended difference from REST, which answers `500`.
pub fn status_from_business(error: &BusinessError) -> Status {
    match error.kind {
        BusinessErrorKind::Validation => Status::invalid_argument(error.message.clone()),
        BusinessErrorKind::Unauthorized => Status::unauthenticated(error.message.clone()),
        BusinessErrorKind::Forbidden => Status::permission_denied(error.message.clone()),
        BusinessErrorKind::NotFound => Status::not_found(error.message.clone()),
        BusinessErrorKind::Conflict => Status::already_exists(error.message.clone()),
        BusinessErrorKind::Locked => Status::failed_precondition(error.message.clone()),
        // The message names internals (hosts, driver errors): keep it in the log, not on the wire.
        BusinessErrorKind::Infrastructure => {
            log::error!("dependency failure: {}", error.message);
            Status::unavailable("a dependency is unavailable, try again")
        }
    }
}

#[cfg(test)]
mod tests {
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
            assert_eq!(status_from_business(&error).code(), code, "{:?}", error.kind);
        }
    }

    #[test]
    fn dependency_failures_do_not_leak_their_message() {
        let status = status_from_business(&BusinessError::infrastructure("mongodb://user:pw@host"));
        assert!(!status.message().contains("pw"));
    }
}
