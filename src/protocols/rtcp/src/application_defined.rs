use byteorder::{BigEndian, ByteOrder};
use std::convert::TryFrom;

use crate::rtcp_header::RtcpHeader;
use abstractions::extensions::cast_error::CastError;

/// Represents an RTCP Application-Defined (APP) packet.
#[derive(Debug)]
pub struct ApplicationDefined<'a> {
    header: RtcpHeader,
    subtype: u8,
    ssrc: u32,
    name: [u8; 4],
    data: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for ApplicationDefined<'a> {
    type Error = CastError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(CastError::from_str("Buffer too short to contain RTCP header and APP packet"));
        }

        let header = RtcpHeader::try_from(bytes)?;
        let subtype = (bytes[0] & 0x1F) as u8;
        let ssrc = BigEndian::read_u32(&bytes[4..8]);
        let name = [bytes[8], bytes[9], bytes[10], bytes[11]];
        let data = &bytes[12..];

        Ok(Self {
            header,
            subtype,
            ssrc,
            name,
            data,
        })
    }
}

impl<'a> ApplicationDefined<'a> {
    pub fn header(&self) -> &RtcpHeader {
        &self.header
    }

    pub fn subtype(&self) -> u8 {
        self.subtype
    }

    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }

    pub fn name(&self) -> &[u8; 4] {
        &self.name
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}