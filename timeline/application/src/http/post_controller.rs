use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::post_json::{CommentJson, PostJson, ReactionJson};
use crate::infrastructure::mapper::{CommentMapper, Mapper, PostMapper, ReactionMapper};
use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use business::use_cases::post_use_case::PostUseCase;
use domain::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedParams {
    /// Zero-based feed page index. Each page returns up to 20 posts.
    pub page: Option<u32>,
}

// ── Create post ───────────────────────────────────────────────────────────────
#[utoipa::path(
    post,
    tag = "timeline",
    path = "/timeline/api/posts",
    request_body = PostJson,
    responses(
        (status = 201, description = "Post created", body = PostJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(("api_key" = []))
)]
pub async fn create_post(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<PostJson>,
) -> HttpResponse<(StatusCode, Json<PostJson>)> {
    let post = PostMapper::domain(payload);

    PostUseCase::create(&state.database, &current_user, post)
        .await
        .map(|p| (StatusCode::CREATED, Json(PostMapper::json(p))))
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::PostCreateFailed)
        })
}

// ── Add comment / reply ───────────────────────────────────────────────────────
#[utoipa::path(
    post,
    tag = "timeline",
    path = "/timeline/api/posts/{post_id}/comments",
    params(
        ("post_id" = String, Path, description = "Id of the post to comment on"),
    ),
    request_body = CommentJson,
    responses(
        (status = 200, description = "Updated post with the new comment", body = PostJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(("api_key" = []))
)]
pub async fn add_comment(
    state: State<AppState>,
    Path(post_id): Path<String>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<CommentJson>,
) -> HttpResponse<(StatusCode, Json<PostJson>)> {
    let comment = CommentMapper::domain(payload);
    let post_response =
        PostUseCase::add_comment(&state.database, &current_user, post_id, comment).await;

    post_response
        .map(|p| (StatusCode::CREATED, Json(PostMapper::json(p))))
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::CommentAddFailed)
        })
}
// ── Add / update reaction ─────────────────────────────────────────────────────
#[utoipa::path(
    post,
    tag = "timeline",
    path = "/timeline/api/posts/{post_id}/reactions",
    params(
        ("post_id" = String, Path, description = "Id of the post to react to"),
    ),
    responses(
        (status = 200, description = "Updated post with the new reaction", body = PostJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(("api_key" = []))
)]
pub async fn add_reaction(
    state: State<AppState>,
    Path(post_id): Path<String>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<ReactionJson>,
) -> HttpResponse<(StatusCode, Json<PostJson>)> {
    let reaction = ReactionMapper::domain(payload);
    PostUseCase::add_reaction(&state.database, &current_user, post_id, reaction)
        .await
        .map(|p| (StatusCode::CREATED, Json(PostMapper::json(p))))
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::ReactionAddFailed)
        })
}
// ── Remove reaction ───────────────────────────────────────────────────────────
#[utoipa::path(
    delete,
    tag = "timeline",
    path = "/timeline/api/posts/{post_id}/reactions",
    params(
        ("post_id" = String, Path, description = "Id of the post"),
    ),
    responses(
        (status = 200, description = "Deleted reaction from post", body = PostJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(("api_key" = []))
)]
pub async fn remove_reaction(
    state: State<AppState>,
    Path(post_id): Path<String>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<(StatusCode, Json<PostJson>)> {
    PostUseCase::remove_reaction(&state.database, post_id, current_user.person_uuid)
        .await
        .map(|p| (StatusCode::OK, Json(PostMapper::json(p))))
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::ReactionRemoveFailed)
        })
}
