use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::s3_json::S3Json;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use business::domain::business_profile::BusinessProfile;
use business::domain::user::User;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use business::use_cases::common_use_case::choose_image_type;
use business::use_cases::person_media_use_case::PersonMediaUseCase;
use business::use_cases::person_use_case::PersonUseCase;
use std::collections::HashMap;

#[utoipa::path(
    get,
    path = "/workout/api/people/me/upload/{image_type}",
    responses(
        (status = 200, description = "Pre-signed URL generated successfully", body = S3Json),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("image_type" = String, Path, description = "Type of image being uploaded (e.g. avatar or cover)"),
        ("format" = Option<String>, Query, description = "Image format. Optional, defaults to jpg")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn person_image_upload(
    state: State<AppState>,
    Path(image_type): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<S3Json>> {
    let person_id = current_user.person_id;
    let format = params
        .get("format")
        .cloned()
        .unwrap_or_else(|| "jpg".to_string());
    let image_type = choose_image_type(image_type.as_str());

    let result = PersonUseCase::upload_person_image(
        &state.conn,
        person_id,
        image_type.clone(),
        format.clone(),
    )
    .await;
    if result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::PersonPreSignedUrlNotGenerated,
        ));
    }
    let image_storage = result.unwrap();
    let response = S3Json::new(image_storage.url, image_storage.object_key, person_id);
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/workout/api/people/me/delete/{image_type}",
    responses(
        (status = 200, description = "Person image deleted"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("image_type" = String, Path, description = "Type of image to delete (e.g. avatar or cover)")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn person_image_delete(
    state: State<AppState>,
    Path(image_type): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<()>> {
    let person_id = current_user.person_id;
    let image_type = choose_image_type(image_type.as_str());
    let result = PersonUseCase::delete_person_image(&state.conn, person_id, image_type).await;
    if result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::PersonPreSignedUrlNotDeleted,
        ));
    }
    Ok(Json(()))
}

#[utoipa::path(
    get,
    path = "/workout/api/business-profiles/upload/{image_type}",
    responses(
        (status = 200, description = "Pre-signed URL generated successfully", body = S3Json),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("business_profile_id" = i32, Path, description = "Business profile id for the company the image is being uploaded"),
        ("image_type" = String, Path, description = "Type of image being uploaded (e.g. logo or cover)"),
        ("format" = Option<String>, Query, description = "Image format. Optional, defaults to jpg")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn business_profile_image_upload(
    state: State<AppState>,
    Path(image_type): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    Extension(active_business_profile): Extension<BusinessProfile>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<S3Json>> {
    let business_profile_id = active_business_profile.id;
    let business_profile_uuid = active_business_profile.uuid.clone().unwrap();
    let format = params
        .get("format")
        .cloned()
        .unwrap_or_else(|| "jpg".to_string());
    let image_type = choose_image_type(image_type.as_str());

    let result = BusinessProfileUseCase::upload_business_profile_image(
        &state.conn,
        business_profile_id.unwrap(),
        business_profile_uuid,
        image_type.clone(),
        format.clone(),
    )
    .await;
    if result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::BusinessProfilePreSignedUrlNotGenerated,
        ));
    }
    let image_storage = result.unwrap();
    let response = S3Json::new(
        image_storage.url,
        image_storage.object_key,
        business_profile_id.unwrap(),
    );
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/workout/api/media/upload",
    responses(
        (status = 200, description = "Pre-signed upload URL generated successfully", body = S3Json),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("format" = Option<String>, Query, description = "Full content type. Optional, defaults to `image/jpeg`"),
        ("album" = String, Query, description = "Album name, e.g. `avatar`, `cover`, `gallery`")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn generate_media_upload_url(
    state: State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<S3Json>> {
    let person_id = current_user.person_id;

    let format = params
        .get("format")
        .cloned()
        .unwrap_or_else(|| "image/jpeg".to_string());

    let album = match params.get("album").cloned() {
        Some(a) if !a.is_empty() => a,
        _ => {
            return Err(ExceptionResponse::BadRequest(
                locale,
                ErrorKey::RequiredParameterMissing,
            ));
        }
    };

    let result =
        PersonMediaUseCase::generate_upload_url(&state.conn, person_id, album, format).await;

    match result {
        Ok(image_storage) => {
            let response = S3Json::new(image_storage.url, image_storage.object_key, person_id);
            Ok(Json(response))
        }
        Err(_e) => Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::PreSignedUrlNotGenerated,
        )),
    }
}
