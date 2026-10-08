use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::consent_json::{AcceptConsentJson, ConsentJson, PendingConsentJson};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::{Extension, Json};
use business::domain::business_error::BusinessErrorKind;
use business::domain::user::User;
use business::use_cases::consent_use_case::ConsentUseCase;
use business::use_cases::sign_up_use_case::acceptance_ip;

fn request_ip(headers: &HeaderMap) -> String {
    let header = |name: &str| headers.get(name).and_then(|v| v.to_str().ok());
    acceptance_ip(header("x-real-ip"), header("x-forwarded-for"))
}

pub async fn list(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<ConsentJson>>> {
    ConsentUseCase::list(&state.conn, current_user.person_id)
        .await
        .map(|rows| Json(rows.into_iter().map(ConsentJson::from).collect()))
        .map_err(|e| ExceptionResponse::from_business(e, locale, ErrorKey::ConsentOperationFailed))
}

/// Legal documents whose current version the caller has not accepted. An empty
/// list means nothing is blocking the caller. Reachable in restricted mode.
pub async fn pending(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<PendingConsentJson>>> {
    ConsentUseCase::pending(&state.conn, current_user.person_id)
        .await
        .map(|rows| Json(rows.into_iter().map(PendingConsentJson::from).collect()))
        .map_err(|e| ExceptionResponse::from_business(e, locale, ErrorKey::ConsentOperationFailed))
}

pub async fn accept(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    headers: HeaderMap,
    Json(payload): Json<AcceptConsentJson>,
) -> HttpResponse<Json<ConsentJson>> {
    ConsentUseCase::accept_confirmed(
        &state.conn,
        current_user.person_id,
        &payload.document,
        &payload.version,
        payload.accepted,
        &request_ip(&headers),
    )
    .await
    .map(|row| Json(ConsentJson::from(row)))
    .map_err(|e| {
        let key = if e.kind == BusinessErrorKind::Validation {
            ErrorKey::ConsentRequired
        } else {
            ErrorKey::ConsentOperationFailed
        };
        ExceptionResponse::from_business(e, locale, key)
    })
}

pub async fn revoke(
    State(state): State<AppState>,
    Path(document): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<()>> {
    ConsentUseCase::revoke_for_user(&state.conn, &current_user, &document)
        .await
        .map_err(|e| {
            ExceptionResponse::from_business(e, locale, ErrorKey::ConsentOperationFailed)
        })?;
    Ok(Json(()))
}
