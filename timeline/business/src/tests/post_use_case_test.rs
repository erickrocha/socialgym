use super::PostUseCase;
use domain::comment::Comment;
use domain::post::Post;
use domain::user::User;

fn test_user() -> User {
    User::new(
        "Actor Name".to_string(),
        "actor@example.com".to_string(),
        "user-uuid".to_string(),
        42,
        "actor-uuid".to_string(),
        String::new(),
        None,
    )
}

#[test]
fn page_zero_has_no_skip() {
    assert_eq!(PostUseCase::feed_skip(0, 20), 0);
}

#[test]
fn page_one_skips_one_full_page() {
    assert_eq!(PostUseCase::feed_skip(1, 20), 20);
}

#[test]
fn feed_skip_saturates_for_maximum_page_index() {
    assert_eq!(PostUseCase::feed_skip(u32::MAX, u64::MAX), u64::MAX);
}

#[test]
fn content_limit_rejects_oversized_post_and_comment_content() {
    assert!(PostUseCase::validate_content(&"x".repeat(5001)).is_err());
    assert!(PostUseCase::validate_content("valid").is_ok());
    assert!(PostUseCase::validate_content(&"é".repeat(5000)).is_ok());
    assert!(PostUseCase::validate_content(&"é".repeat(5001)).is_err());
}

#[test]
fn post_and_comment_attribution_comes_from_authenticated_user() {
    let author = test_user();
    let mut post = Post::updated(
        "post".to_string(),
        1,
        "spoofed-uuid".to_string(),
        "Spoofed".to_string(),
        None,
        None,
        "content".to_string(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut comment = Comment::new(
        "comment".to_string(),
        "post".to_string(),
        "spoofed-uuid".to_string(),
        "Spoofed".to_string(),
        None,
        None,
        "content".to_string(),
        None,
        Vec::new(),
    );

    PostUseCase::apply_post_author(&mut post, &author);
    PostUseCase::apply_comment_author(&mut comment, &author);

    assert_eq!(post.author_id, 42);
    assert_eq!(post.author_uuid, "actor-uuid");
    assert_eq!(comment.author_uuid, "actor-uuid");
    assert_eq!(comment.author_name, "Actor Name");
}
