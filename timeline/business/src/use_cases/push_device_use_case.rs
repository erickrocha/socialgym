use crate::gateway::push_device_gateway::PushDeviceGateway;
use domain::business_error::BusinessError;
use mongodb::Database;
use uuid::Uuid;

pub struct PushDeviceUseCase;

impl PushDeviceUseCase {
    const MAX_TOKEN_LEN: usize = 4096;

    /// Registers (or moves) the installation for `owner_uuid`, the person in the token.
    pub async fn register(
        db: &Database,
        owner_uuid: &str,
        device_uuid: &str,
        platform: &str,
        registration_token: &str,
    ) -> Result<(), BusinessError> {
        if Uuid::parse_str(device_uuid).is_err()
            || !matches!(platform, "android" | "ios")
            || registration_token.trim().is_empty()
            || registration_token.len() > Self::MAX_TOKEN_LEN
        {
            return Err(BusinessError::validation("invalid push device"));
        }
        PushDeviceGateway::register(db, device_uuid, owner_uuid, platform, registration_token).await
    }

    /// Removes the installation only when it belongs to `owner_uuid`.
    pub async fn remove(
        db: &Database,
        owner_uuid: &str,
        device_uuid: &str,
    ) -> Result<(), BusinessError> {
        if PushDeviceGateway::remove_owned(db, device_uuid, owner_uuid).await? {
            Ok(())
        } else {
            Err(BusinessError::not_found("push device not found"))
        }
    }
}
