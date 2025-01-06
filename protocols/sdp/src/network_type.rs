
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