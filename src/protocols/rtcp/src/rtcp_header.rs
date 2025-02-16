use crate::rtcp_packet_type::RtcpPacketType;
use abstractions::extensions::cast_error::CastError;
use byteorder::{BigEndian, ByteOrder};
use std::fmt;

pub(crate) const RTCP_HEADER_SIZE: usize = 4;

const VERSION_MASK: u8 = 0b1100_0000;
const PADDING_MASK: u8 = 0b0010_0000;
const REPORT_COUNT_MASK: u8 = 0b0001_1111;
const VERSION_SHIFT: u8 = 6;
const PADDING_SHIFT: u8 = 5;

#[derive(PartialEq)]
pub struct RtcpHeader {
    v_p_rc: u8,
    packet_type: RtcpPacketType,
    length: u16,
}

impl RtcpHeader {
    pub fn new(
        version: u8,
        padding: bool,
        report_count: u8,
        packet_type: RtcpPacketType,
        length: u16,
    ) -> Self {
        Self {
            v_p_rc: (version << VERSION_SHIFT) | ((padding as u8) << PADDING_SHIFT) | report_count,
            packet_type,
            length,
        }
    }

    pub fn version(&self) -> u8 {
        (self.v_p_rc & VERSION_MASK) >> VERSION_SHIFT
    }

    pub fn padding(&self) -> bool {
        (self.v_p_rc & PADDING_MASK) != 0
    }

    pub fn report_count(&self) -> u8 {
        self.v_p_rc & REPORT_COUNT_MASK
    }

    pub fn packet_type(&self) -> &RtcpPacketType {
        &self.packet_type
    }

    pub fn length(&self) -> u16 {
        self.length
    }
}

impl TryFrom<&[u8]> for RtcpHeader {
    type Error = CastError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < RTCP_HEADER_SIZE {
            return Err(CastError::from_str(
                "Buffer too short to contain RTCP header",
            ));
        }

        let v_p_rc = value[0];
        let packet_type = RtcpPacketType::from(value[1]);
        let length = BigEndian::read_u16(&value[2..4]);

        Ok(Self {
            v_p_rc,
            packet_type,
            length,
        })
    }
}

impl fmt::Debug for RtcpHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RtcpHeader")
            .field("version", &self.version())
            .field("padding", &self.padding())
            .field("report_count", &self.report_count())
            .field("packet_type", &self.packet_type)
            .field("length", &self.length)
            .finish()
    }
}