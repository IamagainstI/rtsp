use byteorder::{BigEndian, ByteOrder};
use std::convert::TryFrom;

use crate::{report_block::ReportBlock, rtcp_header::RtcpHeader};
use abstractions::extensions::cast_error::CastError;

pub(crate) const RECEIVER_REPORT_HEADER_SIZE: usize = 8;
pub(crate) const REPORT_BLOCK_SIZE: usize = 24;

/// Represents an RTCP Receiver Report (RR) packet.
#[derive(Debug)]
pub struct ReceiverReport<'a> {
    header: RtcpHeader,
    ssrc: u32,
    report_blocks: &'a [ReportBlock],
}

impl<'a> TryFrom<&'a [u8]> for ReceiverReport<'a> {
    type Error = CastError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() < RECEIVER_REPORT_HEADER_SIZE {
            return Err(CastError::from_str("Buffer too short to contain Receiver Report header"));
        }

        let header = RtcpHeader::try_from(value)?;
        let ssrc = BigEndian::read_u32(&value[4..8]);

        let report_count = header.report_count() as usize;
        let expected_size = RECEIVER_REPORT_HEADER_SIZE + report_count * REPORT_BLOCK_SIZE;

        if value.len() < expected_size {
            return Err(CastError::from_str("Buffer too short to contain all report blocks"));
        }

        let mut report_blocks = Vec::with_capacity(report_count);
        for i in 0..report_count {
            let start = RECEIVER_REPORT_HEADER_SIZE + i * REPORT_BLOCK_SIZE;
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
            report_blocks,
        })
    }
}

impl<'a> ReceiverReport<'a> {
    pub fn header(&self) -> &RtcpHeader {
        &self.header
    }

    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }

    pub fn report_blocks(&self) -> &[ReportBlock] {
        self.report_blocks
    }
}