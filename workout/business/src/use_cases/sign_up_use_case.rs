use crate::commons::legal_documents;
use crate::domain::access_token::AccessToken;
use crate::domain::person::Person;
use crate::use_cases::authentication::Authentication;
use crate::use_cases::registration_use_case::{RegistrationError, RegistrationRequest, RegistrationUseCase};
use chrono::NaiveDate;
use sea_orm::DbConn;

/// What a person types to register, plus where the call came from. Shared by the REST and gRPC forms so
/// the age, consent and address rules live in one place.
pub struct SignUpRequest {
    pub firstname: String,
    pub surname: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub email: String,
    pub password: String,
    pub terms_accepted: bool,
    pub privacy_accepted: bool,
    pub terms_version: String,
    pub privacy_version: String,
    /// The locale the person's settings start with.
    pub language: String,
    /// The `X-Real-IP` and `X-Forwarded-For` values of the call, for the consent record.
    pub real_ip: Option<String>,
    pub forwarded_for: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SignUpError {
    Underage,
    ConsentRequired,
    WeakPassword,
    /// Any other failure, including an email that is already registered: the answer is the same, so the
    /// call cannot be used to find out which emails have an account.
    Failed,
    /// The account was created but the first sign-in failed.
    Unauthorized,
}

pub fn is_at_least_eighteen(date_of_birth: NaiveDate, today: NaiveDate) -> bool {
    date_of_birth
        .checked_add_months(chrono::Months::new(18 * 12))
        .is_some_and(|birthday| birthday <= today)
}

/// The address recorded with the consent: the gateway's `X-Real-IP`, else the first `X-Forwarded-For`
/// entry, else `unknown`; anything empty or longer than an IPv6 address (45) is `unknown`.
pub fn acceptance_ip(real_ip: Option<&str>, forwarded_for: Option<&str>) -> String {
    real_ip
        .or(forwarded_for)
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.len() <= 45)
        .unwrap_or("unknown")
        .to_string()
}

pub struct SignUpUseCase;

impl SignUpUseCase {
    pub async fn execute(db: &DbConn, request: SignUpRequest, today: NaiveDate) -> Result<AccessToken, SignUpError> {
        if !is_at_least_eighteen(request.date_of_birth, today) {
            return Err(SignUpError::Underage);
        }
        if !request.terms_accepted
            || !request.privacy_accepted
            || !legal_documents::is_current(legal_documents::TERMS, &request.terms_version)
            || !legal_documents::is_current(legal_documents::PRIVACY, &request.privacy_version)
        {
            return Err(SignUpError::ConsentRequired);
        }
        let ip = acceptance_ip(request.real_ip.as_deref(), request.forwarded_for.as_deref());
        let person = Person::new(request.firstname, request.surname, request.date_of_birth, request.gender);
        let password = request.password.clone();
        let user = RegistrationUseCase::execute(
            db,
            RegistrationRequest {
                person,
                email: request.email,
                password: request.password,
                language: request.language,
                terms_version: request.terms_version,
                privacy_version: request.privacy_version,
                ip,
            },
        )
        .await
        .map_err(|error| match error {
            RegistrationError::WeakPassword => SignUpError::WeakPassword,
            RegistrationError::OutdatedLegalDocument => SignUpError::ConsentRequired,
            _ => SignUpError::Failed,
        })?;
        Authentication::execute(db, user.email, password)
            .await
            .map_err(|_| SignUpError::Unauthorized)
    }
}

#[cfg(test)]
#[path = "../tests/sign_up_unit_test.rs"]
mod tests;
