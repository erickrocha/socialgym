use super::MentionUseCase;

#[test]
fn snippet_trims_outer_whitespace_without_truncating() {
    assert_eq!(
        MentionUseCase::content_snippet("  short content \n"),
        "short content"
    );
}

#[test]
fn snippet_preserves_exactly_120_unicode_characters_without_suffix() {
    let content = "é".repeat(120);
    let snippet = MentionUseCase::content_snippet(&content);

    assert_eq!(snippet.chars().count(), 120);
    assert_eq!(snippet, content);
}

#[test]
fn snippet_truncates_after_120_unicode_characters() {
    let content = "é".repeat(121);
    let snippet = MentionUseCase::content_snippet(&content);

    assert_eq!(snippet.chars().count(), 123);
    assert_eq!(snippet, format!("{}...", "é".repeat(120)));
}

#[test]
fn snippet_of_whitespace_is_empty() {
    assert_eq!(MentionUseCase::content_snippet(" \n\t "), "");
}
