use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ImageType {
    Avatar,
    Cover,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::Avatar => write!(f, "Avatar"),
            ImageType::Cover => write!(f, "Cover"),
        }
    }
}

impl ImageType {
    pub fn from_string(s: &str) -> ImageType {
        match s {
            "Avatar" => ImageType::Avatar,
            "Cover" => ImageType::Cover,
            _ => ImageType::Avatar,
        }
    }
}

/// Consent lifecycle shared by friend requests, team-member requests and
/// workout assignments: a request is created `Pending`, then the counterpart
/// `Accepted` or `Rejected` it, or the originator `Cancelled` it.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum InviteStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
}

impl Display for InviteStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl InviteStatus {
    pub fn as_str(&self) -> String {
        match self {
            InviteStatus::Pending => "Pending".to_string(),
            InviteStatus::Accepted => "Accepted".to_string(),
            InviteStatus::Rejected => "Rejected".to_string(),
            InviteStatus::Cancelled => "Cancelled".to_string(),
        }
    }

    pub fn from_string(status: &str) -> InviteStatus {
        match status {
            "Pending" => InviteStatus::Pending,
            "Accepted" => InviteStatus::Accepted,
            "Rejected" => InviteStatus::Rejected,
            "Cancelled" => InviteStatus::Cancelled,
            _ => InviteStatus::Pending,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ProfileType {
    Professional,
    Company,
}

impl Display for ProfileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileType::Professional => write!(f, "Professional"),
            ProfileType::Company => write!(f, "Company"),
        }
    }
}

impl ProfileType {
    pub fn from_string(s: &str) -> ProfileType {
        match s {
            "Professional" => ProfileType::Professional,
            "Company" => ProfileType::Company,
            _ => ProfileType::Professional,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
    Professional,
    Friends,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        match s.to_lowercase().as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            "friends" | "friendsonly" => Visibility::Friends,
            "professional" => Visibility::Professional,
            _ => Visibility::Public,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Category {
    Force,
    Cardio,
    Hypertrophy,
    Endurance,
    Power,
    Speed,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Force => write!(f, "Force"),
            Category::Cardio => write!(f, "Cardio"),
            Category::Hypertrophy => write!(f, "Hypertrophy"),
            Category::Endurance => write!(f, "Endurance"),
            Category::Power => write!(f, "Power"),
            Category::Speed => write!(f, "Speed"),
        }
    }
}

impl Category {
    pub fn from_string(s: &str) -> Category {
        match s {
            "Force" => Category::Force,
            "Cardio" => Category::Cardio,
            "Hypertrophy" => Category::Hypertrophy,
            "Endurance" => Category::Endurance,
            "Power" => Category::Power,
            "Speed" => Category::Speed,
            _ => Category::Force,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MimeType {
    Jpeg(String),
    Png(String),
    Gif(String),
    WebP(String),
    Svg(String),
    Avif(String),
    Mpeg(String),
    Aac(String),
    Mp4(String),
    WebM(String),
    QuickTime(String),
    Unknow(String),
}

impl Display for MimeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MimeType::Jpeg(_) => write!(f, "image/jpeg"),
            MimeType::Png(_) => write!(f, "image/png"),
            MimeType::Gif(_) => write!(f, "image/gif"),
            MimeType::WebP(_) => write!(f, "image/webp"),
            MimeType::Svg(_) => write!(f, "image/svg+xml"),
            MimeType::Avif(_) => write!(f, "image/avif"),
            MimeType::Mp4(_) => write!(f, "video/mp4"),
            MimeType::WebM(_) => write!(f, "video/webm"),
            MimeType::QuickTime(_) => write!(f, "video/quicktime"),
            MimeType::Mpeg(_) => write!(f, "audio/mpeg"),
            MimeType::Aac(_) => write!(f, "audio/aac"),
            MimeType::Unknow(_) => write!(f, "application/octet-stream"),
        }
    }
}

impl MimeType {
    /// Maps a full content-type header value (e.g. `"image/jpeg"`) to a `MimeType`.
    pub fn from_content_type(content_type: &str) -> MimeType {
        // Strip any parameters like "; charset=utf-8"
        let base = content_type
            .split(';')
            .next()
            .unwrap_or(content_type)
            .trim();
        match base {
            "image/jpeg" | "image/jpg" => MimeType::Jpeg(base.to_string()),
            "image/png" => MimeType::Png(base.to_string()),
            "image/gif" => MimeType::Gif(base.to_string()),
            "image/webp" => MimeType::WebP(base.to_string()),
            "image/svg+xml" => MimeType::Svg(base.to_string()),
            "image/avif" => MimeType::Avif(base.to_string()),
            "video/mp4" => MimeType::Mp4(base.to_string()),
            "video/webm" => MimeType::WebM(base.to_string()),
            "video/quicktime" => MimeType::QuickTime(base.to_string()),
            "audio/mpeg" => MimeType::Mpeg(base.to_string()),
            "audio/aac" => MimeType::Aac(base.to_string()),
            _ => MimeType::Unknow(base.to_string()),
        }
    }

