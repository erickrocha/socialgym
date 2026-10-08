use super::{CommentJson, PostJson, ReactionJson};

/// The web client sends only the text (and optionally an avatar URL): the post comes from the route and
/// the author from the token, so a body without them must be accepted, not answered with `422`.
#[test]
fn a_comment_body_needs_only_its_content() {
    let comment: CommentJson = serde_json::from_str(r#"{"postId":"p1","content":"nice"}"#).expect("web payload");
    assert_eq!(comment.content, "nice");
    assert!(comment.post_uuid.is_empty() && comment.author_uuid.is_empty() && comment.author_name.is_empty());
    assert!(comment.mentions.is_empty());
}

#[test]
fn a_comment_body_without_content_is_still_refused() {
    assert!(serde_json::from_str::<CommentJson>(r#"{"postUuid":"p1"}"#).is_err());
}

#[test]
fn a_full_comment_body_keeps_every_field() {
    let comment: CommentJson = serde_json::from_str(
        r#"{"postUuid":"p1","authorUuid":"a1","authorName":"Ana","content":"hi","parentUuid":"c0","mentions":[{"name":"Bob","mentionedUuid":"b1"}]}"#,
    )
    .unwrap();
    assert_eq!((comment.post_uuid.as_str(), comment.author_uuid.as_str(), comment.author_name.as_str()), ("p1", "a1", "Ana"));
    assert_eq!((comment.parent_uuid.as_deref(), comment.mentions.len()), (Some("c0"), 1));
}

/// The web client posts only the text and, with media, the media and the consent flag.
#[test]
fn a_post_body_needs_only_its_content() {
    let post: PostJson = serde_json::from_str(r#"{"content":"hello"}"#).expect("web payload");
    assert_eq!(post.content, "hello");
    assert!((post.author_id, post.author_uuid.is_empty(), post.author_name.is_empty()) == (0, true, true));
    let with_media: PostJson = serde_json::from_str(
        r#"{"content":"","media":[{"mediaType":"Image","objectKey":"k","url":"https://s3/k"}],"thirdPartyConsentConfirmed":true}"#,
    )
    .expect("web payload with media");
    assert!(with_media.third_party_consent_confirmed);
    assert_eq!(with_media.media.len(), 1);
}

/// The web client reacts with only the reaction type.
#[test]
fn a_reaction_body_needs_only_its_type() {
    let reaction: ReactionJson = serde_json::from_str(r#"{"reactionType":"Like"}"#).expect("web payload");
    assert_eq!(reaction.reaction_type, "Like");
    assert!(reaction.author_id.is_empty() && reaction.author_name.is_empty());
}
