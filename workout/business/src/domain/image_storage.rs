use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response after successfully storing an image
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageStorage {
    /// The URL to access the stored image
    pub url: String,
    /// The object key in S3 bucket
    pub object_key: String,
    /// The ID of the owner of the image
    pub owner_id: i32,
}

impl ImageStorage {
    /// Create a new ImageStorage instance
    pub fn new(url: String, object_key: String, owner_id: i32) -> Self {
        Self {
            url,
            object_key,
            owner_id,
        }
    }
}
