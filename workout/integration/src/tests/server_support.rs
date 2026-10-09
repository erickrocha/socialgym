//! An in-process gRPC server wired like `main.rs` (the authentication layer in front of the services),
//! over a disposable PostGIS database (`TEST_DATABASE_URL`), for the C-010 service tests.
use crate::auth::grpc_auth_layer::GrpcAuthLayer;
use crate::proto::account::account_service_server::AccountServiceServer;
use crate::proto::address::address_search_service_server::AddressSearchServiceServer;
use crate::proto::auth::auth_service_server::AuthServiceServer;
use crate::proto::consent::consent_service_server::ConsentServiceServer;
use crate::proto::legal::legal_document_service_server::LegalDocumentServiceServer;
use crate::service::account_service::GrpcAccountService;
use crate::service::address_search_service::GrpcAddressSearchService;
use crate::service::auth_service::GrpcAuthService;
use crate::service::consent_service::GrpcConsentService;
use crate::service::legal_document_service::GrpcLegalDocumentService;
use business::commons::rate_limit::RateLimiter;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Server};
use tower::Layer;

pub struct Served {
    pub db: Arc<DatabaseConnection>,
    pub channel: Channel,
}

/// A fresh database and a server with the authentication, consent and account services; `limit` calls a
/// minute per address for sign-up, sign-in and refresh.
pub async fn serve(limit: u32) -> Served {
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", "c010-access-secret");
        std::env::set_var("REFRESH_TOKEN_SECRET", "c010-refresh-secret");
        std::env::remove_var("AUTH_RULES_ENABLED");
    }
    let url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostGIS database");
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    let db = Arc::new(db);
    let credentials = RateLimiter::new(limit, Duration::from_secs(60));
    let legal = RateLimiter::new(limit, Duration::from_secs(60));
    let layer = GrpcAuthLayer::new(db.clone())
        .public_method("/grpc.auth.AuthService/Signup", credentials.clone())
        .public_method("/grpc.auth.AuthService/Login", credentials)
        .public_method(
            "/grpc.auth.AuthService/Refresh",
            RateLimiter::new(limit, Duration::from_secs(60)),
        )
        .public_method(
            "/grpc.legal.LegalDocumentService/ListLegalDocuments",
            legal.clone(),
        )
        .public_method("/grpc.legal.LegalDocumentService/GetLegalDocument", legal);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = Server::builder()
        .add_service(layer.layer(AuthServiceServer::new(GrpcAuthService::new(db.clone()))))
        .add_service(
            layer.layer(ConsentServiceServer::new(GrpcConsentService::new(
                db.clone(),
            ))),
        )
        .add_service(
            layer.layer(AccountServiceServer::new(GrpcAccountService::new(
                db.clone(),
            ))),
        )
        .add_service(layer.layer(LegalDocumentServiceServer::new(GrpcLegalDocumentService)))
        .add_service(layer.layer(AddressSearchServiceServer::new(GrpcAddressSearchService)))
        .serve_with_incoming(TcpListenerStream::new(listener));
    tokio::spawn(server);
    let channel = Channel::from_shared(format!("http://{address}"))
        .unwrap()
        .connect()
        .await
        .unwrap();
    Served { db, channel }
}

/// A request with the access token and any extra metadata.
pub fn with<T>(message: T, bearer: Option<&str>, headers: &[(&'static str, &str)]) -> Request<T> {
    let mut request = Request::new(message);
    if let Some(token) = bearer {
        request.metadata_mut().insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {token}")).unwrap(),
        );
    }
    for (name, value) in headers {
        request
            .metadata_mut()
            .insert(*name, MetadataValue::try_from(*value).unwrap());
    }
    request
}

/// The REST `errorKey` a status carries in its `error-key` trailer.
pub fn error_key(status: &tonic::Status) -> &str {
    status
        .metadata()
        .get("error-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
}
