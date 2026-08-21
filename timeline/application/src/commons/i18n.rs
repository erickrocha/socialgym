use fluent_templates::{static_loader, Loader};
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
    FeedFetchFailed,
    CommentAddFailed,
    ReactionAddFailed,
    ReactionRemoveFailed,

    EvolutionCheckInAddFailed,

    RateLimited,

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
            ErrorKey::FeedFetchFailed => "FEED_FETCH_FAILED",
            ErrorKey::CommentAddFailed => "COMMENT_ADD_FAILED",
            ErrorKey::ReactionAddFailed => "REACTION_ADD_FAILED",
            ErrorKey::ReactionRemoveFailed => "REACTION_REMOVE_FAILED",
            ErrorKey::EvolutionCheckInAddFailed => "EVOLUTION_CHECKIN_ADD_FAILED",
            ErrorKey::RateLimited => "RATE_LIMITED",
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
            ErrorKey::FeedFetchFailed => "feed-fetch-failed",
            ErrorKey::CommentAddFailed => "comment-add-failed",
            ErrorKey::ReactionAddFailed => "reaction-add-failed",
            ErrorKey::ReactionRemoveFailed => "reaction-remove-failed",
            ErrorKey::EvolutionCheckInAddFailed => "evolution-checkin-add-failed",
            ErrorKey::RateLimited => "rate-limited",
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

    const ALL_KEYS: [ErrorKey; 13] = [
        ErrorKey::AuthTokenInvalid,
        ErrorKey::AuthHeaderEmpty,
        ErrorKey::AuthTokenMissing,
        ErrorKey::AuthTokenMalformed,
        ErrorKey::WorkoutAddFailed,
        ErrorKey::WorkoutNotFound,
        ErrorKey::PostCreateFailed,
        ErrorKey::FeedFetchFailed,
        ErrorKey::CommentAddFailed,
        ErrorKey::ReactionAddFailed,
        ErrorKey::ReactionRemoveFailed,
        ErrorKey::EvolutionCheckInAddFailed,
        ErrorKey::Unknown,
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
            translate(Locale::En, ErrorKey::WorkoutNotFound),
            "Workout session not found"
        );
        assert_eq!(
            translate(Locale::PtBr, ErrorKey::AuthTokenInvalid),
            "Token invalido"
        );
        assert_eq!(
            translate(Locale::Es, ErrorKey::Unknown),
            "Error interno inesperado"
        );
        assert_eq!(
            translate(Locale::Fr, ErrorKey::FeedFetchFailed),
            "Echec du chargement du fil"
        );
        assert_eq!(
            translate(Locale::Dutch, ErrorKey::PostCreateFailed),
            "Post aanmaken mislukt"
        );
    }
}
