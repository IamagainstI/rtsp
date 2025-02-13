
use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt},
    parsing::{parsing_error::ParsingError, COMMA, SEMICOLON, WHITESPACE},
};
use base64::{prelude::BASE64_STANDARD, Engine};

use crate::codec::{CodecHelper, Codec};

const SPROP_KEY: &[u8] = b"sprop-parameter-sets=";
const PACKETIZATION_MODE_KEY: &[u8] = b"packetization-mode=";
const PROFILE_LEVEL_ID_KEY: &[u8] = b"profile-level-id=";

const START_MARKER: [u8; 4] = [0, 0, 0, 1];

pub(crate) const NAME: &'static str = "H264";

#[derive(Debug, PartialEq, Eq)]
pub enum PackatizationMode {
    SingleNalUnit = 0,
    NonInterleaved = 1,
    Interleaved = 2,
}

impl PackatizationMode {
    pub fn from_bits(bit: u8) -> Option<PackatizationMode> {
        match bit {
            0 => Some(PackatizationMode::SingleNalUnit),
            1 => Some(PackatizationMode::NonInterleaved),
            2 => Some(PackatizationMode::Interleaved),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct H264Codec {
    format: u16,
    clock_rate: u32,
    channel_count: Option<u8>,
    profile_level_id: String,
    packetization_mode: PackatizationMode,
    sps_pps_bytes: Vec<u8>,
}

impl H264Codec {
    pub fn new(format: u16, clock_rate: u32, channel_count: Option<u8>, profile_level_id: String, packetization_mode: PackatizationMode, sps_pps_bytes: Vec<u8>) -> Self {
        Self { format, clock_rate, channel_count, profile_level_id, packetization_mode, sps_pps_bytes }
    }
    
    pub fn profile_level_id(&self) -> &str {
        &self.profile_level_id
    }
    
    pub fn packetization_mode(&self) -> &PackatizationMode {
        &self.packetization_mode
    }
    
    pub fn sps_pps_bytes(&self) -> &[u8] {
        &self.sps_pps_bytes
    }
}

impl CodecHelper for H264Codec {
    fn from_fmtp_internal(format: u16, clock_rate: u32,channel_count: Option<u8>,data: &[u8]) -> Result<Self, abstractions::parsing::parsing_error::ParsingError> {
        let mut current: &[u8] = data;

        let mut profile_level_id: Option<String> = None;
        let mut packetization_mode: Option<PackatizationMode> = None;
        let mut sps_pps_bytes: Vec<u8> = vec![];

        let binding = [WHITESPACE, SEMICOLON].concat();
        let trim = binding.as_slice();

        while let Some((left, right)) = current.while_separate_trimmed(WHITESPACE, WHITESPACE) {
            if let Some((_, id)) = left.separate_trimmed(PROFILE_LEVEL_ID_KEY, trim) {
                profile_level_id = Some(id.utf8_to_str()?.to_string());
            } 
            else if let Some((_, bit)) = left.separate_trimmed(PACKETIZATION_MODE_KEY, trim)
            {
                packetization_mode = Some(
                    PackatizationMode::from_bits(bit.utf8_to_number::<u8>()?)
                        .ok_or_else(|| ParsingError::from_bytes(data))?,
                );
            } 
            else if let Some((_, bytes)) = left.separate_trimmed(SPROP_KEY, WHITESPACE) {
                sps_pps_bytes = get_sps_pps_bytes(bytes)?;
            }
            current = right;
        }
        if sps_pps_bytes.is_empty() {
            return Err(ParsingError::from_bytes(data));
            
        }
        let profile_level_id = profile_level_id.ok_or_else(|| ParsingError::from_bytes(data))?;
        let packetization_mode = packetization_mode.ok_or_else(|| ParsingError::from_bytes(data))?;
        
        Ok(H264Codec {
            format,
            clock_rate,
            channel_count,
            profile_level_id,
            packetization_mode,
            sps_pps_bytes,
        })
    }
}

fn get_sps_pps_bytes(data: &[u8]) -> Result<Vec<u8>, ParsingError> {
    let mut result = START_MARKER.to_vec();
    match data.separate_trimmed(COMMA, WHITESPACE) {
        Some((sps, pps)) => {
            let mut pps_start = START_MARKER.to_vec();
            result.extend(BASE64_STANDARD.decode(sps)
                    .map_err(|_| ParsingError::from_bytes(sps))?);
            pps_start.extend(BASE64_STANDARD.decode(sps)
                    .map_err(|_| ParsingError::from_bytes(pps))?);
            result.extend(pps_start);
        }
        None => {
            result.extend(BASE64_STANDARD.decode(data)
                .map_err(|_| ParsingError::from_bytes(data))?);
        }
    }
    Ok(result)
}

impl Codec for H264Codec {
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
