use fluent_templates::{static_loader, Loader};
use std::fmt::{Display, Formatter};
use unic_langid::{langid, LanguageIdentifier};

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    En,
    PtBr,
    Pt,
    Es,
    Fr,
    Dutch,
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Locale::En => write!(f, "en"),
            Locale::Pt => write!(f, "pt"),
            Locale::PtBr => write!(f, "pt-BR"),
            Locale::Es => write!(f, "es"),
            Locale::Fr => write!(f, "fr"),
            Locale::Dutch => write!(f, "nl"),
        }
    }
}

impl Locale {
    pub fn from_accept_language(header: Option<&str>) -> Self {
        let lang = header
            .unwrap_or("en")
            .split(',')
            .next()
            .unwrap_or("en")
            .trim()
            .to_ascii_lowercase();

        if lang.starts_with("pt-br") {
            Locale::PtBr
        } else if lang.starts_with("pt") {
            Locale::Pt
        } else if lang.starts_with("es") {
            Locale::Es
        } else if lang.starts_with("fr") {
            Locale::Fr
        } else if lang.starts_with("nl") {
            Locale::Dutch
        } else {
            Locale::En
        }
    }

    pub fn language_id(self) -> LanguageIdentifier {
        match self {
            Locale::En => langid!("en"),
            Locale::Pt => langid!("pt"),
            Locale::PtBr => langid!("pt-BR"),
            Locale::Es => langid!("es"),
            Locale::Fr => langid!("fr"),
            Locale::Dutch => langid!("nl"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKey {
    SignUpUserFailed,

    AuthHeaderMissing,
    UnknowAuthError,
    BadCredentials,
    WeakPassword,
    AccountLocked,
    TokenRevoked,

    PersonNotFound,
    PersonNotUpdated,

    PersonInfoNotUpdated,

    PersonAddressNotUpdated,
    PersonAddressNotAdded,
    PersonAddressNotDeleted,

    ResourcesNotFound,
    PersonPreSignedUrlNotGenerated,
    PersonPreSignedUrlNotDeleted,

    BusinessProfilePreSignedUrlNotGenerated,
    BusinessProfileNotFound,
    BusinessProfileForbidden,

    RequiredParameterMissing,
    InvalidParameterValue,
    RequiredHeaderValueMissing,
    InvalidJwtToken,
    PreSignedUrlNotGenerated,

    FriendNotFound,
    FriendSendRequestFailed,
    FriendAcceptRequestFailed,
    FriendDenyRequestFailed,
    FriendCancelRequestFailed,

    TeamMemberNotFound,
    TeamMemberSendRequestFailed,
    TeamMemberAcceptRequestFailed,
    TeamMemberDenyRequestFailed,
    TeamMemberCancelRequestFailed,

    WorkoutNotFound,
    WorkoutAddFailed,
    WorkoutAcceptAssignmentFailed,
    WorkoutRejectAssignmentFailed,
    WorkoutCancelAssignmentFailed,
    ExercisesFetchFailed,
    ExercisesNotAdded,

    SettingsNotFound,
    SettingsAddedFailed,
    SettingsUpdatedFailed,

    RateLimited,

    AddressSearchFailed,
    AddressSearchDisabled,

    AccountDisabled,
    AccountDeletionNotPending,
    AccountDeletionRequestFailed,
    AccountDeletionCancelFailed,
    UnderageRegistration,
    ConsentRequired,
    ConsentOperationFailed,
    DataExportFailed,
    DataExportNotReady,
}

impl ErrorKey {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorKey::SignUpUserFailed => "SignUpUserFailed",
            ErrorKey::AuthHeaderMissing => "AuthHeaderMissing",
            ErrorKey::UnknowAuthError => "UnknowAuthError",
            ErrorKey::BadCredentials => "BadCredentials",
            ErrorKey::WeakPassword => "WeakPassword",
            ErrorKey::AccountLocked => "AccountLocked",
            ErrorKey::TokenRevoked => "TokenRevoked",
            ErrorKey::PersonNotFound => "PersonNotFound",
            ErrorKey::PersonNotUpdated => "PersonNotUpdated",
            ErrorKey::PersonInfoNotUpdated => "PersonInfoNotUpdated",
            ErrorKey::PersonAddressNotUpdated => "PersonAddressNotUpdated",
            ErrorKey::PersonAddressNotAdded => "PersonAddressNotAdded",
            ErrorKey::PersonAddressNotDeleted => "PersonAddressNotDeleted",
            ErrorKey::ResourcesNotFound => "ResourcesNotFound",
            ErrorKey::PersonPreSignedUrlNotGenerated => "PersonPreSignedUrlNotGenerated",
            ErrorKey::PersonPreSignedUrlNotDeleted => "PersonPreSignedUrlNotDeleted",
            ErrorKey::BusinessProfilePreSignedUrlNotGenerated => {
                "BusinessProfilePreSignedUrlNotGenerated"
            }
            ErrorKey::BusinessProfileNotFound => "BusinessProfileNotFound",
            ErrorKey::BusinessProfileForbidden => "BusinessProfileForbidden",
            ErrorKey::RequiredParameterMissing => "RequiredParameterMissing",
            ErrorKey::InvalidParameterValue => "InvalidParameterValue",
            ErrorKey::RequiredHeaderValueMissing => "RequiredHeaderValueMissing",
            ErrorKey::InvalidJwtToken => "InvalidJwtToken",
            ErrorKey::PreSignedUrlNotGenerated => "PreSignedUrlNotGenerated",
            ErrorKey::FriendNotFound => "FriendNotFound",
            ErrorKey::FriendSendRequestFailed => "FriendSendRequestFailed",
            ErrorKey::FriendAcceptRequestFailed => "FriendAcceptRequestFailed",
            ErrorKey::FriendDenyRequestFailed => "FriendDenyRequestFailed",
            ErrorKey::FriendCancelRequestFailed => "FriendCancelRequestFailed",
            ErrorKey::TeamMemberNotFound => "TeamMemberNotFound",
            ErrorKey::TeamMemberSendRequestFailed => "TeamMemberSendRequestFailed",
            ErrorKey::TeamMemberAcceptRequestFailed => "TeamMemberAcceptRequestFailed",
            ErrorKey::TeamMemberDenyRequestFailed => "TeamMemberDenyRequestFailed",
            ErrorKey::TeamMemberCancelRequestFailed => "TeamMemberCancelRequestFailed",
            ErrorKey::WorkoutNotFound => "WorkoutNotFound",
            ErrorKey::WorkoutAddFailed => "WorkoutAddFailed",
            ErrorKey::WorkoutAcceptAssignmentFailed => "WorkoutAcceptAssignmentFailed",
            ErrorKey::WorkoutRejectAssignmentFailed => "WorkoutRejectAssignmentFailed",
            ErrorKey::WorkoutCancelAssignmentFailed => "WorkoutCancelAssignmentFailed",
            ErrorKey::ExercisesFetchFailed => "ExercisesFetchFailed",
            ErrorKey::ExercisesNotAdded => "ExercisesNotAdded",
            ErrorKey::SettingsNotFound => "SettingsNotFound",
            ErrorKey::SettingsAddedFailed => "SettingsAddedFailed",
            ErrorKey::SettingsUpdatedFailed => "SettingsUpdatedFailed",
            ErrorKey::RateLimited => "RateLimited",
            ErrorKey::AddressSearchFailed => "AddressSearchFailed",
            ErrorKey::AddressSearchDisabled => "AddressSearchDisabled",
            ErrorKey::AccountDisabled => "AccountDisabled",
            ErrorKey::AccountDeletionNotPending => "AccountDeletionNotPending",
            ErrorKey::AccountDeletionRequestFailed => "AccountDeletionRequestFailed",
            ErrorKey::AccountDeletionCancelFailed => "AccountDeletionCancelFailed",
            ErrorKey::UnderageRegistration => "underage-registration",
            // Cross-service contract: timeline emits the same value for this
            // condition so clients can detect it with one check.
            ErrorKey::ConsentRequired => "CONSENT_REQUIRED",
            ErrorKey::ConsentOperationFailed => "consent-operation-failed",
            ErrorKey::DataExportFailed => "data-export-failed",
            ErrorKey::DataExportNotReady => "data-export-not-ready",
        }
    }

    pub fn message_id(self) -> &'static str {
        match self {
            ErrorKey::SignUpUserFailed => "sign-up-user-failed",
            ErrorKey::AuthHeaderMissing => "auth-header-missing",
            ErrorKey::UnknowAuthError => "unknown-auth-error",
            ErrorKey::BadCredentials => "bad-credentials",
            ErrorKey::WeakPassword => "weak-password",
            ErrorKey::AccountLocked => "account-locked",
            ErrorKey::TokenRevoked => "token-revoked",
            ErrorKey::PersonNotFound => "person-not-found",
            ErrorKey::PersonNotUpdated => "person-not-updated",
            ErrorKey::PersonInfoNotUpdated => "person-info-not-updated",
            ErrorKey::PersonAddressNotUpdated => "person-address-not-updated",
            ErrorKey::PersonAddressNotAdded => "person-address-not-added",
            ErrorKey::PersonAddressNotDeleted => "person-address-not-deleted",
            ErrorKey::ResourcesNotFound => "resources-not-found",
            ErrorKey::PersonPreSignedUrlNotGenerated => "person-pre-signed-url-not-generated",
            ErrorKey::PersonPreSignedUrlNotDeleted => "person-pre-signed-url-not-deleted",
            ErrorKey::BusinessProfilePreSignedUrlNotGenerated => {
                "business-profile-pre-signed-url-not-generated"
            }
            ErrorKey::BusinessProfileNotFound => "business-profile-not-found",
            ErrorKey::BusinessProfileForbidden => "business-profile-forbidden",
            ErrorKey::RequiredParameterMissing => "required-parameter-missing",
            ErrorKey::InvalidParameterValue => "invalid-parameter-value",
            ErrorKey::RequiredHeaderValueMissing => "required-header-value-missing",
            ErrorKey::InvalidJwtToken => "invalid-jwt-token",
            ErrorKey::PreSignedUrlNotGenerated => "pre-signed-url-not-generated",
            ErrorKey::FriendNotFound => "friend-not-found",
            ErrorKey::FriendSendRequestFailed => "friend-send-request-failed",
            ErrorKey::FriendAcceptRequestFailed => "friend-accept-request-failed",
            ErrorKey::FriendDenyRequestFailed => "friend-deny-request-failed",
            ErrorKey::FriendCancelRequestFailed => "friend-cancel-request-failed",
            ErrorKey::TeamMemberNotFound => "team-member-not-found",
            ErrorKey::TeamMemberSendRequestFailed => "team-member-send-request-failed",
            ErrorKey::TeamMemberAcceptRequestFailed => "team-member-accept-request-failed",
            ErrorKey::TeamMemberDenyRequestFailed => "team-member-deny-request-failed",
            ErrorKey::TeamMemberCancelRequestFailed => "team-member-cancel-request-failed",
            ErrorKey::WorkoutNotFound => "workout-not-found",
            ErrorKey::WorkoutAddFailed => "workout-add-failed",
            ErrorKey::WorkoutAcceptAssignmentFailed => "workout-accept-assignment-failed",
            ErrorKey::WorkoutRejectAssignmentFailed => "workout-reject-assignment-failed",
            ErrorKey::WorkoutCancelAssignmentFailed => "workout-cancel-assignment-failed",
            ErrorKey::ExercisesFetchFailed => "exercises-fetch-failed",
            ErrorKey::ExercisesNotAdded => "exercises-not-added",
            ErrorKey::SettingsNotFound => "settings-not-found",
            ErrorKey::SettingsAddedFailed => "settings-added-failed",
            ErrorKey::SettingsUpdatedFailed => "settings-updated-failed",
            ErrorKey::RateLimited => "rate-limited",
            ErrorKey::AddressSearchFailed => "address-search-failed",
            ErrorKey::AddressSearchDisabled => "address-search-disabled",
            ErrorKey::AccountDisabled => "account-disabled",
            ErrorKey::AccountDeletionNotPending => "account-deletion-not-pending",
            ErrorKey::AccountDeletionRequestFailed => "account-deletion-request-failed",
            ErrorKey::AccountDeletionCancelFailed => "account-deletion-cancel-failed",
            ErrorKey::UnderageRegistration => "underage-registration",
            ErrorKey::ConsentRequired => "consent-required",
            ErrorKey::ConsentOperationFailed => "consent-operation-failed",
            ErrorKey::DataExportFailed => "data-export-failed",
            ErrorKey::DataExportNotReady => "data-export-not-ready",
        }
    }
}

pub fn translate(locale: Locale, key: ErrorKey) -> String {
    let lang_id = locale.language_id();
    match LOCALES.try_lookup(&lang_id, key.message_id()) {
        Some(message) => message,
        None => {
            log::error!(
                "missing fluent translation: locale={} key={}",
                lang_id,
                key.message_id()
            );
            "An unexpected error occurred".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_LOCALES: [Locale; 6] = [
        Locale::En,
        Locale::Pt,
        Locale::PtBr,
        Locale::Es,
        Locale::Fr,
        Locale::Dutch,
    ];

    const ALL_KEYS: [ErrorKey; 41] = [
        ErrorKey::SignUpUserFailed,
        ErrorKey::AuthHeaderMissing,
        ErrorKey::UnknowAuthError,
        ErrorKey::BadCredentials,
        ErrorKey::WeakPassword,
        ErrorKey::AccountLocked,
        ErrorKey::TokenRevoked,
        ErrorKey::PersonNotFound,
        ErrorKey::PersonNotUpdated,
        ErrorKey::PersonInfoNotUpdated,
        ErrorKey::PersonAddressNotUpdated,
        ErrorKey::PersonAddressNotAdded,
        ErrorKey::PersonAddressNotDeleted,
        ErrorKey::ResourcesNotFound,
        ErrorKey::PersonPreSignedUrlNotGenerated,
        ErrorKey::PersonPreSignedUrlNotDeleted,
        ErrorKey::BusinessProfilePreSignedUrlNotGenerated,
        ErrorKey::BusinessProfileNotFound,
        ErrorKey::BusinessProfileForbidden,
        ErrorKey::RequiredParameterMissing,
        ErrorKey::InvalidParameterValue,
        ErrorKey::RequiredHeaderValueMissing,
        ErrorKey::InvalidJwtToken,
        ErrorKey::PreSignedUrlNotGenerated,
        ErrorKey::FriendNotFound,
        ErrorKey::FriendSendRequestFailed,
        ErrorKey::FriendAcceptRequestFailed,
        ErrorKey::FriendDenyRequestFailed,
        ErrorKey::FriendCancelRequestFailed,
        ErrorKey::TeamMemberNotFound,
        ErrorKey::TeamMemberSendRequestFailed,
        ErrorKey::TeamMemberAcceptRequestFailed,
        ErrorKey::TeamMemberDenyRequestFailed,
        ErrorKey::TeamMemberCancelRequestFailed,
        ErrorKey::WorkoutNotFound,
        ErrorKey::WorkoutAddFailed,
        ErrorKey::WorkoutAcceptAssignmentFailed,
        ErrorKey::WorkoutRejectAssignmentFailed,
        ErrorKey::WorkoutCancelAssignmentFailed,
        ErrorKey::ExercisesFetchFailed,
        ErrorKey::ExercisesNotAdded,
    ];

    #[test]
    fn every_error_key_has_a_translation_in_every_locale() {
        for &locale in ALL_LOCALES.iter() {
            for &key in ALL_KEYS.iter() {
                let lang_id = locale.language_id();
                assert!(
                    LOCALES.try_lookup(&lang_id, key.message_id()).is_some(),
                    "missing translation for locale {:?} key {:?}",
                    locale,
                    key
                );
            }
        }
    }

    #[test]
    fn translate_returns_expected_strings_for_a_sample_of_locales() {
        assert_eq!(
            translate(Locale::En, ErrorKey::PersonNotFound),
            "Person not found"
        );
        assert_eq!(
            translate(Locale::PtBr, ErrorKey::PersonNotFound),
            "Pessoa não encontrada"
        );
        assert_eq!(
            translate(Locale::Es, ErrorKey::BadCredentials),
            "Credenciales incorrectas"
        );
        assert_eq!(
            translate(Locale::Fr, ErrorKey::WorkoutNotFound),
            "Entraînement non trouvé"
        );
        assert_eq!(
            translate(Locale::Dutch, ErrorKey::FriendNotFound),
            "Vriend niet gevonden"
        );
    }
}
