//! The translated error messages live in `business` (the gRPC server uses them too); this keeps the
//! old path working for the REST controllers.
pub use business::commons::i18n::*;
