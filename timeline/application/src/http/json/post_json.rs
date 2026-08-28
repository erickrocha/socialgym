use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
/// Full post returned by every endpoint
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    pub author_id: i32,
    pub author_uuid: String,
    pub author_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_object_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_avatar: Option<String>,
    pub content: String,
    #[serde(default)]
    pub media: Vec<MediaJson>,
    #[serde(default)]
    pub reactions: Vec<ReactionJson>,
    #[serde(default)]
    pub comments: Vec<CommentJson>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(default)]
    pub mentions: Vec<MentionJson>,
    #[serde(default, skip_serializing)]
    pub third_party_consent_confirmed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaJson {
    pub uuid: Option<String>,
    pub url: String,
    pub media_type: String,
    pub object_key: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReactionJson {
    pub uuid: Option<String>,
    pub author_id: String,
    pub author_name: String,
    pub reaction_type: String,
}

/// A comment or reply (flat list; use parent_id to build tree on the client)
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    pub post_uuid: String,
    pub author_uuid: String,
    pub author_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_object_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_avatar: Option<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(default)]
    pub mentions: Vec<MentionJson>,
}

/// Request body to add a mention in a comment or post
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MentionJson {
    pub name: String,
    pub mentioned_uuid: String,
}
