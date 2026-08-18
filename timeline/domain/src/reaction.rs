use crate::enums::ReactionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {

    #[serde(rename = "_id")]
    pub uuid: String,
    pub author_id: String,
    pub author_name: String,
    pub reaction_type: ReactionType,
}

impl Reaction {
    pub fn new(uuid: String, author_id: String, author_name: String, reaction_type: ReactionType) -> Self {
        Self {
            uuid,
            author_id,
            author_name,
            reaction_type,
        }
    }

}