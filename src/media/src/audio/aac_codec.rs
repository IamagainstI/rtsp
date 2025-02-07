use abstractions::parsing::parsing_error::ParsingError;
use hex::encode;
use abstractions::extensions::array_extensions::ArrayExt;
use abstractions::extensions::utf8_array_extensions::U8ArrayExt;
use crate::codec::Codec;

const SIZE_LENGTH_START_STRING: &[u8] = "sizeLength=".as_bytes();
const INDEX_LENGTH_START_STRING: &[u8] = "indexLength=".as_bytes();
const INDEX_DELTA_LENGTH_START_STRING: &[u8] = "indexDeltaLength=".as_bytes();
const CONFIG_START_STRING: &[u8] = "config=".as_bytes();
const DEFAULT: &[u8] = b"";
const SEPARATOR: &[u8] = b";";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AacCodec {
    size_length: i32,
    index_length: i32,
    index_delta_length: i32,
    
    samples_frequency: i32,
    format: i32,

    config_bytes: Option<Vec<u8>>,
}

impl Codec for AacCodec {

    fn samples_frequency(&self) -> i32 {
        self.samples_frequency
    }

    fn format(&self) -> i32 {
        self.format
    }

    fn parse_fmtp(&self, fmtp: &[u8]) -> Result<(), ParsingError> {
        let mut current_span: &[u8] = fmtp;

        let mut config_parameter: &[u8] = DEFAULT;
        let mut size_length_parameter: &[u8] = DEFAULT;
        let mut index_length_parameter: &[u8] = DEFAULT;
        let mut index_delta_length_parameter: &[u8] = DEFAULT;

        while let Some((left, right)) = current_span.separate(SEPARATOR) {
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
            current_span = right;
        }
        if size_length_parameter != DEFAULT && !size_length_parameter.is_empty() &&
           index_delta_length_parameter != DEFAULT && !index_delta_length_parameter.is_empty() &&
           index_length_parameter != DEFAULT && !index_length_parameter.is_empty() {
            
            let size_length = size_length_parameter.utf8_to_number::<i32>()?;
            let index_length = index_length_parameter.utf8_to_number::<i32>()?;
            let index_delta_length = index_delta_length_parameter.utf8_to_number::<i32>()?;

            if config_parameter != DEFAULT && !config_parameter.is_empty() {
                let config_bytes = Some(config_parameter
                    .chunks(2)
                    .map(|chunk| {
                        let hex = encode(chunk);
                        u8::from_str_radix(&hex, 16).unwrap()
                    })
                    .collect::<Vec<u8>>());

                return Ok(());
            }
        }
        return  Err(ParsingError::from_bytes(fmtp));
    }
    
}