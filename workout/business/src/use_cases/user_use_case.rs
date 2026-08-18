use crate::commons::password_policy::{self, PasswordPolicyViolation};
use crate::domain::user::{User, UserEntityMapper};
use crate::gateway::user_gateway::UserGateway;
use sea_orm::DbConn;
use crate::commons::entity_mapper::EntityMapper;

#[derive(Debug)]
pub enum UserUseCaseError {
    InvalidInput,
    WeakPassword(Vec<PasswordPolicyViolation>),
    PersistFailed,
}

pub struct UserUseCase {}

impl UserUseCase {
    pub async fn add(db: &DbConn, user: User) -> Result<User, UserUseCaseError> {
        log::info!("Adding user: {:?}", user);

        if user.email.is_empty() || user.password.is_empty() {
            log::warn!("Email or password is empty");
            return Err(UserUseCaseError::InvalidInput);
        }

        if let Err(violations) = password_policy::validate(&user.password) {
            log::warn!("Password does not satisfy policy: {:?}", violations);
            return Err(UserUseCaseError::WeakPassword(violations));
        }

        let user_with_password_encrypted = User {
            password: bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).map_err(|e| {
                log::error!("Error encrypting password: {}", e);
                UserUseCaseError::PersistFailed
            })?,
            ..user
        };

        log::info!("Adding user: {:?}", user_with_password_encrypted);
        let entity = UserGateway::persist(db, user_with_password_encrypted).await;
        if entity.is_err() {
            let error = entity.err().unwrap();
            log::error!("Error adding user: {}", error);
            return Err(UserUseCaseError::PersistFailed);
        }
        let user = entity.unwrap();

        Ok(UserEntityMapper::from_active_model(user))
    }
}
