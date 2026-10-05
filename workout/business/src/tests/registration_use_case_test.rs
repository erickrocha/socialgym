use super::{RegistrationError, RegistrationRequest, RegistrationUseCase};
use crate::commons::legal_documents;
use crate::domain::person::Person;
use chrono::NaiveDate;

fn request(firstname: &str, password: &str, terms_version: &str) -> RegistrationRequest {
    RegistrationRequest {
        person: Person::new(
            firstname.to_string(),
            "Person".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "X".to_string(),
        ),
        email: "person@example.test".to_string(),
        password: password.to_string(),
        language: "en".to_string(),
        terms_version: terms_version.to_string(),
        privacy_version: legal_documents::current_version(legal_documents::PRIVACY).unwrap(),
        ip: "127.0.0.1".to_string(),
    }
}

#[test]
fn rejects_blank_person_fields_before_database_access() {
    let mut invalid = request(" ", "Str0ng!Password", "1.0.0");
    invalid.person.firstname = " \t ".to_string();

    assert!(matches!(
        RegistrationUseCase::validate(&invalid),
        Err(RegistrationError::InvalidPerson)
    ));
}

#[test]
fn rejects_weak_password() {
    unsafe { std::env::set_var("AUTH_RULES_ENABLED", "true") };
    unsafe { std::env::remove_var("PASSWORD_POLICY_ENABLED") };
    let result = RegistrationUseCase::validate(&request("Person", "weak", "1.0.0"));
    unsafe { std::env::remove_var("AUTH_RULES_ENABLED") };

    assert!(matches!(result, Err(RegistrationError::WeakPassword)));
}

#[test]
fn rejects_outdated_terms_version() {
    let current_terms = legal_documents::current_version(legal_documents::TERMS).unwrap();
    let result = RegistrationUseCase::validate(&request(
        "Person",
        "Str0ng!Password",
        &format!("{current_terms}-outdated"),
    ));

    assert!(matches!(
        result,
        Err(RegistrationError::OutdatedLegalDocument)
    ));
}

#[test]
fn accepts_valid_person_password_and_current_legal_versions() {
    let current_terms = legal_documents::current_version(legal_documents::TERMS).unwrap();
    assert!(
        RegistrationUseCase::validate(&request("Person", "Str0ng!Password", &current_terms,))
            .is_ok()
    );
}
