use std::convert::TryFrom;

use crate::rtcp_header::RtcpHeader;
use abstractions::extensions::cast_error::CastError;
use byteorder::{BigEndian, ByteOrder};

/// Represents an RTCP Goodbye (BYE) packet.
#[derive(Debug)]
pub struct Goodbye<'a> {
    header: RtcpHeader,
    sources: &'a [u32],
    reason: Option<&'a str>,
}

impl<'a> TryFrom<&'a [u8]> for Goodbye<'a> {
    type Error = CastError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 4 {
            return Err(CastError::from_str("Buffer too short to contain RTCP header"));
        }

        let header = RtcpHeader::try_from(bytes)?;
        let source_count = header.report_count() as usize;
        let mut offset = 4;

        if bytes.len() < offset + source_count * 4 {
            return Err(CastError::from_str("Buffer too short to contain all sources"));
        }

        let sources = &bytes[offset..offset + source_count * 4];
        let mut sources_vec = Vec::with_capacity(source_count);
        for i in 0..source_count {
            let source = BigEndian::read_u32(&sources[i * 4..(i + 1) * 4]);
            sources_vec.push(source);
        }
        let sources = unsafe {
            std::slice::from_raw_parts(sources_vec.as_ptr(), sources_vec.len())
        };
        offset += source_count * 4;

        let reason = if offset < bytes.len() {
            let reason_length = bytes[offset] as usize;
            if offset + 1 + reason_length > bytes.len() {
                return Err(CastError::from_str("Buffer too short to contain reason"));
            }
            Some(std::str::from_utf8(&bytes[offset + 1..offset + 1 + reason_length])
                .map_err(|_| CastError::from_str("Invalid UTF-8 in reason"))?)
        } else {
            None
        };

        Ok(Self { header, sources, reason })
    }
}

impl<'a> Goodbye<'a> {
    pub fn header(&self) -> &RtcpHeader {
        &self.header
    }

    pub fn sources(&self) -> &[u32] {
        self.sources
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason
    }
}