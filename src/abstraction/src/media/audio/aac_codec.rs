use std::{io::Error, str};

use hex::encode;
use crate::extensions::array_extensions::ArrayExt;
use crate::media::codec::Codec;

const SIZE_LENGTH_START_STRING: &[u8] = "sizeLength=".as_bytes();
const INDEX_LENGTH_START_STRING: &[u8] = "indexLength=".as_bytes();
const INDEX_DELTA_LENGTH_START_STRING: &[u8] = "indexDeltaLength=".as_bytes();
const CONFIG_START_STRING: &[u8] = "config=".as_bytes();
const DEFAULT: &[u8] = b"";
const SEPARATOR: u8 = b';';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AacCodec {
    size_length: i32,
    index_length: i32,
    index_delta_length: i32,
    
    samples_frequency: i32,
    format: i32,

    config_bytes: Option<Vec<u8>>,
}

///Must add config_parameter parsing!!!
impl Codec for AacCodec {
    fn samples_frequency(&self) -> i32 {
        self.samples_frequency
    }

    fn format(&self) -> i32 {
        self.format
    }

    fn parse_fmtp(&mut self, fmtp: &[u8]) -> Result<(), std::io::Error> {
        let mut current_span: &[u8] = fmtp;

        //not parsed now
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
            
            self.size_length = str::from_utf8(size_length_parameter)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
                .and_then(|res| res.parse::<i32>().map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err)))?;
            self.index_length = str::from_utf8(index_length_parameter)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
                .and_then(|res| res.parse::<i32>().map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err)))?;
            self.index_delta_length = str::from_utf8(index_delta_length_parameter)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
                .and_then(|res| res.parse::<i32>().map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err)))?;

            if config_parameter != DEFAULT && !config_parameter.is_empty() {
                self.config_bytes = Some(config_parameter
                    .chunks(2)
                    .map(|chunk| {
                        let hex = encode(chunk);
                        u8::from_str_radix(&hex, 16).unwrap()
                    })
                    .collect::<Vec<u8>>());
                return Ok(());
            }
        }
        return  Result::Err(Error::new(std::io::ErrorKind::Other, "error"));
    }
    
    fn new() -> Self {
        AacCodec {
            size_length: 0,
            index_length: 0,
            index_delta_length: 0,
            samples_frequency: 0,
            format: 0,
            config_bytes: None,
        }
    }
}