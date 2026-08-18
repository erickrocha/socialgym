pub mod proto;
pub mod service;
pub mod infrastructure;
pub mod auth;

use std::sync::Arc;
use std::{env, fs};
use std::path::Path;
use sea_orm::Database;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tower::ServiceBuilder;
use crate::auth::grpc_auth_layer::GrpcAuthLayer;
use crate::proto::business_profile::business_profile_service_server::BusinessProfileServiceServer;
use crate::proto::exercise::exercise_service_server::ExerciseServiceServer;
use crate::proto::friend::friend_service_server::FriendServiceServer;
use crate::proto::person::person_service_server::PersonServiceServer;
use crate::proto::settings::settings_service_server::SettingsServiceServer;
use crate::proto::team_member::team_member_service_server::TeamMemberServiceServer;
use crate::proto::workout::workout_service_server::WorkoutServiceServer;
use crate::service::business_profile_service::GrpcBusinessProfileService;
use crate::service::exercise_service::GrpcExerciseService;
use crate::service::friend_service::GrpcFriendService;
use crate::service::person_service::GrpcPersonService;
use crate::service::settings_service::GrpcSettingService;
use crate::service::team_member_service::GrpcTeamMemberService;
use crate::service::workout_service::GrpcWorkoutService;

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
        Database::connect(&db_url)
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
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    log::info!("Loading Tls certificate");

    let (cert_pem, key_pem) = load_tls_credentials()?;

    let identity = Identity::from_pem(cert_pem, key_pem);

    let tls_config = ServerTlsConfig::new()
        .identity(identity);

    log::info!("Registering the authentication layer to Grpc server");
    let auth_layer = ServiceBuilder::new().layer(GrpcAuthLayer::new(Arc::clone(&conn)));

    log::info!("gRPC server listening on https://{}", addr);
    Server::builder()
        .tls_config(tls_config)?
        .add_service(reflection)
        .add_service(auth_layer.clone().service(PersonServiceServer::new(person_service)))
        .add_service(auth_layer.clone().service(FriendServiceServer::new(friend_service)))
        .add_service(auth_layer.clone().service(WorkoutServiceServer::new(workout_service)))
        .add_service(auth_layer.clone().service(BusinessProfileServiceServer::new(business_profile_service)))
        .add_service(auth_layer.clone().service(SettingsServiceServer::new(settings_service)))
        .add_service(auth_layer.clone().service(ExerciseServiceServer::new(exercise_service)))
        .add_service(auth_layer.clone().service(TeamMemberServiceServer::new(team_member_service)))
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
