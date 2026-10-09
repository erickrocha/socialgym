use crate::http::welcome_controller::welcome;
use crate::routes::chat_routes::chat_routes;
use crate::routes::content_report_routes::{moderation_routes, report_routes};
use crate::routes::evolution_checkin_routes::evolution_checkin_routes;
use crate::routes::feed_routes::feed_route;
use crate::routes::notification_routes::notification_routes;
use crate::routes::post_routes::post_routes;
use crate::routes::workout_session_routes::workout_session_routes;
use axum::routing::get;
use axum::{
    Router,
    http::{HeaderName, Method, header},
};
use business::commons::chat_hub::ChatHub;
use mongodb::Database;
use std::env;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

pub mod authentication;
mod commons;
mod http;
mod infrastructure;
pub mod routes;

/// CORS origins this API accepts: a comma-separated `CORS_ALLOWED_ORIGINS` env
/// var, or localhost dev origins when unset — mobile clients don't send an
/// `Origin` header at all, so this only ever gates the web app.
fn allowed_origins() -> AllowOrigin {
    let configured = env::var("CORS_ALLOWED_ORIGINS")
        .ok()
        .filter(|s| !s.is_empty());
    let origins: Vec<axum::http::HeaderValue> = match configured {
        Some(raw) => raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|origin| axum::http::HeaderValue::from_str(origin).ok())
            .collect(),
        None => {
            log::warn!(
                "CORS_ALLOWED_ORIGINS is not set; falling back to localhost dev origins. \
                 Set it explicitly in production."
            );
            ["http://localhost:5173", "http://localhost:3000"]
                .into_iter()
                .filter_map(|origin| axum::http::HeaderValue::from_str(origin).ok())
                .collect()
        }
    };
    AllowOrigin::list(origins)
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        http::workout_session_controller::get_workout_session,
        http::workout_session_controller::create_workout_session,
        http::workout_session_controller::get_workout_sessions,
        http::post_controller::create_post,
        http::feed_controller::get_feed,
        http::feed_controller::get_feed_by_uuid,
        http::post_controller::add_comment,
        http::post_controller::add_reaction,
        http::post_controller::remove_reaction,
        http::notification_controller::list_notifications,
        http::notification_controller::mark_notification_read,
        http::push_device_controller::register_push_device,
        http::push_device_controller::remove_push_device,
        http::evolution_controller::add,
        http::evolution_controller::get_by_owner,
        http::chat_controller::list_conversations,
        http::chat_controller::create_direct,
        http::chat_controller::create_business_team_group,
        http::chat_controller::create_business_direct,
        http::chat_controller::list_messages,
        http::chat_controller::send_message,
        http::chat_controller::mark_read,
        http::chat_controller::presence,
    ),
    components(
        schemas(
            http::json::workout_session_json::WorkoutSessionJson,
            http::json::exercise_json::ExerciseJson,
            http::json::post_json::PostJson,
            http::json::post_json::MediaJson,
            http::json::post_json::ReactionJson,
            http::json::post_json::CommentJson,
            http::json::post_json::MentionJson,
            http::json::notification_json::NotificationJson,
            http::json::notification_json::MarkNotificationReadJson,
            http::json::push_device_json::RegisterPushDeviceJson,
            http::json::chat_json::ConversationJson,
            http::json::chat_json::ConversationParticipantJson,
            http::json::chat_json::LastMessagePreviewJson,
            http::json::chat_json::MessageJson,
            http::json::chat_json::MessageMediaJson,
            http::json::chat_json::SendMessageJson,
            http::json::chat_json::CreateDirectConversationJson,
            http::json::chat_json::CreateBusinessTeamGroupJson,
            http::json::chat_json::CreateBusinessDirectJson,
            http::json::chat_json::MarkReadJson,
            http::json::chat_json::MarkReadResultJson,
            http::json::chat_json::PresenceJson,
            http::json::evolution_check_in_json::EvolutionCheckInJson,
            http::json::error_response_json::ErrorResponseJson,
            http::json::error_response_json::BadRequestErrorJson,
            http::json::error_response_json::UnauthorizedErrorJson,
            http::json::error_response_json::ForbiddenErrorJson,
            http::json::error_response_json::InternalServerErrorJson,
        )
    ),
    tags(
        (name = "timeline", description = "Time Line API")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

/// Serves the REST API (and runs the background workers) on `HOST:PORT` until it stops.
pub async fn serve(database: Arc<Database>, chat_hub: ChatHub) -> anyhow::Result<()> {
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let state = AppState { database, chat_hub };

    infrastructure::mongo_indexes::ensure_indexes(&state.database).await;
    infrastructure::mention_notification_worker::start(Arc::clone(&state.database));
    infrastructure::friendship_notification_worker::start(Arc::clone(&state.database));
    infrastructure::push_notification_worker::start(Arc::clone(&state.database));

    log::info!("Starting server...");

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_origin(allowed_origins())
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::ACCESS_CONTROL_ALLOW_METHODS,
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderName::from_static("ngrok-skip-browser-warning"),
        ]);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(welcome))
        .nest(
            "/timeline/api",
            Router::new()
                .nest("/workout-sessions", workout_session_routes(state.clone()))
                .nest(
                    "/evolution-checkin",
                    evolution_checkin_routes(state.clone()),
                )
                .nest("/posts", post_routes(state.clone()))
                .nest("/feed", feed_route(state.clone()))
                .nest("/notifications", notification_routes(state.clone()))
                .nest(
                    "/push-devices",
                    crate::routes::push_device_routes::push_device_routes(state.clone()),
                )
                .nest("/chat", chat_routes(state.clone()))
                .route("/chat/ws", get(http::chat_ws_handler::ws))
                .nest("/reports", report_routes(state.clone()))
                .nest("/moderation", moderation_routes(state.clone())),
        )
        // 5 MiB cap: posts/workout-sessions carry JSON payloads (media metadata,
        // executed sets) but never raw file bytes — those go to S3 via presigned URLs.
        .layer(axum::extract::DefaultBodyLimit::max(5 * 1024 * 1024))
        .layer(cors)
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    log::info!("Server started on address {}", server_url);
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Database>,
    pub chat_hub: ChatHub,
}
