use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use axum::Json;
use axum::extract::{Path, State};
use business::use_cases::account_data_deletion_use_case::AccountDataDeletionUseCase;
use futures::TryStreamExt;
use mongodb::bson::{Document, doc};
use serde::Serialize;

/// Internal, service-to-service endpoint (see `internal_auth_middleware`) that
/// `workout`'s account-deletion sweep calls to cascade-delete every piece of
/// timeline data belonging to a person as part of account deletion.
pub async fn delete_person_data(
    state: State<AppState>,
    Path(person_uuid): Path<String>,
) -> HttpResponse<Json<()>> {
    AccountDataDeletionUseCase::delete_all_for_person(&state.database, &person_uuid)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, Locale::En, ErrorKey::AccountDataDeletionFailed)
        })?;

    Ok(Json(()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonDataExport {
    posts: Vec<Document>,
    evolutions: Vec<Document>,
    workout_sessions: Vec<Document>,
    notifications: Vec<Document>,
}

async fn documents(
    collection: mongodb::Collection<Document>,
    filter: Document,
) -> Result<Vec<Document>, mongodb::error::Error> {
    let mut cursor = collection.find(filter).await?;
    let mut rows = Vec::new();
    while let Some(row) = cursor.try_next().await? {
        rows.push(row);
    }
    Ok(rows)
}

pub async fn export_person_data(
    state: State<AppState>,
    Path(person_uuid): Path<String>,
) -> HttpResponse<Json<PersonDataExport>> {
    let posts = documents(
        state.database.collection("posts"),
        doc! {
            "$or": [
                { "authorUuid": &person_uuid },
                { "comments.authorUuid": &person_uuid },
                { "reactions.authorId": &person_uuid },
            ]
        },
    )
    .await
    .map_err(|_| {
        ExceptionResponse::internal_server_error(Locale::En, ErrorKey::AccountDataDeletionFailed)
    })?;
    let evolutions = documents(
        state.database.collection("evolutions"),
        doc! { "personUuid": &person_uuid },
    )
    .await
    .map_err(|_| {
        ExceptionResponse::internal_server_error(Locale::En, ErrorKey::AccountDataDeletionFailed)
    })?;
    let workout_sessions = documents(
        state.database.collection("workout_sessions"),
        doc! { "personUuid": &person_uuid },
    )
    .await
    .map_err(|_| {
        ExceptionResponse::internal_server_error(Locale::En, ErrorKey::AccountDataDeletionFailed)
    })?;
    let notifications = documents(
        state.database.collection("in_app_notifications"),
        doc! {
            "$or": [{ "recipientPersonUuid": &person_uuid }, { "actorPersonUuid": &person_uuid }]
        },
    )
    .await
    .map_err(|_| {
        ExceptionResponse::internal_server_error(Locale::En, ErrorKey::AccountDataDeletionFailed)
    })?;
    Ok(Json(PersonDataExport {
        posts,
        evolutions,
        workout_sessions,
        notifications,
    }))
}
