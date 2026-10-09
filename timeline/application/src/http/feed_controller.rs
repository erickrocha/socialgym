use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::ErrorResponseJson;
use crate::http::json::post_json::PostJson;
use crate::http::post_controller::FeedParams;
use crate::infrastructure::mapper::{Mapper, PostMapper};
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use business::use_cases::post_use_case::PostUseCase;
use domain::post::Post;
use domain::user::User;

/// The posts as a feed shows them: avatar and media URLs signed.
async fn signed_posts(
    posts: Vec<Post>,
    locale: Locale,
) -> Result<Vec<PostJson>, ExceptionResponse> {
    let url_cache = PostUseCase::signed_urls_for(&posts)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::FeedFetchFailed)
        })?;
    Ok(posts
        .into_iter()
        .map(|post| PostMapper::json(PostUseCase::with_signed_urls(post, &url_cache)))
        .collect())
}

// ── Feed (page-based pagination) ───────────────────────────────────────────────
#[utoipa::path(
	get,
	tag = "timeline",
	path = "/timeline/api/feed",
	params(
        ("page" = Option<u32>, Query,
            description = "Zero-based page index. Omit or use 0 for the first page. Each page returns up to 20 posts."),
	),
	responses(
        (status = 200, description = "Paginated list of posts", body = [PostJson]),
        (status = 401, description = "Unauthorized", body = ErrorResponseJson),
        (status = 403, description = "Forbidden", body = ErrorResponseJson),
        (status = 500, description = "Internal server error", body = ErrorResponseJson),
	),
	security(("api_key" = []))
)]
pub async fn get_feed(
    state: State<AppState>,
    Query(params): Query<FeedParams>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<Vec<PostJson>>> {
    let page = params.page.unwrap_or(0);

    match PostUseCase::get_feed(
        &state.database,
        current_user.person_id,
        current_user.person_uuid.clone(),
        page,
    )
    .await
    {
        Ok(posts) => Ok(Json(signed_posts(posts, locale).await?)),
        Err(e) => Err(ExceptionResponse::from_business(
            e,
            locale,
            ErrorKey::FeedFetchFailed,
        )),
    }
}

#[utoipa::path(
	get,
	tag = "timeline",
	path = "/timeline/api/feed/{uuid}",
	params(
        ("page" = Option<u32>, Query,
            description = "Zero-based page index. Omit or use 0 for the first page. Each page returns up to 20 posts."),
        ("uuid" = String, Path, description = "UUID of the user to fetch the feed for"),
	),
	responses(
        (status = 200, description = "Paginated list of posts", body = [PostJson]),
        (status = 401, description = "Unauthorized", body = ErrorResponseJson),
        (status = 403, description = "Forbidden", body = ErrorResponseJson),
        (status = 500, description = "Internal server error", body = ErrorResponseJson),
	),
	security(("api_key" = []))
)]
pub async fn get_feed_by_uuid(
    state: State<AppState>,
    Query(params): Query<FeedParams>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<PostJson>>> {
    let page = params.page.unwrap_or(0);
    match PostUseCase::get_business_feed(&state.database, uuid, page).await {
        Ok(posts) => Ok(Json(signed_posts(posts, locale).await?)),
        Err(e) => Err(ExceptionResponse::from_business(
            e,
            locale,
            ErrorKey::FeedFetchFailed,
        )),
    }
}
