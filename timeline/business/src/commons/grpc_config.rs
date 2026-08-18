use std::fs;
use tonic::codegen::http;
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, ClientTlsConfig};
use domain::business_error::BusinessError;
use crate::commons::token_context::current_forwarded_token;

pub struct GrpcConfig {}

impl GrpcConfig {

	/// tonic interceptor that forwards the request-local Bearer token as the
	/// `authorization` gRPC metadata header.
	///
	/// Must be used with [`token_context::with_forwarded_token`] wrapping the
	/// async task so that [`current_forwarded_token`] can read the value.
	/// Requests executed outside such a scope pass through unchanged.
	pub fn auth_interceptor(
		mut req: tonic::Request<()>,
	) -> Result<tonic::Request<()>, tonic::Status> {
		if let Some(token) = current_forwarded_token() {
			let header_value = format!("Bearer {}", token)
				.parse::<MetadataValue<tonic::metadata::Ascii>>()
				.map_err(|e| {
					log::error!("Failed to build authorization metadata value: {:?}", e);
					tonic::Status::internal("invalid token metadata")
				})?;
			req.metadata_mut().insert("authorization", header_value);
		}
		Ok(req)
	}

	pub fn build_endpoint() -> String {
		log::info!("Building GRPC endpoint from environment variables");
		let protocol =std::env::var("GRPC_PROTOCOL").unwrap_or_else(|_| "https".to_string());
		let grpc_host = std::env::var("GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
		let grpc_port = std::env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
		let endpoint = format!("{}://{}:{}",protocol,grpc_host, grpc_port);
		log::info!("Using grpc_config endpoint {}", endpoint);
		endpoint
	}

	pub async fn create_channel(endpoint: &str) -> Result<Channel,BusinessError> {
		log::info!("Creating channel {}", endpoint);
		let use_tls = std::env::var("GRPC_USE_TLS")
			.unwrap_or_else(|_| "false".to_string())
			.to_lowercase() == "true";

		let uri = endpoint
			.parse()
			.map_err(|e| {
				log::error!("Invalid gRPC endpoint URI '{}': {:?}", endpoint, e);
				BusinessError::new("Invalid gRPC endpoint".to_string())
			})?;

		let channel = if use_tls {
			Self::create_tls_channel(uri).await?
		} else {
			Channel::builder(uri)
				.connect()
				.await
				.map_err(|e| {
					log::error!("Failed to connect to insecure gRPC endpoint '{}': {:?}", endpoint, e);
					BusinessError::new("Failed to connect to gRPC service".to_string())
				})?
		};

		Ok(channel)
	}

	/// Create a TLS-secured gRPC channel
	async fn create_tls_channel(uri: http::Uri) -> Result<Channel, BusinessError> {
		let cert_path = std::env::var("GRPC_CERT_PATH")
			.unwrap_or_else(|_| "/etc/grpc/certs/server.crt".to_string());

		let ca_cert = fs::read(&cert_path)
			.map_err(|e| {
				log::error!("Failed to read gRPC CA certificate from '{}': {}", cert_path, e);
				BusinessError::new(format!("Failed to load gRPC certificate: {}", e))
			})?;

		let domain_name = std::env::var("GRPC_DOMAIN_NAME")
			.unwrap_or_else(|_| "localhost".to_string());
		log::info!("Using gRPC domain name for TLS: {}", domain_name);
		let tls_config = ClientTlsConfig::new()
			.ca_certificate(tonic::transport::Certificate::from_pem(&ca_cert))
			.domain_name(domain_name);

		Channel::builder(uri)
			.tls_config(tls_config)
			.map_err(|e| {
				log::error!("Failed to configure TLS for gRPC: {:?}", e);
				BusinessError::new("Failed to configure gRPC TLS".to_string())
			})?
			.connect()
			.await
			.map_err(|e| {
				log::error!("Failed to connect to TLS-secured gRPC endpoint: {:?}", e);
				BusinessError::new("Failed to connect to gRPC service".to_string())
			})
	}
}