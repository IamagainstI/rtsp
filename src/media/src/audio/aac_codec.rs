use crate::codec::{Codec, CodecHelper};
use abstractions::extensions::array_extensions::ArrayExt;
use abstractions::extensions::utf8_array_extensions::U8ArrayExt;
use abstractions::extensions::EMPTY_BYTE_SLICE;
use abstractions::parsing::{SEMICOLON, WHITESPACE};
use abstractions::parsing::parsing_error::ParsingError;

const SIZE_LENGTH_START_STRING: &[u8] = b"sizeLength=";
const INDEX_LENGTH_START_STRING: &[u8] = b"indexLength=";
const INDEX_DELTA_LENGTH_START_STRING: &[u8] = b"indexDeltaLength=";
const CONFIG_START_STRING: &[u8] = b"config=";

pub(crate) const NAME: &'static str = "MPEG4-GENERIC";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AacCodec {
    format: u16,
    clock_rate: u32,
    channel_count: Option<u8>,
    size_length: i32,
    index_length: i32,
    index_delta_length: i32,
    config_bytes: Option<Vec<u8>>,
}

impl AacCodec {
    pub fn new(format: u16, clock_rate: u32, channel_count: Option<u8>, size_length: i32, index_length: i32, index_delta_length: i32, config_bytes: Option<Vec<u8>>) -> Self {
        Self { format, clock_rate, channel_count, size_length, index_length, index_delta_length, config_bytes }
    }
    
    pub fn size_length(&self) -> i32 {
        self.size_length
    }
    
    pub fn index_length(&self) -> i32 {
        self.index_length
    }
    
    pub fn index_delta_length(&self) -> i32 {
        self.index_delta_length
    }
    
    pub fn config_bytes(&self) -> Option<&Vec<u8>> {
        self.config_bytes.as_ref()
    }
}

impl CodecHelper for AacCodec {
    fn from_fmtp_internal(format: u16, clock_rate: u32, channel_count: Option<u8>, data: &[u8]) -> Result<Self, ParsingError> {
        let mut current: &[u8] = data;

        let mut config_parameter: &[u8] = EMPTY_BYTE_SLICE;
        let mut size_length_parameter: &[u8] = EMPTY_BYTE_SLICE;
        let mut index_length_parameter: &[u8] = EMPTY_BYTE_SLICE;
        let mut index_delta_length_parameter: &[u8] = EMPTY_BYTE_SLICE;

        while let Some((left, right)) = current.while_separate_trimmed(SEMICOLON, WHITESPACE) {
            if left.starts_with(SIZE_LENGTH_START_STRING) {
                size_length_parameter = &left[SIZE_LENGTH_START_STRING.len()..];
            } 
            else if left.starts_with(INDEX_LENGTH_START_STRING) {
                index_length_parameter = &left[INDEX_LENGTH_START_STRING.len()..];
            } 
            else if left.starts_with(INDEX_DELTA_LENGTH_START_STRING) {
                index_delta_length_parameter = &left[INDEX_DELTA_LENGTH_START_STRING.len()..]
            } 
            else if left.starts_with(CONFIG_START_STRING) {
                config_parameter = &left[CONFIG_START_STRING.len()..]
            }
            current = right;
        }
        if size_length_parameter != EMPTY_BYTE_SLICE
            && index_delta_length_parameter != EMPTY_BYTE_SLICE
            && index_length_parameter != EMPTY_BYTE_SLICE
        {
            let size_length = size_length_parameter.utf8_to_number::<i32>()?;
            let index_length = index_length_parameter.utf8_to_number::<i32>()?;
            let index_delta_length = index_delta_length_parameter.utf8_to_number::<i32>()?;
            if config_parameter != EMPTY_BYTE_SLICE && !config_parameter.is_empty() {
                let config_bytes = Some(
                    config_parameter
                        .chunks(2)
                        .map(|chunk| {
                            let hex = chunk.utf8_to_str()?;
                            u8::from_str_radix(hex, 16)
                                .map_err(|_| ParsingError::from_bytes(data))
                        })
                        .collect::<Result<Vec<u8>, ParsingError>>()?,
                );
                return Ok(AacCodec {
                    format,
                    clock_rate,
                    channel_count,
                    size_length,
                    index_length,
                    index_delta_length,
                    config_bytes,
                });
            }
        }
        return Err(ParsingError::from_bytes(data));
    }
}

impl Codec for AacCodec {

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
        NAME
    }

    fn parse(clock_rate: u32, channel_count: Option<u8>, data: &[u8]) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        Self::from_fmtp(clock_rate, channel_count, data)
    }
}