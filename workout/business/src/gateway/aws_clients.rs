use aws_config::BehaviorVersion;
use std::env;

const AWS_ENDPOINT_URL: &str = "AWS_ENDPOINT_URL";
const AWS_S3_PUBLIC_ENDPOINT_URL: &str = "AWS_S3_PUBLIC_ENDPOINT_URL";

async fn s3_client_with_endpoint(endpoint: Option<String>) -> aws_sdk_s3::Client {
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    match endpoint.filter(|value| !value.is_empty()) {
        Some(endpoint) => {
            let config = aws_sdk_s3::config::Builder::from(&shared_config)
                .endpoint_url(endpoint)
                .force_path_style(true)
                .build();
            aws_sdk_s3::Client::from_conf(config)
        }
        _ => aws_sdk_s3::Client::new(&shared_config),
    }
}

pub async fn s3_client() -> aws_sdk_s3::Client {
    s3_client_with_endpoint(env::var(AWS_ENDPOINT_URL).ok()).await
}

pub async fn s3_presign_client() -> aws_sdk_s3::Client {
    let endpoint = env::var(AWS_S3_PUBLIC_ENDPOINT_URL)
        .ok()
        .or_else(|| env::var(AWS_ENDPOINT_URL).ok());
    s3_client_with_endpoint(endpoint).await
}

pub async fn sqs_client() -> aws_sdk_sqs::Client {
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    match env::var(AWS_ENDPOINT_URL) {
        Ok(endpoint) if !endpoint.is_empty() => {
            let config = aws_sdk_sqs::config::Builder::from(&shared_config)
                .endpoint_url(endpoint)
                .build();
            aws_sdk_sqs::Client::from_conf(config)
        }
        _ => aws_sdk_sqs::Client::new(&shared_config),
    }
}
