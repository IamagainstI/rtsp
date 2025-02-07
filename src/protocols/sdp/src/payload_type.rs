use abstractions::parsing::parsing_error::ParsingError;


/// Represents the payload type in SDP.
/// 
/// The `PayloadType` enum corresponds to the `m=` field in SDP, which specifies
/// the media type for a media description.
/// 
/// According to RFC 4566, the `m=` field can have the following values:
/// 
/// - `video`: Video media type.
/// - `audio`: Audio media type.
/// - `application`: Application media type.
/// - `data`: Data media type.
/// - `control`: Control media type.
/// 
/// Example:
/// 
/// ```text
/// m=video 49170 RTP/AVP 31
/// ```
/// 
/// # Variants
/// 
/// * `Video` - Video media type.
/// * `Audio` - Audio media type.
/// * `Application` - Application media type.
/// * `Data` - Data media type.
/// * `Control` - Control media type.
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

    pub fn from_str(s: &str) -> Result<Self, ParsingError> {
        match s {
            "video" => Ok(PayloadType::Video),
            "audio" => Ok(PayloadType::Audio),
            "application" => Ok(PayloadType::Application),
            "data" => Ok(PayloadType::Data),
            "control" => Ok(PayloadType::Control),
            _ => Err(ParsingError::from_str(s)),
        }
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParsingError> {
        match bytes {
            b"video" => Ok(PayloadType::Video),
            b"audio" => Ok(PayloadType::Audio),
            b"application" => Ok(PayloadType::Application),
            b"data" => Ok(PayloadType::Data),
            b"control" => Ok(PayloadType::Control),
            _ => Err(ParsingError::from_bytes(bytes)),
        }
    }
}