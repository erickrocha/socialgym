use super::*;

const ALL_LOCALES: [Locale; 6] = [
    Locale::En,
    Locale::Pt,
    Locale::PtBr,
    Locale::Es,
    Locale::Fr,
    Locale::Dutch,
];

const ALL_KEYS: [ErrorKey; 16] = [
    ErrorKey::AuthTokenInvalid,
    ErrorKey::AuthHeaderEmpty,
    ErrorKey::AuthTokenMissing,
    ErrorKey::AuthTokenMalformed,
    ErrorKey::WorkoutAddFailed,
    ErrorKey::WorkoutNotFound,
    ErrorKey::PostCreateFailed,
    ErrorKey::PostDeleteFailed,
    ErrorKey::FeedFetchFailed,
    ErrorKey::CommentAddFailed,
    ErrorKey::ReactionAddFailed,
    ErrorKey::ReactionRemoveFailed,
    ErrorKey::EvolutionCheckInAddFailed,
    ErrorKey::InternalAuthInvalid,
    ErrorKey::AccountDataDeletionFailed,
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
    assert_eq!(
        translate(Locale::En, ErrorKey::PostDeleteFailed),
        "Failed to delete post"
    );
}
