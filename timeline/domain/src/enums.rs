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

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Force => write!(f, "force"),
            Category::Cardio => write!(f, "cardio"),
        }
    }
}

impl Category {
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
    /// Strict, case-insensitive parser: `Love`, `LOVE` and `love` are the same type and any other
    /// name is `None`, so an invalid value can be rejected instead of silently becoming `like`.
    pub fn parse(s: &str) -> Option<ReactionType> {
        match s.to_ascii_lowercase().as_str() {
            "like" => Some(ReactionType::Like),
            "love" => Some(ReactionType::Love),
            "haha" => Some(ReactionType::Haha),
            "wow" => Some(ReactionType::Wow),
            "sad" => Some(ReactionType::Sad),
            "angry" => Some(ReactionType::Angry),
            _ => None,
        }
    }

    /// Lenient form kept for stored data: unknown values read back as `like`.
    pub fn from_string(s: &str) -> ReactionType {
        Self::parse(s).unwrap_or(ReactionType::Like)
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
