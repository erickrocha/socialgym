use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::consent_json::{AcceptConsentJson, ConsentJson, PendingConsentJson};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::{Extension, Json};
use business::commons::legal_documents::{PRIVACY, TERMS};
use business::domain::business_error::BusinessErrorKind;
use business::domain::user::User;
use business::use_cases::consent_use_case::ConsentUseCase;
use business::use_cases::token_revocation::TokenRevocation;

fn request_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-real-ip")
        .or_else(|| headers.get("x-forwarded-for"))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(str::trim)
        .filter(|v| !v.is_empty() && v.len() <= 45)
        .unwrap_or("unknown")
        .to_string()
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
    if !payload.accepted {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::ConsentRequired,
        ));
    }
    ConsentUseCase::accept(
        &state.conn,
        current_user.person_id,
        &payload.document,
        &payload.version,
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
    ConsentUseCase::revoke(&state.conn, current_user.person_id, &document)
        .await
        .map_err(|e| {
            ExceptionResponse::from_business(e, locale, ErrorKey::ConsentOperationFailed)
        })?;
    if matches!(document.as_str(), TERMS | PRIVACY) {
        if let Some(user_id) = current_user.id {
            TokenRevocation::revoke_all_for_user(&state.conn, user_id).await;
        }
    }
    Ok(Json(()))
}
