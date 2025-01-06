/// Data transfer mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTransferMode {
    /// Inactive
    Inactive = 0b00,
    /// Receive only
    Receive = 0b01,
    /// Send only
    Send = 0b10,
    /// Send and receive
    SendReceive = 0b11,
}

impl DataTransferMode {
    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0b00 => Some(DataTransferMode::Inactive),
            0b01 => Some(DataTransferMode::Receive),
            0b10 => Some(DataTransferMode::Send),
            0b11 => Some(DataTransferMode::SendReceive),
            _ => None,
        }
    }

    pub fn bits(self) -> u8 {
        self as u8
    }
}