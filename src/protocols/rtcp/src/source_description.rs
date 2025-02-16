use std::convert::TryFrom;

use crate::rtcp_header::RtcpHeader;
use abstractions::extensions::cast_error::CastError;

const RTCP_HEADER_SIZE: usize = 4;

/// Represents an RTCP Source Description (SDES) packet.
#[derive(Debug)]
pub struct SourceDescription<'a> {
    header: RtcpHeader,
    chunks: Vec<SdesChunk<'a>>,
}

/// Represents an SDES chunk.
#[derive(Debug)]
pub struct SdesChunk<'a> {
    ssrc: u32,
    items: Vec<SdesItem<'a>>,
}

/// Represents an SDES item.
#[derive(Debug)]
pub struct SdesItem<'a> {
    item_type: u8,
    length: u8,
    data: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for SourceDescription<'a> {
    type Error = CastError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() < RTCP_HEADER_SIZE {
            return Err(CastError::from_str("Buffer too short to contain RTCP header"));
        }

        let header = RtcpHeader::try_from(bytes)?;
        let chunk_count = header.report_count() as usize;
        let mut chunks = Vec::new();
        let mut offset = RTCP_HEADER_SIZE;

        for _ in 0..chunk_count {
            if offset + 4 > bytes.len() {
                return Err(CastError::from_str("Buffer too short to contain SDES chunk"));
            }

            let ssrc = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap());
            offset += 4;

            let mut items = Vec::new();
            while offset < bytes.len() && bytes[offset] != 0 {
                if offset + 2 > bytes.len() {
                    return Err(CastError::from_str("Buffer too short to contain SDES item"));
                }

                let item_type = bytes[offset];
                let length = bytes[offset + 1];
                if offset + 2 + length as usize > bytes.len() {
                    return Err(CastError::from_str("Buffer too short to contain SDES item data"));
                }

                let data = &bytes[offset + 2..offset + 2 + length as usize];
                items.push(SdesItem {
                    item_type,
                    length,
                    data,
                });
                offset += 2 + length as usize;
            }

            // Skip the null octet that terminates the list of items
            if offset < bytes.len() && bytes[offset] == 0 {
                offset += 1;
            }

            chunks.push(SdesChunk { ssrc, items });
        }

        Ok(Self { header, chunks })
    }
}

impl<'a> SourceDescription<'a> {
    pub fn header(&self) -> &RtcpHeader {
        &self.header
    }

    pub fn chunks(&self) -> &[SdesChunk<'a>] {
        &self.chunks
    }
}

impl<'a> SdesChunk<'a> {
    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }

    pub fn items(&self) -> &[SdesItem<'a>] {
        &self.items
    }
}

impl<'a> SdesItem<'a> {
    pub fn item_type(&self) -> u8 {
        self.item_type
    }

    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}