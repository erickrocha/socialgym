//! The timeline gRPC server: the transport the mobile app and `workout` use. Handlers only
//! translate; every rule lives in the use cases the REST controllers call too.
pub mod auth;
pub mod chat;
pub mod convert;
pub mod health;
pub mod internal;
pub mod rate_limit;
pub mod reports_and_sessions;
pub mod services;
pub mod status;

use business::proto::proto::timeline::chat_service_server::ChatServiceServer;
use business::proto::proto::timeline::content_report_service_server::ContentReportServiceServer;
use business::proto::proto::timeline::workout_session_service_server::WorkoutSessionServiceServer;
use business::proto::proto::timeline::evolution_check_in_service_server::EvolutionCheckInServiceServer;
use business::proto::proto::timeline::feed_service_server::FeedServiceServer;
use business::proto::proto::timeline::health_service_server::HealthServiceServer;
use business::proto::proto::timeline::internal_service_server::InternalServiceServer;
use business::proto::proto::timeline::notification_service_server::NotificationServiceServer;
use business::proto::proto::timeline::post_service_server::PostServiceServer;
use business::proto::proto::timeline::push_device_service_server::PushDeviceServiceServer;
use crate::AppState;
use std::net::SocketAddr;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::server::Router;
use tonic::transport::{Identity, Server, ServerTlsConfig};

/// Largest message the server accepts or sends. REST caps bodies at 5 MiB; a larger gRPC message
/// is `RESOURCE_EXHAUSTED` before any use case runs.
pub const MAX_MESSAGE_BYTES: usize = 5 * 1024 * 1024;

/// A service behind the user authentication interceptor with the explicit message limits.
macro_rules! secured {
    ($server:ident, $services:expr) => {
        InterceptedService::new(
            $server::new($services)
                .max_decoding_message_size(MAX_MESSAGE_BYTES)
                .max_encoding_message_size(MAX_MESSAGE_BYTES),
            auth::user_interceptor,
        )
    };
}

/// All timeline services, each behind its authentication interceptor. Tests serve this router
/// in-process; `serve` adds TLS and the listen address.
pub fn router(server: Server, state: AppState) -> Router {
    let mut server = server;
    let services = services::Services { state };
    server
        .add_service(secured!(HealthServiceServer, health::Health))
        .add_service(secured!(PostServiceServer, services.clone()))
        .add_service(secured!(FeedServiceServer, services.clone()))
        .add_service(secured!(NotificationServiceServer, services.clone()))
        .add_service(secured!(EvolutionCheckInServiceServer, services.clone()))
        .add_service(secured!(PushDeviceServiceServer, services.clone()))
        .add_service(secured!(ChatServiceServer, services.clone()))
        .add_service(secured!(ContentReportServiceServer, services.clone()))
        .add_service(secured!(WorkoutSessionServiceServer, services.clone()))
        // Service-to-service: the shared secret, never a user token.
        .add_service(InterceptedService::new(
            InternalServiceServer::new(services)
                .max_decoding_message_size(MAX_MESSAGE_BYTES)
                .max_encoding_message_size(MAX_MESSAGE_BYTES),
            auth::internal_interceptor,
        ))
}

/// Serves gRPC on `TIMELINE_GRPC_PORT` (default 50052). TLS is on unless `TIMELINE_GRPC_TLS` is
/// `false`; with TLS the certificate comes from `TLS_CERT_PATH` and `TLS_KEY_PATH`.
pub async fn serve(state: AppState) -> anyhow::Result<()> {
    // A missing secret is a startup error: the internal service would otherwise refuse every call.
    anyhow::ensure!(
        std::env::var("INTERNAL_SERVICE_SECRET").is_ok_and(|s| !s.is_empty()),
        "INTERNAL_SERVICE_SECRET must be set"
    );
    let host = std::env::var("TIMELINE_GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("TIMELINE_GRPC_PORT").unwrap_or_else(|_| "50052".into());
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    // The TLS server needs a process-wide crypto provider; the second install is a harmless Err.
    let _ = rustls::crypto::ring::default_provider().install_default();
    // Keepalive pings keep an idle chat stream alive through the gateway and detect dead peers.
    let mut server = Server::builder()
        .http2_keepalive_interval(Some(std::time::Duration::from_secs(30)))
        .http2_keepalive_timeout(Some(std::time::Duration::from_secs(10)));
    if std::env::var("TIMELINE_GRPC_TLS").map(|v| v != "false").unwrap_or(true) {
        let cert = std::fs::read_to_string(std::env::var("TLS_CERT_PATH")?)?;
        let key = std::fs::read_to_string(std::env::var("TLS_KEY_PATH")?)?;
        server = server.tls_config(ServerTlsConfig::new().identity(Identity::from_pem(cert, key)))?;
    } else {
        log::warn!("timeline gRPC is serving without TLS (TIMELINE_GRPC_TLS=false)");
    }
    log::info!("timeline gRPC listening on {addr}");
    router(server, state).serve(addr).await?;
    Ok(())
}
