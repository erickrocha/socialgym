use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EntityType {
    Person,
    BusinessProfile,
}

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Person => write!(f, "person"),
            EntityType::BusinessProfile => write!(f, "business_profile"),
        }
    }
}

impl EntityType {
    pub fn from_string(s: &str) -> EntityType {
        match s {
            "person" => EntityType::Person,
            "business Profile" => EntityType::BusinessProfile,
            _ => EntityType::Person,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    #[default]
    Private,
    Professional,
    Friends,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Private => write!(f, "private"),
            Visibility::Friends => write!(f, "friends"),
            Visibility::Professional => write!(f, "professional"),
        }
    }
}

impl Visibility {
    pub fn from_string(s: &str) -> Visibility {
        match s {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            "friends" => Visibility::Friends,
            "professional" => Visibility::Professional,
            _ => Visibility::Public,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Category {
    #[default]
    Force,
    Cardio,
}

impl Category {
    pub fn to_text(&self) -> String {
        match self {
            Category::Force => "force".to_string(),
            Category::Cardio => "cardio".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Category {
        match s {
            "force" => Category::Force,
            "cardio" => Category::Cardio,
            _ => Category::Force,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReactionType {
    Like,
    Love,
    Haha,
    Wow,
    Sad,
    Angry,
}

impl Display for ReactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReactionType::Like => write!(f, "like"),
            ReactionType::Love => write!(f, "love"),
            ReactionType::Haha => write!(f, "haha"),
            ReactionType::Wow => write!(f, "wow"),
            ReactionType::Sad => write!(f, "sad"),
            ReactionType::Angry => write!(f, "angry"),
        }
    }
}

impl ReactionType {
    pub fn from_string(s: &str) -> ReactionType {
        match s {
            "love" => ReactionType::Love,
            "haha" => ReactionType::Haha,
            "wow" => ReactionType::Wow,
            "sad" => ReactionType::Sad,
            "angry" => ReactionType::Angry,
            _ => ReactionType::Like,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaType {
    Image,
    Video,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaType::Image => write!(f, "image"),
            MediaType::Video => write!(f, "video"),
        }
    }
}

impl MediaType {
    pub fn from_string(s: &str) -> MediaType {
        match s {
            "video" => MediaType::Video,
            "Video" => MediaType::Video,
            _ => MediaType::Image,
        }
    }
}