    pub fn from_string(s: &str) -> MimeType {
        match s {
            "Jpeg" => MimeType::Jpeg("image/jpeg".to_string()),
            "jpeg" => MimeType::Jpeg("image/jpeg".to_string()),
            "Png" => MimeType::Png("image/png".to_string()),
            "png" => MimeType::Png("image/png".to_string()),
            "Gif" => MimeType::Gif("image/gif".to_string()),
            "gif" => MimeType::Gif("image/gif".to_string()),
            "WebP" => MimeType::WebP("image/webp".to_string()),
            "webP" => MimeType::WebP("image/webp".to_string()),
            "Svg" => MimeType::Svg("image/svg+xml".to_string()),
            "svg" => MimeType::Svg("image/svg+xml".to_string()),
            "Avif" => MimeType::Aac("image/avif".to_string()),
            "avif" => MimeType::Aac("image/avif".to_string()),
            "Mp4" => MimeType::Mp4("video/mp4".to_string()),
            "mp4" => MimeType::Mp4("video/mp4".to_string()),
            "WebM" => MimeType::WebM("video/webm".to_string()),
            "webM" => MimeType::WebM("video/webm".to_string()),
            "QuickTime" => MimeType::QuickTime("video/quicktime".to_string()),
            "quicktime" => MimeType::QuickTime("video/quicktime".to_string()),
            "Mpeg" => MimeType::Mpeg("audio/mpeg".to_string()),
            "mpeg" => MimeType::Mpeg("audio/mpeg".to_string()),
            "Aac" => MimeType::Aac("audio/aac".to_string()),
            "aac" => MimeType::Aac("audio/aac".to_string()),
            _ => MimeType::Unknow("application/octet-stream".to_string()),
        }
    }
}

pub enum MediaType {
    Image,
    Video,
    Audio,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::Image => write!(f, "Image"),
            MediaType::Video => write!(f, "Video"),
            MediaType::Audio => write!(f, "Audio"),
        }
    }
}

impl MediaType {
    pub fn from_string(s: &str) -> MediaType {
        match s {
            "Image" => MediaType::Image,
            "Video" => MediaType::Video,
            "Audio" => MediaType::Audio,
            _ => MediaType::Image,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Position {
    Left,
    Top,
    Bottom,
    Right,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Left => write!(f, "Left"),
            Position::Top => write!(f, "Top"),
            Position::Bottom => write!(f, "Bottom"),
            Position::Right => write!(f, "Right"),
        }
    }
}

impl Position {
    pub fn from_string(s: &str) -> Position {
        match s {
            "Left" => Position::Left,
            "Top" => Position::Top,
            "Bottom" => Position::Bottom,
            "Right" => Position::Right,
            _ => Position::Left,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Difficulty {
    Soft,
    Easy,
    Medium,
    Hard,
    Strong,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Soft => write!(f, "Soft"),
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
            Difficulty::Strong => write!(f, "Strong"),
        }
    }
}

impl Difficulty {
    pub fn from_string(s: &str) -> Difficulty {
        match s {
            "Soft" => Difficulty::Soft,
            "Easy" => Difficulty::Easy,
            "Medium" => Difficulty::Medium,
            "Hard" => Difficulty::Hard,
            "Strong" => Difficulty::Strong,
            _ => Difficulty::Soft,
        }
    }
}
