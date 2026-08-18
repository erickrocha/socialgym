    use std::fmt::Display;

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

    pub fn to_text(&self) -> String {
        match self {
            ProfileType::Professional => "Professional".to_string(),
            ProfileType::Company => "Company".to_string(),
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

    fn to_text(&self) -> String {
        match self {
            Visibility::Public => "Public".to_string(),
            Visibility::Private => "Private".to_string(),
            Visibility::Friends => "Friends".to_string(),
            Visibility::Professional => "Professional".to_string(),
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
    Speed
}

impl Category {
    pub fn to_text(&self) -> String {
        match self {
            Category::Force => "Force".to_string(),
            Category::Cardio => "Cardio".to_string(),
            Category::Hypertrophy => "Hypertrophy".to_string(),
            Category::Endurance => "Endurance".to_string(),
            Category::Power => "Power".to_string(),
            Category::Speed => "Speed".to_string(),
        }
    }

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

impl MimeType {
    /// Maps a full content-type header value (e.g. `"image/jpeg"`) to a `MimeType`.
    pub fn from_content_type(content_type: &str) -> MimeType {
        // Strip any parameters like "; charset=utf-8"
        let base = content_type.split(';').next().unwrap_or(content_type).trim();
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

    pub fn to_text(&self) -> String {
        match self {
            MimeType::Jpeg(_) => "image/jpeg".to_string(),
            MimeType::Png(_) => "image/png".to_string(),
            MimeType::Gif(_) => "image/gif".to_string(),
            MimeType::WebP(_) => "image/webp".to_string(),
            MimeType::Svg(_) => "image/svg+xml".to_string(),
            MimeType::Avif(_) => "image/avif".to_string(),
            MimeType::Mp4(_) => "video/mp4".to_string(),
            MimeType::WebM(_) => "video/webm".to_string(),
            MimeType::QuickTime(_) => "video/quicktime".to_string(),
            MimeType::Mpeg(_) => "audio/mpeg".to_string(),
            MimeType::Aac(_) => "audio/aac".to_string(),
            MimeType::Unknow(_) => "application/octet-stream".to_string(),
        }
    }
}

pub enum MediaType {
    Image, Video, Audio,
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

    pub fn to_text(&self) -> String {
        match self {
            MediaType::Image => "Image".to_string(),
            MediaType::Video => "Video".to_string(),
            MediaType::Audio => "Audio".to_string(),
        }
    }
}


#[derive(Debug, Clone)]
pub enum Position {
    Left,
    Top,
    Bottom,
    Right
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

    pub fn to_text(&self) -> String {
        match self {
            Position::Left => "Left".to_string(),
            Position::Top => "Top".to_string(),
            Position::Bottom => "Bottom".to_string(),
            Position::Right => "Right".to_string(),
        }
    }
}