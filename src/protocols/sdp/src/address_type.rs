#[derive(Debug, PartialEq)]
pub enum AddressType {
    Ipv4,
    Ipv6,
}

impl Default for AddressType {
    fn default() -> Self {
        AddressType::Ipv4
    }
}

impl AddressType {
    pub fn from_str(value: &str) -> Option<AddressType> {
        match value {
            "IP4" => Some(AddressType::Ipv4),
            "IP6" => Some(AddressType::Ipv6),
            _ => None,
        }
    }
    
    pub fn from_bytes(value: &[u8]) -> Option<AddressType> {
        match value {
            b"IP4" => Some(AddressType::Ipv4),
            b"IP6" => Some(AddressType::Ipv6),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AddressType::Ipv4 => "IP4",
            AddressType::Ipv6 => "IP6",
        }
    }
}