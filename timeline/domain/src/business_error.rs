use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessErrorKind {
    Validation,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    Locked,
    Infrastructure,
}

pub struct BusinessError {
    pub kind: BusinessErrorKind,
    pub message: String,
}

impl BusinessError {
    pub fn new(message: String) -> Self {
        Self::infrastructure(message)
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::with_kind(BusinessErrorKind::Validation, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::with_kind(BusinessErrorKind::Unauthorized, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::with_kind(BusinessErrorKind::Forbidden, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::with_kind(BusinessErrorKind::NotFound, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::with_kind(BusinessErrorKind::Conflict, message)
    }

    pub fn locked(message: impl Into<String>) -> Self {
        Self::with_kind(BusinessErrorKind::Locked, message)
    }

    pub fn infrastructure(message: impl Into<String>) -> Self {
        Self::with_kind(BusinessErrorKind::Infrastructure, message)
    }

    pub fn with_kind(kind: BusinessErrorKind, message: impl Into<String>) -> Self {
        BusinessError {
            kind,
            message: message.into(),
        }
    }
}

impl Debug for BusinessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BusinessError")
            .field("kind", &self.kind)
            .field("message", &self.message)
            .finish()
    }
}

impl Display for BusinessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for BusinessError {
    fn description(&self) -> &str {
        &self.message
    }
}
