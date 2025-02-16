use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum RtcpPacketType {
    SenderReport = 200,
    ReceiverReport = 201,
    SourceDescription = 202,
    Goodbye = 203,
    ApplicationDefined = 204,
}

impl From<u8> for RtcpPacketType {
    fn from(value: u8) -> Self {
        RtcpPacketType::from_u8(value).unwrap_or_else(|| panic!("Invalid RTCP packet type: {}", value))
    }
}