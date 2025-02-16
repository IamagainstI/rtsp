use abstractions::extensions::cast_error::CastError;
use byteorder::{BigEndian, ByteOrder};
use std::convert::TryFrom;

use crate::{report_block::{ReportBlock, REPORT_BLOCK_SIZE}, rtcp_header::RtcpHeader};

pub(crate) const SENDER_REPORT_HEADER_SIZE: usize = 28;

/// Represents an RTCP Sender Report (SR) packet.
#[derive(Debug)]
pub struct SenderReport<'a> {
    header: RtcpHeader,
    ssrc: u32,
    ntp_timestamp: u64,
    rtp_timestamp: u32,
    packet_count: u32,
    octet_count: u32,
    report_blocks: &'a [ReportBlock],
}

impl<'a> TryFrom<&'a [u8]> for SenderReport<'a> {
    type Error = CastError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() < SENDER_REPORT_HEADER_SIZE {
            return Err(CastError::from_str("Buffer too short to contain Sender Report header"));
        }

        let header = RtcpHeader::try_from(value)?;
        let ssrc = BigEndian::read_u32(&value[4..8]);
        let ntp_timestamp = BigEndian::read_u64(&value[8..16]);
        let rtp_timestamp = BigEndian::read_u32(&value[16..20]);
        let packet_count = BigEndian::read_u32(&value[20..24]);
        let octet_count = BigEndian::read_u32(&value[24..28]);

        let report_count = header.report_count() as usize;
        let expected_size = SENDER_REPORT_HEADER_SIZE + report_count * REPORT_BLOCK_SIZE;

        if value.len() < expected_size {
            dbg!(header);
            dbg!(value);
            return Err(CastError::from_str("Buffer too short to contain all report blocks"));
        }

        let mut report_blocks = Vec::with_capacity(report_count);
        for i in 0..report_count {
            let start = SENDER_REPORT_HEADER_SIZE + i * REPORT_BLOCK_SIZE;
            let end = start + REPORT_BLOCK_SIZE;
            let block = ReportBlock::try_from(&value[start..end])?;
            report_blocks.push(block);
        }
        let report_blocks = unsafe {
            std::slice::from_raw_parts(report_blocks.as_ptr(), report_count)
        };

        Ok(Self {
            header,
            ssrc,
            ntp_timestamp,
            rtp_timestamp,
            packet_count,
            octet_count,
            report_blocks,
        })
    }
}

impl<'a> SenderReport<'a> {
    pub fn header(&self) -> &RtcpHeader {
        &self.header
    }
    
    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }
    
    pub fn ntp_timestamp(&self) -> u64 {
        self.ntp_timestamp
    }
    
    pub fn rtp_timestamp(&self) -> u32 {
        self.rtp_timestamp
    }
    
    pub fn packet_count(&self) -> u32 {
        self.packet_count
    }
    
    pub fn octet_count(&self) -> u32 {
        self.octet_count
    }
    
    pub fn report_blocks(&self) -> &[ReportBlock] {
        self.report_blocks
    }
}