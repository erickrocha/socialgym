use crate::commons::entity_mapper::EntityMapper;
use crate::commons::{legal_documents, password_policy};
use crate::domain::enums::Position;
use crate::domain::person::{Person, PersonEntityMapper};
use crate::domain::settings::{Settings, SettingsEntityMapper};
use crate::domain::user::{User, UserEntityMapper};
use chrono::Utc;
use entity::{consent_entity, person_info_entity};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbConn, Set, TransactionTrait};

#[derive(Debug)]
pub enum RegistrationError {
    InvalidPerson,
    WeakPassword,
    OutdatedLegalDocument,
    Persistence,
}

pub struct RegistrationRequest {
    pub person: Person,
    pub email: String,
    pub password: String,
    pub language: String,
    pub terms_version: String,
    pub privacy_version: String,
    pub ip: String,
}

pub struct RegistrationUseCase;

impl RegistrationUseCase {
    pub async fn execute(
        db: &DbConn,
        request: RegistrationRequest,
    ) -> Result<User, RegistrationError> {
        Self::validate(&request)?;
        let txn = db
            .begin()
            .await
            .map_err(|_| RegistrationError::Persistence)?;
        let result = Self::persist(&txn, request).await;
        match result {
            Ok(user) => {
                txn.commit()
                    .await
                    .map_err(|_| RegistrationError::Persistence)?;
                Ok(user)
            }
            Err(error) => {
                let _ = txn.rollback().await;
                Err(error)
            }
        }
    }

    fn validate(request: &RegistrationRequest) -> Result<(), RegistrationError> {
        if request.person.firstname.trim().is_empty()
            || request.person.surname.trim().is_empty()
            || request.person.gender.trim().is_empty()
            || request.email.trim().is_empty()
            || request.password.is_empty()
            || request.person.firstname.len() > 255
            || request.person.surname.len() > 255
            || request.person.gender.len() > 255
            || request.email.len() > 500
            || request.password.len() > 128
        {
            return Err(RegistrationError::InvalidPerson);
        }
        if password_policy::validate(&request.password).is_err() {
            return Err(RegistrationError::WeakPassword);
        }
        if !legal_documents::is_current(legal_documents::TERMS, &request.terms_version)
            || !legal_documents::is_current(legal_documents::PRIVACY, &request.privacy_version)
        {
            return Err(RegistrationError::OutdatedLegalDocument);
        }
        Ok(())
    }

    async fn persist(
        txn: &DatabaseTransaction,
        request: RegistrationRequest,
    ) -> Result<User, RegistrationError> {
        let person = PersonEntityMapper::build_active_model(request.person)
            .insert(txn)
            .await
            .map_err(|_| RegistrationError::Persistence)?;

        person_info_entity::ActiveModel {
            person_id: Set(person.id),
            biography: Set(None),
            relationship: Set(None),
            job: Set(None),
            home_town: Set(None),
            current_city: Set(None),
            weight: Set(None),
            height: Set(None),
            ..Default::default()
        }
        .insert(txn)
        .await
        .map_err(|_| RegistrationError::Persistence)?;

        let person_uuid = person.uuid.to_string();
        SettingsEntityMapper::build_active_model(Settings::new(
            person.id,
            person_uuid.clone(),
            request.language,
            "default".to_string(),
            true,
            Position::Left,
            "Feed".to_string(),
        ))
        .insert(txn)
        .await
        .map_err(|_| RegistrationError::Persistence)?;

        let hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|_| RegistrationError::Persistence)?;
        let user = UserEntityMapper::build_active_model(User::new(
            Some(format!("{} {}", person.first_name, person.surname)),
            request.email,
            hash,
            person.id,
            person_uuid,
        ))
        .insert(txn)
        .await
        .map_err(|_| RegistrationError::Persistence)?;

        for (document, version) in [
            (legal_documents::TERMS, request.terms_version),
            (legal_documents::PRIVACY, request.privacy_version),
        ] {
            consent_entity::ActiveModel {
                person_id: Set(person.id),
                document: Set(document.to_string()),
                version: Set(version),
                accepted_at: Set(Utc::now()),
                ip: Set(request.ip.clone()),
                revoked_at: Set(None),
                ..Default::default()
            }
            .insert(txn)
            .await
            .map_err(|_| RegistrationError::Persistence)?;
        }

        Ok(UserEntityMapper::from_model(user))
    }
}
