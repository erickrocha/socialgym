use fluent_templates::{Loader, static_loader};
use unic_langid::{LanguageIdentifier, langid};

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
    AuthTokenInvalid,
    AuthHeaderEmpty,
    AuthTokenMissing,
    AuthTokenMalformed,

    WorkoutAddFailed,
    WorkoutNotFound,

    PostCreateFailed,
    PostDeleteFailed,
    FeedFetchFailed,
    CommentAddFailed,
    ReactionAddFailed,
    ReactionRemoveFailed,

    EvolutionCheckInAddFailed,

    RateLimited,

    InternalAuthInvalid,
    AccountDataDeletionFailed,
    ConsentRequired,

    Unknown,
}

impl ErrorKey {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorKey::AuthTokenInvalid => "AUTH_TOKEN_INVALID",
            ErrorKey::AuthHeaderEmpty => "AUTH_HEADER_EMPTY",
            ErrorKey::AuthTokenMissing => "AUTH_TOKEN_MISSING",
            ErrorKey::AuthTokenMalformed => "AUTH_TOKEN_MALFORMED",
            ErrorKey::WorkoutAddFailed => "WORKOUT_ADD_FAILED",
            ErrorKey::WorkoutNotFound => "WORKOUT_NOT_FOUND",
            ErrorKey::PostCreateFailed => "POST_CREATE_FAILED",
            ErrorKey::PostDeleteFailed => "POST_DELETE_FAILED",
            ErrorKey::FeedFetchFailed => "FEED_FETCH_FAILED",
            ErrorKey::CommentAddFailed => "COMMENT_ADD_FAILED",
            ErrorKey::ReactionAddFailed => "REACTION_ADD_FAILED",
            ErrorKey::ReactionRemoveFailed => "REACTION_REMOVE_FAILED",
            ErrorKey::EvolutionCheckInAddFailed => "EVOLUTION_CHECKIN_ADD_FAILED",
            ErrorKey::RateLimited => "RATE_LIMITED",
            ErrorKey::InternalAuthInvalid => "INTERNAL_AUTH_INVALID",
            ErrorKey::AccountDataDeletionFailed => "ACCOUNT_DATA_DELETION_FAILED",
            ErrorKey::ConsentRequired => "CONSENT_REQUIRED",
            ErrorKey::Unknown => "UNKNOWN_ERROR",
        }
    }

    pub fn message_id(self) -> &'static str {
        match self {
            ErrorKey::AuthTokenInvalid => "auth-token-invalid",
            ErrorKey::AuthHeaderEmpty => "auth-header-empty",
            ErrorKey::AuthTokenMissing => "auth-token-missing",
            ErrorKey::AuthTokenMalformed => "auth-token-malformed",
            ErrorKey::WorkoutAddFailed => "workout-add-failed",
            ErrorKey::WorkoutNotFound => "workout-not-found",
            ErrorKey::PostCreateFailed => "post-create-failed",
            ErrorKey::PostDeleteFailed => "post-delete-failed",
            ErrorKey::FeedFetchFailed => "feed-fetch-failed",
            ErrorKey::CommentAddFailed => "comment-add-failed",
            ErrorKey::ReactionAddFailed => "reaction-add-failed",
            ErrorKey::ReactionRemoveFailed => "reaction-remove-failed",
            ErrorKey::EvolutionCheckInAddFailed => "evolution-checkin-add-failed",
            ErrorKey::RateLimited => "rate-limited",
            ErrorKey::InternalAuthInvalid => "internal-auth-invalid",
            ErrorKey::AccountDataDeletionFailed => "account-data-deletion-failed",
            ErrorKey::ConsentRequired => "consent-required",
            ErrorKey::Unknown => "unknown-error",
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
#[path = "../tests/i18n_unit_test.rs"]
mod tests;
