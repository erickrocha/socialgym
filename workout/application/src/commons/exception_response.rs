use crate::commons::i18n::{translate, ErrorKey, Locale};
use crate::http::json::error_response_json::ErrorResponseJson;
use axum::response::IntoResponse;
use axum::Json;
use business::domain::business_error::{BusinessError, BusinessErrorKind};

pub type HttpResponse<T> = Result<T, ExceptionResponse>;

#[derive(Debug)]
pub enum ExceptionResponse {
    Unauthorized(Locale, ErrorKey),

    Forbidden(Locale, ErrorKey),

    BadRequest(Locale, ErrorKey),

    NotFound(Locale, ErrorKey),

    Locked(Locale, ErrorKey),

    Conflict(Locale, ErrorKey),

    InternalServerError(Locale, ErrorKey),

    TooManyRequests(Locale, ErrorKey),
}

impl ExceptionResponse {
    pub fn from_business(error: BusinessError, locale: Locale, key: ErrorKey) -> Self {
        match error.kind {
            BusinessErrorKind::Validation => Self::BadRequest(locale, key),
            BusinessErrorKind::Unauthorized => Self::Unauthorized(locale, key),
            BusinessErrorKind::Forbidden => Self::Forbidden(locale, key),
            BusinessErrorKind::NotFound => Self::NotFound(locale, key),
            BusinessErrorKind::Conflict => Self::Conflict(locale, key),
            BusinessErrorKind::Locked => Self::Locked(locale, key),
            BusinessErrorKind::Infrastructure => Self::InternalServerError(locale, key),
        }
    }
}

impl IntoResponse for ExceptionResponse {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let (status, locale, key) = match self {
            ExceptionResponse::Unauthorized(locale, key) => {
                (axum::http::StatusCode::UNAUTHORIZED, locale, key)
            }
            ExceptionResponse::Forbidden(locale, key) => {
                (axum::http::StatusCode::FORBIDDEN, locale, key)
            }
            ExceptionResponse::BadRequest(locale, key) => {
                (axum::http::StatusCode::BAD_REQUEST, locale, key)
            }
            ExceptionResponse::NotFound(locale, key) => {
                (axum::http::StatusCode::NOT_FOUND, locale, key)
            }
            ExceptionResponse::Locked(locale, key) => (axum::http::StatusCode::LOCKED, locale, key),
            ExceptionResponse::Conflict(locale, key) => {
                (axum::http::StatusCode::CONFLICT, locale, key)
            }
            ExceptionResponse::InternalServerError(locale, key) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, locale, key)
            }
            ExceptionResponse::TooManyRequests(locale, key) => {
                (axum::http::StatusCode::TOO_MANY_REQUESTS, locale, key)
            }
        };

        let payload = ErrorResponseJson::new(key.as_str().to_string(), translate(locale, key));
        (status, Json(payload)).into_response()
    }
}
