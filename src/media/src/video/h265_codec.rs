use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt, EMPTY_BYTE_SLICE},
    parsing::{parsing_error::ParsingError, SEMICOLON, WHITESPACE},
};
use base64::{prelude::BASE64_STANDARD, Engine};

use crate::codec::{Codec, CodecHelper};

const SPS_START: &[u8] = b"sprop-sps=";
const VPS_START: &[u8] = b"sprop-vps=";
const PPS_START: &[u8] = b"sprop-pps=";
const PROFILE_ID_START: &[u8] = b"profile-id=";


pub(crate) const NAME: &'static str = "H265";

#[derive(Debug, PartialEq, Eq)]
pub struct H265Codec {
    format: u16,
    clock_rate: u32,
    channel_count: Option<u8>,
    profile_id: u16,
    sps_pps_bytes: Vec<u8>,
}

impl H265Codec {
    pub fn new(format: u16, clock_rate: u32, channel_count: Option<u8>, profile_id: u16, sps_pps_bytes: Vec<u8>) -> Self {
        Self { format, clock_rate, channel_count, profile_id, sps_pps_bytes }
    }
    
    pub fn profile_id(&self) -> u16 {
        self.profile_id
    }

    pub fn sps_pps_vps_bytes(&self) -> &[u8] {
        &self.sps_pps_bytes
    }
}

impl Codec for H265Codec {
    
    fn channel_count(&self) -> &Option<u8> {
        &self.channel_count
    }

    fn clock_rate(&self) -> u32 {
        self.clock_rate
    }

    fn format(&self) -> u16 {
        self.format
    }

    fn name(&self) -> &'static str {
        &NAME
    }

    fn parse(clock_rate: u32, channel_count: Option<u8>, data: &[u8]) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        Self::from_fmtp(clock_rate, channel_count, data)
    }
    
}



impl CodecHelper for H265Codec {
    fn from_fmtp_internal(format: u16, clock_rate: u32, channel_count: Option<u8>, data: &[u8]) -> Result<Self, ParsingError> {
        let mut sps = EMPTY_BYTE_SLICE;
        let mut pps = EMPTY_BYTE_SLICE;
        let mut vps = EMPTY_BYTE_SLICE;
        let mut profile_id: Option<u16> = None;
    
        let mut current = data;
        while let Some((left, right)) = current.while_separate_trimmed(SEMICOLON, WHITESPACE) {
            if let Some((_, sps_slice)) = left.separate_trimmed(SPS_START, WHITESPACE) {
                sps = sps_slice;
            } 
            else if let Some((_, vps_slice)) = left.separate_trimmed(VPS_START, WHITESPACE) {
                vps = vps_slice;
            } 
            else if let Some((_, pps_slice)) = left.separate_trimmed(PPS_START, WHITESPACE) {
                pps = pps_slice;
            }
            else if let Some((_, profile_id_slice)) = left.separate_trimmed(PROFILE_ID_START, WHITESPACE) {
                profile_id = Some(profile_id_slice.utf8_to_number::<u16>()?);
            }
            current = right;
        }
        if sps == EMPTY_BYTE_SLICE || pps == EMPTY_BYTE_SLICE || vps == EMPTY_BYTE_SLICE {
            return Err(ParsingError::from_bytes(data));
        }
    
        let mut sps_pps_bytes = BASE64_STANDARD
            .decode(sps)
            .map_err(|_| ParsingError::from_bytes(sps))?;
    
        let pps = BASE64_STANDARD
            .decode(pps)
            .map_err(|_| ParsingError::from_bytes(pps))?;
    
        let vps = BASE64_STANDARD
            .decode(vps)
            .map_err(|_| ParsingError::from_bytes(vps))?;
    
        sps_pps_bytes.extend(pps);
        sps_pps_bytes.extend(vps);
        let profile_id = profile_id.ok_or_else(|| ParsingError::from_bytes(data))?;

        return Ok(H265Codec {
            format,
            clock_rate,
            channel_count,
            profile_id,
            sps_pps_bytes,
        });
    }
}