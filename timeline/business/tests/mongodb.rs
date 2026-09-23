use business::gateway::post_gateway::PostGateway;
use business::repositories::repository::Repository;
use domain::comment::Comment;
use domain::enums::ReactionType;
use domain::post::Post;
use domain::reaction::Reaction;
use mongodb::{bson::doc, Client, Database};

async fn test_database() -> Database {
    let url = std::env::var("TEST_MONGO_URL")
        .expect("TEST_MONGO_URL must point to a disposable MongoDB database");
    let client = Client::with_uri_str(url).await.unwrap();
    client.database("timeline_test")
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c005_post_comment_reaction_feed_acceptance() {
    let database = test_database().await;
    let posts = PostGateway::new(&database);
    database
        .collection::<Post>("posts")
        .delete_many(doc! {})
        .await
        .unwrap();

    let post = Post::updated(
        "c005-post".to_string(),
        42,
        "actor-uuid".to_string(),
        "Actor".to_string(),
        None,
        None,
        "hello timeline".to_string(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let persisted = posts.persist(post).await.unwrap();
    assert_eq!(persisted.uuid, "c005-post");

    let comment = Comment::new(
        "c005-comment".to_string(),
        "c005-post".to_string(),
        "commenter-uuid".to_string(),
        "Commenter".to_string(),
        None,
        None,
        "nice post".to_string(),
        None,
        Vec::new(),
    );
    let with_comment = posts.add_comment("c005-post", comment).await.unwrap();
    assert_eq!(with_comment.comments.len(), 1);

    let reaction = Reaction::new(
        "c005-reaction".to_string(),
        "reactor-uuid".to_string(),
        "Reactor".to_string(),
        ReactionType::Like,
    );
    let with_reaction = posts.add_reaction("c005-post", reaction).await.unwrap();
    assert_eq!(with_reaction.reactions.len(), 1);

    let feed = posts
        .find_feed(vec!["actor-uuid".to_string()], 0, 20)
        .await
        .unwrap();
    assert_eq!(feed.len(), 1);
    assert_eq!(feed[0].uuid, "c005-post");

    let without_reaction = posts
        .remove_reaction("c005-post", "reactor-uuid")
        .await
        .unwrap();
    assert!(without_reaction.reactions.is_empty());

    assert!(posts.delete("c005-post".to_string()).await.unwrap());
    assert!(posts.find_by_id("c005-post".to_string()).await.is_none());
}
