use axum::{
    http::{header, HeaderName, Method},
    middleware,
    routing::get,
    Router,
};
use business::sea_orm::DatabaseConnection;
use migration::{Migrator, MigratorTrait};
use std::env;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod authentication;
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

/// `AUTH_RULES_ENABLED=false` silently turns off password policy, login
/// lockout and token revocation together. Make that loud at boot so a
/// copy-pasted dev `.env` can't disable it unnoticed.
fn warn_if_auth_rules_disabled() {
    let master_disabled = env::var("AUTH_RULES_ENABLED")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .map(|enabled| !enabled)
        .unwrap_or(false);
    if master_disabled {
        log::warn!(
            "AUTH_RULES_ENABLED=false: password policy, login lockout and token \
             revocation are ALL disabled. This must never be set in production."
        );
    }
}

use crate::authentication::authentication_middleware::authentication;
use crate::http::address_search_controller::search_address;
use crate::http::image_controller::generate_media_upload_url;
use crate::http::resource_controller::get_resource;
use crate::http::welcome_controller::welcome;
use crate::infrastructure::account_purge_worker;
use crate::infrastructure::data_export_worker;
use crate::infrastructure::sqs_worker;
use crate::routes::authentication_routes::auth_routes;
use crate::routes::business_profile_routes::business_profile_routes;
use crate::routes::exercise_routes::exercise_routes;
use crate::routes::friend_routes::friend_routes;
use crate::routes::person_routes::person_routes;
use crate::routes::settings_routes::settings_routes;
use crate::routes::team_member_routes::team_member_routes;
use crate::routes::workout_routes::workout_routes;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
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

#[derive(OpenApi)]
#[openapi(
	modifiers(&SecurityAddon),
	paths(
		http::auth_controller::sign_in,
		http::auth_controller::sign_up,
		http::auth_controller::refresh_token,
		http::auth_controller::logout,
		http::auth_controller::activate,
		http::auth_controller::deactivate,
		http::resource_controller::get_resource,
		http::address_search_controller::search_address,
		http::account_deletion_controller::request_account_deletion,
		http::account_deletion_controller::cancel_account_deletion,
		http::person_controller::get_me,
		http::person_controller::get_me_by_uuid,
		http::person_controller::update_person,
		http::person_controller::get_my_friend,
		http::person_controller::search_persons,
		http::person_address_controller::add_person_address,
		http::person_address_controller::update_person_address,
		http::person_address_controller::delete_person_address,
		http::person_info_controller::update_person_info,
		http::image_controller::person_image_upload,
		http::image_controller::person_image_delete,
		http::friend_controller::get_friends,
		http::friend_controller::find_friends,
		http::friend_controller::send_friend_request,
		http::friend_controller::accept_friend_request,
		http::friend_controller::deny_friend_request,
		http::friend_controller::cancel_friend_request,
		http::friend_controller::get_friend,
		http::team_member_controller::get_team_members,
		http::team_member_controller::get_team_member,
		http::team_member_controller::send_team_member_request,
		http::team_member_controller::accept_team_member_request,
		http::team_member_controller::deny_team_member_request,
		http::team_member_controller::cancel_team_member_request,
		http::image_controller::business_profile_image_upload,
		http::business_profile_controller::get_by_owner_id,
		http::business_profile_controller::get_active,
		http::workout_controller::add_workout,
		http::workout_controller::get_workouts,
		http::workout_controller::add_exercises,
		http::workout_controller::get_exercises,
		http::workout_controller::get_workout_by_id,
		http::workout_controller::get_workout_by_uuid,
		http::workout_controller::get_workouts_by_owner_uuid,
		http::workout_controller::update_workout,
		http::workout_controller::delete_workout_by_id,
		http::workout_controller::delete_workout_by_uuid,
		http::workout_controller::add_exercises_by_workout_uuid,
		http::exercise_controller::add_exercise,
		http::exercise_controller::delete_exercise,
		http::exercise_controller::query_exercises,
		http::exercise_controller::get_exercise_by_id,
		http::exercise_controller::get_exercise_by_uuid,
		http::exercise_controller::update_exercise,
		http::exercise_controller::delete_exercise_by_uuid,
		http::image_controller::generate_media_upload_url,
		http::settings_controller::get_settings_by_id,
		http::settings_controller::get_settings_by_uuid,
		http::settings_controller::get_my_settings,
		http::settings_controller::update_my_settings,
		http::settings_controller::create_settings,
	),
	components(
		schemas(
			http::json::user_json::UserJson,
			http::json::sign_up_json::SignUpJson,
			http::json::login_request::LoginRequest,
			http::json::access_token_json::AccessTokenJson,
			http::json::access_token_json::PendingAccountDeletionJson,
			http::json::refresh_token_request::RefreshTokenRequest,
			http::json::logout_request::LogoutRequest,
			http::json::resource_json::ResourceJson,
			http::json::country_json::CountryJson,
			http::json::address_candidate_json::AddressCandidateJson,
			http::json::account_deletion_json::AccountDeletionRequestJson,
			http::json::account_deletion_json::AccountDeletionStatusJson,
			http::json::person_json::PersonJson,
			http::json::friend_json::FriendJson,
			http::json::friend_page_json::FriendPageJson,
			http::json::team_member_json::TeamMemberJson,
			http::json::team_member_page_json::TeamMemberPageJson,
			http::json::person_info_json::PersonInfoJson,
			http::json::person_address_json::PersonAddressJson,
			http::json::s3_json::S3Json,
			http::json::business_profile_json::BusinessProfileJson,
			http::json::business_profile_address_json::BusinessProfileAddressJson,
			http::json::workout_json::WorkoutJson,
			http::json::exercise_json::ExerciseJson,
			http::json::exercise_json::PaginatedExerciseJson,
			http::json::settings_json::SettingsJson,
			http::exercise_controller::ExerciseParams,
		),
	),
	tags(
        (name = "workout", description = "Workout API")
	)
)]
struct ApiDoc;

