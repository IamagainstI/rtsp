/// Represents the network type in SDP.
/// 
/// The `NetworkType` enum corresponds to the `c=` field in SDP, which specifies
/// the network type for a connection data field.
/// 
/// According to RFC 4566, the `c=` field can have the following values:
/// 
/// - `IN`: Internet.
/// 
/// Example:
/// 
/// ```text
/// c=IN IP4 192.0.2.10
/// ```
/// 
/// # Variants
/// 
/// * `Internet` - Internet network type.
#[derive(Debug, PartialEq)]
pub enum NetworkType {
    Internet,
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::Internet
    }   
}

impl NetworkType {
    pub fn from_str(value: &str) -> Option<NetworkType> {
        match value {
            "IN" => Some(NetworkType::Internet),
            _ => None,
        }
    }

    pub fn from_bytes(value: &[u8]) -> Option<NetworkType> {
        match value {
            b"IN" => Some(NetworkType::Internet),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            NetworkType::Internet => "IN",
        }
    }
}