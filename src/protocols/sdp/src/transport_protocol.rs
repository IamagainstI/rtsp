use abstractions::{extensions::utf8_array_extensions::U8ArrayExt, parsing::parsing_error::ParsingError};

#[derive(Debug, PartialEq)]
pub enum MediaTransportProtocol {
    RtpAvp,
    RtpSavp,
    Unknknown(String),
}

impl MediaTransportProtocol {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParsingError> {
        match bytes {
            b"RTP/AVP" => Ok(MediaTransportProtocol::RtpAvp),
            b"RTP/SAVP" => Ok(MediaTransportProtocol::RtpSavp),
            _ => Ok(MediaTransportProtocol::Unknknown(bytes.utf8_to_str()?.to_string())),
        }
    }
}