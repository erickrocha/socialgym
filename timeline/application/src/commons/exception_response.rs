use crate::commons::i18n::{translate, ErrorKey, Locale};
use crate::http::json::error_response_json::ErrorResponseJson;
use axum::response::IntoResponse;
use axum::Json;

pub type HttpResponse<T> = Result<T, ExceptionResponse>;

#[derive(Debug)]
pub enum ExceptionResponse {
    Unauthorized(Locale, ErrorKey),
    Forbidden(Locale, ErrorKey),
    BadRequest(Locale, ErrorKey),
    InternalServerError(Locale, ErrorKey),
    NotFound(Locale,ErrorKey)
}

impl ExceptionResponse {
    pub fn bad_request(locale: Locale, key: ErrorKey) -> Self {
        Self::BadRequest(locale, key)
    }
    pub fn internal_server_error(locale: Locale, key: ErrorKey) -> Self {
        Self::InternalServerError(locale, key)
    }
    pub fn not_found(locale: Locale, key: ErrorKey) -> Self {
        Self::NotFound(locale, key)
    }
}

impl IntoResponse for ExceptionResponse {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let (status, locale, key) = match self {
            ExceptionResponse::Unauthorized(locale, key) => (axum::http::StatusCode::UNAUTHORIZED, locale, key),
            ExceptionResponse::Forbidden(locale, key) => (axum::http::StatusCode::FORBIDDEN, locale, key),
            ExceptionResponse::BadRequest(locale, key) => (axum::http::StatusCode::BAD_REQUEST, locale, key),
            ExceptionResponse::InternalServerError(locale, key) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, locale, key),
            ExceptionResponse::NotFound(locale, key) => (axum::http::StatusCode::NOT_FOUND, locale, key),
        };

        let payload = ErrorResponseJson::new(key.as_str().to_string(), translate(locale, key));
        (status, Json(payload)).into_response()
    }
}