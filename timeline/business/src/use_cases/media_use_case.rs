use std::borrow::Cow;
use std::env;
use cloudfront_sign::{get_signed_url, SignedOptions};
use domain::business_error::BusinessError;

const CLOUDFRONT_KEY_PAIR_ID: &str = "CLOUDFRONT_KEY_PAIR_ID";
const CLOUDFRONT_DOMAIN: &str = "CLOUDFRONT_DOMAIN";
const PRIVATE_KEY_RAW: &str = "PRIVATE_KEY_RAW";
pub struct MediaUseCase {}

impl MediaUseCase {

	pub async fn generate_cloud_front_signed_url(
		object_key: &str,
	) -> Result<String, BusinessError> {
		log::info!("Generating CloudFront signed URL for object_key: {}", object_key);
		// 1. Load configs from environment variables
		let key_id = env::var(CLOUDFRONT_KEY_PAIR_ID).expect("CLOUDFRONT_KEY_PAIR_ID not defined");
		let domain = env::var(CLOUDFRONT_DOMAIN).expect("CLOUDFRONT_DOMAIN not defined");

		let raw_value = env::var(PRIVATE_KEY_RAW).expect("Private key not found");

		// 2. read the private key from the file system
		// Note: the private key should be in PEM format, and should be the one associated with the key pair ID used in CloudFront
		let private_key_pem = raw_value.replace("\\n", "\n");

		// 3. Monta a URL base
		let resource_url = format!("{}/{}", domain, object_key);

		// 4. Generate the signed URL using the cloudfront_sign crate
		let options = SignedOptions {
			key_pair_id: Cow::from(key_id),
			private_key: Cow::from(private_key_pem.trim().to_string()),
			..Default::default()
		};

		let signed_url = get_signed_url(&resource_url, &options);
		if signed_url.is_err() {
			log::error!(
                "Error generating CloudFront signed URL: {:?}",
                signed_url.err()
            );
			return Err(BusinessError::new(
				"Failed to generate CloudFront signed URL".to_string(),
			));
		}
		Ok(signed_url.unwrap())
	}
}