// ==================== Route Builders ====================
/// Build public welcome route
fn welcome_route() -> Router<AppState> {
    Router::new().route("/", get(welcome))
}

/// Build media/upload routes
fn media_routes(state: AppState) -> Router<AppState> {
    Router::new().route(
        "/upload",
        get(generate_media_upload_url).route_layer(middleware::from_fn_with_state(
            state.clone(),
            authentication,
        )),
    )
}

/// Build resource/utility routes
fn resource_routes(state: AppState) -> Router<AppState> {
    Router::new().route(
        "/resource",
        get(get_resource).route_layer(middleware::from_fn_with_state(
            state.clone(),
            authentication,
        )),
    )
}

/// Build address search routes (Google Maps-backed)
fn address_search_routes(state: AppState) -> Router<AppState> {
    Router::new().route(
        "/search",
        get(search_address).route_layer(middleware::from_fn_with_state(
            state.clone(),
            authentication,
        )),
    )
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let connection = business::commons::db_pool::connect(&db_url, "workout-application")
        .await
        .expect("Failed to connect to database");
    Migrator::up(&connection, None).await?;

    let state = AppState {
        conn: Arc::new(connection),
    };

    // Start the SQS consumer background worker.
    // It will no-op gracefully when AWS_SQS_QUEUE_URL is not set.
    sqs_worker::start(Arc::clone(&state.conn));

    // Start the account-deletion sweep worker (immediate and 30-day-grace purges).
    account_purge_worker::start(Arc::clone(&state.conn));
    data_export_worker::start(Arc::clone(&state.conn));

    log::info!("Starting server...");

    warn_if_auth_rules_disabled();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::HEAD,
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
        // Public routes (no authentication)
        .merge(welcome_route())
        .route(
            "/legal/documents",
            get(http::legal_document_controller::list),
        )
        .route(
            "/legal/documents/{document}",
            get(http::legal_document_controller::get),
        )
        .merge(auth_routes(state.clone()))
        // API routes with authentication - nested organization
        .nest(
            "/workout/api",
            Router::new()
                .nest("/people", person_routes(state.clone()))
                .nest("/friends", friend_routes(state.clone()))
                .nest("/team-members", team_member_routes(state.clone()))
                .nest("/business-profiles", business_profile_routes(state.clone()))
                .nest("/workouts", workout_routes(state.clone()))
                .nest("/exercises", exercise_routes(state.clone()))
                .nest("/settings", settings_routes(state.clone()))
                .nest("/media", media_routes(state.clone()))
                .nest("/address", address_search_routes(state.clone()))
                .merge(resource_routes(state.clone())),
        )
        // 10 MiB cap: generous enough for exercise/workout JSON payloads with
        // several media/exercise entries, small enough to bound abuse.
        .layer(axum::extract::DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    log::info!("Server started on address {}", server_url);
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
