pub mod proto;
pub mod service;
pub mod infrastructure;
pub mod auth;

#[cfg(test)]
#[path = "tests/c010_authorization_gaps_test.rs"]
mod c010_authorization_gaps_test;

#[cfg(test)]
#[path = "tests/server_support.rs"]
mod server_support;

#[cfg(test)]
#[path = "tests/auth_service_test.rs"]
mod auth_service_test;

#[cfg(test)]
#[path = "tests/account_consent_service_test.rs"]
mod account_consent_service_test;

#[cfg(test)]
#[path = "tests/file_flows_test.rs"]
mod file_flows_test;

#[cfg(test)]
#[path = "tests/person_friend_settings_test.rs"]
mod person_friend_settings_test;

#[cfg(test)]
#[path = "tests/business_profile_service_test.rs"]
mod business_profile_service_test;

#[cfg(test)]
#[path = "tests/workout_public_utility_test.rs"]
mod workout_public_utility_test;

use business::commons::rate_limit::RateLimiter;
use std::sync::Arc;
use std::time::Duration;
use std::{env, fs};
use std::path::Path;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tower::ServiceBuilder;
use crate::auth::grpc_auth_layer::GrpcAuthLayer;
use crate::proto::business_profile::business_profile_service_server::BusinessProfileServiceServer;
use crate::proto::exercise::exercise_service_server::ExerciseServiceServer;
use crate::proto::friend::friend_service_server::FriendServiceServer;
use crate::proto::person::person_service_server::PersonServiceServer;
use crate::proto::resource::resource_service_server::ResourceServiceServer;
use crate::proto::settings::settings_service_server::SettingsServiceServer;
use crate::proto::team_member::team_member_service_server::TeamMemberServiceServer;
use crate::proto::workout::workout_service_server::WorkoutServiceServer;
use crate::proto::account::account_service_server::AccountServiceServer;
use crate::proto::address::address_search_service_server::AddressSearchServiceServer;
use crate::proto::auth::auth_service_server::AuthServiceServer;
use crate::proto::legal::legal_document_service_server::LegalDocumentServiceServer;
use crate::proto::consent::consent_service_server::ConsentServiceServer;
use crate::proto::media::media_service_server::MediaServiceServer;
use crate::service::account_service::GrpcAccountService;
use crate::service::address_search_service::GrpcAddressSearchService;
use crate::service::legal_document_service::GrpcLegalDocumentService;
use crate::service::auth_service::GrpcAuthService;
use crate::service::consent_service::GrpcConsentService;
use crate::service::media_service::GrpcMediaService;
use crate::service::business_profile_service::GrpcBusinessProfileService;
use crate::service::exercise_service::GrpcExerciseService;
use crate::service::friend_service::GrpcFriendService;
use crate::service::person_service::GrpcPersonService;
use crate::service::resource_service::GrpcResourceService;
use crate::service::settings_service::GrpcSettingService;
use crate::service::team_member_service::GrpcTeamMemberService;
use crate::service::workout_service::GrpcWorkoutService;

/// Largest message the server accepts or sends; a larger one is `OUT_OF_RANGE` before any use case runs.
const MAX_MESSAGE_BYTES: usize = 5 * 1024 * 1024;

