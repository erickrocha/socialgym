//! The timeline gRPC server: the transport the mobile app and `workout` use. Services only
//! translate; every rule lives in the use cases the REST controllers call too.
pub mod auth;
pub mod infrastructure;
pub mod proto;
pub mod service;

use crate::auth::grpc_auth::{internal_interceptor, user_interceptor};
use crate::proto::timeline::chat_service_server::ChatServiceServer;
use crate::proto::timeline::content_report_service_server::ContentReportServiceServer;
use crate::proto::timeline::evolution_check_in_service_server::EvolutionCheckInServiceServer;
use crate::proto::timeline::feed_service_server::FeedServiceServer;
use crate::proto::timeline::health_service_server::HealthServiceServer;
use crate::proto::timeline::internal_service_server::InternalServiceServer;
use crate::proto::timeline::notification_service_server::NotificationServiceServer;
use crate::proto::timeline::post_service_server::PostServiceServer;
use crate::proto::timeline::push_device_service_server::PushDeviceServiceServer;
use crate::proto::timeline::workout_session_service_server::WorkoutSessionServiceServer;
use crate::service::chat_service::GrpcChatService;
use crate::service::content_report_service::GrpcContentReportService;
use crate::service::evolution_check_in_service::GrpcEvolutionCheckInService;
use crate::service::feed_service::GrpcFeedService;
use crate::service::health_service::GrpcHealthService;
use crate::service::internal_service::GrpcInternalService;
use crate::service::notification_service::GrpcNotificationService;
use crate::service::post_service::GrpcPostService;
use crate::service::push_device_service::GrpcPushDeviceService;
use crate::service::workout_session_service::GrpcWorkoutSessionService;
use business::commons::chat_hub::ChatHub;
use mongodb::Database;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::server::Router;
use tonic::transport::{Identity, Server, ServerTlsConfig};

/// Largest message the server accepts or sends. REST caps bodies at 5 MiB; a larger gRPC message
/// is `RESOURCE_EXHAUSTED` before any use case runs.
pub const MAX_MESSAGE_BYTES: usize = 5 * 1024 * 1024;

/// A service behind the user authentication interceptor with the explicit message limits.
macro_rules! secured {
    ($server:ident, $service:expr) => {
        InterceptedService::new(
            $server::new($service)
                .max_decoding_message_size(MAX_MESSAGE_BYTES)
                .max_encoding_message_size(MAX_MESSAGE_BYTES),
            user_interceptor,
        )
    };
}

/// All timeline services, each behind its authentication interceptor. Tests serve this router
/// in-process; `serve` adds TLS and the listen address.
pub fn router(server: Server, database: Arc<Database>, chat_hub: ChatHub) -> Router {
    let mut server = server;
    server
        .add_service(secured!(HealthServiceServer, GrpcHealthService))
        .add_service(secured!(PostServiceServer, GrpcPostService::new(database.clone())))
        .add_service(secured!(FeedServiceServer, GrpcFeedService::new(database.clone())))
        .add_service(secured!(NotificationServiceServer, GrpcNotificationService::new(database.clone())))
        .add_service(secured!(EvolutionCheckInServiceServer, GrpcEvolutionCheckInService::new(database.clone())))
        .add_service(secured!(PushDeviceServiceServer, GrpcPushDeviceService::new(database.clone())))
        .add_service(secured!(ChatServiceServer, GrpcChatService::new(database.clone(), chat_hub)))
        .add_service(secured!(ContentReportServiceServer, GrpcContentReportService::new(database.clone())))
        .add_service(secured!(WorkoutSessionServiceServer, GrpcWorkoutSessionService::new(database.clone())))
        // Service-to-service: the shared secret, never a user token.
        .add_service(InterceptedService::new(
            InternalServiceServer::new(GrpcInternalService::new(database))
                .max_decoding_message_size(MAX_MESSAGE_BYTES)
                .max_encoding_message_size(MAX_MESSAGE_BYTES),
            internal_interceptor,
        ))
}

/// Serves gRPC on `TIMELINE_GRPC_PORT` (default 50052). TLS is on unless `TIMELINE_GRPC_TLS` is
/// `false`; with TLS the certificate comes from `TLS_CERT_PATH` and `TLS_KEY_PATH`.
pub async fn serve(database: Arc<Database>, chat_hub: ChatHub) -> anyhow::Result<()> {
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
    router(server, database, chat_hub).serve(addr).await?;
    Ok(())
}
