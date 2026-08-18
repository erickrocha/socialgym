use business::use_cases::authentication::Authentication;
use business::use_cases::media_use_case::MediaUseCase;
use domain::access_token::Claims;
use futures::executor::block_on;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::env;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

fn env_lock() -> &'static Mutex<()> {
	static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
	LOCK.get_or_init(|| Mutex::new(()))
}

struct EnvReset {
	keys: Vec<String>,
}

impl Drop for EnvReset {
	fn drop(&mut self) {
		for key in &self.keys {
			unsafe {
				env::remove_var(key);
			}
		}
	}
}

fn set_env(vars: &[(&str, &str)]) -> EnvReset {
	for (key, value) in vars {
		unsafe {
			env::set_var(key, value);
		}
	}

	EnvReset {
		keys: vars.iter().map(|(k, _)| (*k).to_string()).collect(),
	}
}

fn future_exp() -> i64 {
	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs() as i64;

	now + 3600
}

fn sample_claims() -> Claims {
	Claims {
		sub: "erick@example.com".to_string(),
		exp: future_exp(),
		uuid: "user-uuid-1".to_string(),
		name: "Erick Rocha".to_string(),
		person_id: 42,
		person_uuid: "person-uuid-1".to_string(),
		person_object_key: "person/person-uuid-1/avatar/avatar-1".to_string(),
	}
}

#[test]
fn validate_access_token_returns_user_from_claims() {
	let _guard = env_lock().lock().unwrap();
	let _env = set_env(&[("ACCESS_TOKEN_SECRET", "test-access-secret")]);

	let claims = sample_claims();
	let token = encode(
		&Header::new(Algorithm::HS512),
		&claims,
		&EncodingKey::from_secret("test-access-secret".as_bytes()),
	)
		.unwrap();

	let user = block_on(Authentication::validate(token)).unwrap();

	assert_eq!(user.email, "erick@example.com");
	assert_eq!(user.uuid, "user-uuid-1");
	assert_eq!(user.name, "Erick Rocha");
	assert_eq!(user.person_id, 42);
	assert_eq!(user.person_uuid, "person-uuid-1");
	assert_eq!(
		user.person_object_key,
		"person/person-uuid-1/avatar/avatar-1"
	);
}

#[test]
fn validate_refresh_token_returns_user_from_claims() {
	let _guard = env_lock().lock().unwrap();
	let _env = set_env(&[("REFRESH_TOKEN_SECRET", "test-refresh-secret")]);

	let claims = sample_claims();
	let token = encode(
		&Header::new(Algorithm::HS512),
		&claims,
		&EncodingKey::from_secret("test-refresh-secret".as_bytes()),
	)
		.unwrap();

	let user = block_on(Authentication::validate_refresh_token(token)).unwrap();

	assert_eq!(user.email, "erick@example.com");
	assert_eq!(user.uuid, "user-uuid-1");
	assert_eq!(user.name, "Erick Rocha");
	assert_eq!(user.person_id, 42);
	assert_eq!(user.person_uuid, "person-uuid-1");
}

#[test]
fn validate_returns_business_error_for_invalid_token() {
	let _guard = env_lock().lock().unwrap();
	let _env = set_env(&[("ACCESS_TOKEN_SECRET", "test-access-secret")]);

	let result = block_on(Authentication::validate("not-a-jwt".to_string()));

	assert!(result.is_err());
	assert_eq!(result.unwrap_err().to_string(), "Token is invalid");
}

#[test]
fn generate_cloud_front_signed_url_returns_error_for_invalid_private_key() {
	let _guard = env_lock().lock().unwrap();
	let _env = set_env(&[
		("CLOUDFRONT_KEY_PAIR_ID", "K1234567890"),
		("CLOUDFRONT_DOMAIN", "https://d111111abcdef8.cloudfront.net"),
		("PRIVATE_KEY_RAW", "invalid-private-key"),
	]);

	let result = block_on(MediaUseCase::generate_cloud_front_signed_url(
		"person/person-uuid-1/post/media-1",
	));

	assert!(result.is_err());
	assert_eq!(
		result.unwrap_err().to_string(),
		"Failed to generate CloudFront signed URL"
	);
}