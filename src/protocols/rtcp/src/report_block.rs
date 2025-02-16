use abstractions::extensions::cast_error::CastError;
use byteorder::{BigEndian, ByteOrder};



pub(crate) const REPORT_BLOCK_SIZE: usize = 24;

/// Represents a report block in an RTCP Sender Report (SR) packet.
#[derive(Debug)]
pub struct ReportBlock {
    ssrc: u32,
    fraction_lost: u8,
    cumulative_lost: u32,
    highest_seq_num: u32,
    jitter: u32,
    last_sr: u32,
    delay_since_last_sr: u32,
}



impl ReportBlock {
    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }

    pub fn fraction_lost(&self) -> u8 {
        self.fraction_lost
    }

    pub fn cumulative_lost(&self) -> u32 {
        self.cumulative_lost
    }

    pub fn highest_seq_num(&self) -> u32 {
        self.highest_seq_num
    }

    pub fn jitter(&self) -> u32 {
        self.jitter
    }

    pub fn last_sr(&self) -> u32 {
        self.last_sr
    }

    pub fn delay_since_last_sr(&self) -> u32 {
        self.delay_since_last_sr
    }
}

impl TryFrom<&[u8]> for ReportBlock {
    type Error = CastError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < REPORT_BLOCK_SIZE {
            return Err(CastError::from_str("Buffer too short to contain Report Block"));
        }

        let ssrc = BigEndian::read_u32(&value[0..4]);
        let fraction_lost = value[4];
        let cumulative_lost = BigEndian::read_u24(&value[5..8]);
        let highest_seq_num = BigEndian::read_u32(&value[8..12]);
        let jitter = BigEndian::read_u32(&value[12..16]);
        let last_sr = BigEndian::read_u32(&value[16..20]);
        let delay_since_last_sr = BigEndian::read_u32(&value[20..24]);

        Ok(Self {
            ssrc,
            fraction_lost,
            cumulative_lost,
            highest_seq_num,
            jitter,
            last_sr,
            delay_since_last_sr,
        })
    }
}