const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("socialgym_descriptor");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // 1. Install Ring as Crypto Provider in a global scope
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Error installing Ring as the default CryptoProvider");

    let host = env::var("HOST").expect("HOST must be set");
    let grpc_port = env::var("GRPC_PORT").expect("GRPC_PORT must be set");
    let addr = format!("{}:{}",host,grpc_port).parse()?;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Arc::new(
        business::commons::db_pool::connect(&db_url, "workout-integration")
            .await
            .expect("Failed to connect to database"),
    );

    let person_service = GrpcPersonService::new(Arc::clone(&conn));
    let friend_service = GrpcFriendService::new(Arc::clone(&conn));
    let workout_service = GrpcWorkoutService::new(Arc::clone(&conn));
    let business_profile_service = GrpcBusinessProfileService::new(Arc::clone(&conn));
    let settings_service  = GrpcSettingService::new(Arc::clone(&conn));
    let exercise_service = GrpcExerciseService::new(Arc::clone(&conn));
    let team_member_service = GrpcTeamMemberService::new(Arc::clone(&conn));
    let resource_service  = GrpcResourceService::new(Arc::clone(&conn));
    let auth_service = GrpcAuthService::new(Arc::clone(&conn));
    let consent_service = GrpcConsentService::new(Arc::clone(&conn));
    let account_service = GrpcAccountService::new(Arc::clone(&conn));
    let media_service = GrpcMediaService::new(Arc::clone(&conn));
    let legal_service = GrpcLegalDocumentService;
    let address_service = GrpcAddressSearchService;
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    log::info!("Loading Tls certificate");

    let (cert_pem, key_pem) = load_tls_credentials()?;

    let identity = Identity::from_pem(cert_pem, key_pem);

    let tls_config = ServerTlsConfig::new()
        .identity(identity);

    log::info!("Registering the authentication layer to Grpc server");
    // Sign-up and sign-in share the REST allowance (20 a minute per address); refresh has its own.
    let credential_limiter = RateLimiter::new(20, Duration::from_secs(60));
    let refresh_limiter = RateLimiter::new(20, Duration::from_secs(60));
    // The legal documents are public static text read a few at a time; a wider allowance than credentials.
    let legal_limiter = RateLimiter::new(60, Duration::from_secs(60));
    let auth_layer = ServiceBuilder::new().layer(
        GrpcAuthLayer::new(Arc::clone(&conn))
            .public_method("/grpc.auth.AuthService/Signup", credential_limiter.clone())
            .public_method("/grpc.auth.AuthService/Login", credential_limiter)
            .public_method("/grpc.auth.AuthService/Refresh", refresh_limiter)
            .public_method("/grpc.legal.LegalDocumentService/ListLegalDocuments", legal_limiter.clone())
            .public_method("/grpc.legal.LegalDocumentService/GetLegalDocument", legal_limiter),
    );

    log::info!("gRPC server listening on https://{}", addr);
    Server::builder()
        .tls_config(tls_config)?
        .add_service(reflection)
        .add_service(auth_layer.clone().service(PersonServiceServer::new(person_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(FriendServiceServer::new(friend_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(WorkoutServiceServer::new(workout_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(BusinessProfileServiceServer::new(business_profile_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(SettingsServiceServer::new(settings_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(ExerciseServiceServer::new(exercise_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(TeamMemberServiceServer::new(team_member_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(ResourceServiceServer::new(resource_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(AuthServiceServer::new(auth_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(ConsentServiceServer::new(consent_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(AccountServiceServer::new(account_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(MediaServiceServer::new(media_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(LegalDocumentServiceServer::new(legal_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .add_service(auth_layer.clone().service(AddressSearchServiceServer::new(address_service).max_decoding_message_size(MAX_MESSAGE_BYTES).max_encoding_message_size(MAX_MESSAGE_BYTES)))
        .serve(addr)
        .await?;
    log::info!("Server created and services registered");
    Ok(())
}

fn load_tls_credentials() -> Result<(String,String), Box<dyn std::error::Error>> {

    let path_cert_url = env::var("TLS_CERT_PATH").expect("TLS_CERT_PATH must be set");
    let path_key_url = env::var("TLS_KEY_PATH").expect("TLS_KEY_PATH must be set");

    // we define the relative path for the files
    let path_cert = Path::new(path_cert_url.as_str());
    let path_key = Path::new(path_key_url.as_str());

    // read the files and convert to string
    let cert = fs::read_to_string(path_cert)
        .map_err(|e| format!("Error reading the file (server.crt): {}", e))?;
    let key = fs::read_to_string(path_key)
        .map_err(|e| format!("Error reading the private key (server.key): {}", e))?;

    Ok((cert, key))
}

#[cfg(test)]
#[path = "tests/contract_parity_test.rs"]
mod contract_parity_test;
