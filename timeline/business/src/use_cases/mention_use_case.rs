use std::collections::HashSet;
use mongodb::Database;
use domain::business_error::BusinessError;
use domain::comment::Comment;
use domain::mention_notification_event::MentionNotificationEvent;
use domain::post::Post;
use crate::commons::grpc_config::GrpcConfig;
use crate::gateway::friend_gateway::FriendGateway;
use crate::use_cases::mention_notification_use_case::MentionNotificationUseCase;

pub struct MentionUseCase;

impl MentionUseCase {

	pub async fn enqueue_mentions_from_post(db: &Database,author_person_id: i32,post: &Post) -> Result<(), BusinessError> {
		log::info!("Queueing mentions found in a post {}",post.uuid);
		let mentioned_uuids: Vec<String> = post.mentions.iter().map(|m| m.mentioned_uuid.clone()).collect();
		let snippet = Self::content_snippet(&post.content);

		let events: Vec<MentionNotificationEvent> = mentioned_uuids
			.into_iter()
			.map(|mentioned_uuid| {
				MentionNotificationEvent::new(
					format!("{}:{}", post.uuid, mentioned_uuid),
					"post".to_string(),
					post.uuid.clone(),
					Some(post.uuid.clone()),
					None,
					author_person_id,
					post.author_uuid.clone(),
					post.author_name.clone(),
					mentioned_uuid,
					snippet.clone(),
				)
			})
			.collect();

		MentionNotificationUseCase::enqueue(db, events).await
	}

	pub async fn enqueue_mentions_from_comment(db: &Database,author_person_id: i32,post: &Post,comment: &Comment) -> Result<(), BusinessError> {
		let comment_id =  &comment.uuid;
		let post_id =  &post.uuid;

		let allowed_mentions =
			Self::load_allowed_friend_uuids(author_person_id, &comment.author_uuid).await?;
		let mentioned_uuids: Vec<String> = post.mentions.iter().map(|m| m.mentioned_uuid.clone()).collect();
		let snippet = Self::content_snippet(&comment.content);

		let events: Vec<MentionNotificationEvent> = mentioned_uuids
			.into_iter()
			.filter(|mentioned_uuid| allowed_mentions.contains(mentioned_uuid))
			.map(|mentioned_uuid| {
				MentionNotificationEvent::new(
					format!("{}:{}", comment_id, mentioned_uuid),
					"comment".to_string(),
					comment_id.clone(),
					Some(post_id.clone()),
					Some(comment_id.clone()),
					author_person_id,
					comment.author_uuid.clone(),
					comment.author_name.clone(),
					mentioned_uuid,
					snippet.clone(),
				)
			})
			.collect();

		MentionNotificationUseCase::enqueue(db, events).await
	}

	async fn load_allowed_friend_uuids(author_person_id: i32,author_person_uuid: &str) -> Result<HashSet<String>, BusinessError> {
		log::info!("Loading allowed friend UUIDs for author_person_id: {}", author_person_id);
		let gateway = FriendGateway::new(GrpcConfig::build_endpoint());
		let friend_uuids = gateway.find_friend_uuids(author_person_id, author_person_uuid).await?;
		Ok(friend_uuids.into_iter().collect())
	}

	fn content_snippet(content: &str) -> String {
		let trimmed = content.trim();
		let mut snippet: String = trimmed.chars().take(120).collect();
		if trimmed.chars().count() > 120 {
			snippet.push_str("...");
		}
		snippet
	}
}

#[cfg(test)]
mod tests {
	use crate::use_cases::mention_use_case::MentionUseCase;

	#[test]
	fn snippet_truncates_large_content() {
		let content = "a".repeat(130);
		let snippet = MentionUseCase::content_snippet(&content);

		assert_eq!(snippet.len(), 123);
		assert!(snippet.ends_with("..."));
	}
}