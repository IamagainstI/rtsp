/// Represents the data transfer mode in SDP.
/// 
/// The `DataTransferMode` enum corresponds to the `a=` field in SDP, which specifies
/// the direction attribute for media streams.
/// 
/// According to RFC 4566, the `a=` field can have the following values:
/// 
/// - `inactive`: Neither send nor receive.
/// - `recvonly`: Receive only.
/// - `sendonly`: Send only.
/// - `sendrecv`: Send and receive.
/// 
/// Example:
/// 
/// ```text
/// a=sendrecv
/// ```
/// 
/// # Variants
/// 
/// * `Inactive` - Neither send nor receive.
/// * `Receive` - Receive only.
/// * `Send` - Send only.
/// * `SendReceive` - Send and receive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTransferMode {
    Inactive = 0b00,
    Receive = 0b01,
    Send = 0b10,
    SendReceive = 0b11,
}

impl DataTransferMode {
    pub fn from_bit(bits: u8) -> Option<Self> {
        match bits {
            0b00 => Some(DataTransferMode::Inactive),
            0b01 => Some(DataTransferMode::Receive),
            0b10 => Some(DataTransferMode::Send),
            0b11 => Some(DataTransferMode::SendReceive),
            _ => None,
        }
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"inactive" => Some(DataTransferMode::Inactive),
            b"recvonly" => Some(DataTransferMode::Receive),
            b"sendonly" => Some(DataTransferMode::Send),
            b"sendrecv" => Some(DataTransferMode::SendReceive),
            _ => None,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}