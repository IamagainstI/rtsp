#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayloadType {
    Video,
    Audio,
    Application,
    Data,
    Control,
}

impl PayloadType {
    pub fn as_str(&self) -> &str {
        match self {
            PayloadType::Video => "video",
            PayloadType::Audio => "audio",
            PayloadType::Application => "application",
            PayloadType::Data => "data",
            PayloadType::Control => "control",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "video" => Some(PayloadType::Video),
            "audio" => Some(PayloadType::Audio),
            "application" => Some(PayloadType::Application),
            "data" => Some(PayloadType::Data),
            "control" => Some(PayloadType::Control),
            _ => None,
        }
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"video" => Some(PayloadType::Video),
            b"audio" => Some(PayloadType::Audio),
            b"application" => Some(PayloadType::Application),
            b"data" => Some(PayloadType::Data),
            b"control" => Some(PayloadType::Control),
            _ => None,
        }
    }
}