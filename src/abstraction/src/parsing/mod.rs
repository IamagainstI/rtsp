use thiserror::Error;

use crate::extensions::{utf8_array_extensions::U8ArrayExt, CastError};

pub mod payload_parser;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Invalid data: {0:?}")]
    InvalidData(String),
    #[error("UTF-8 error")]
    Utf8Error(#[from] CastError),
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

impl ParsingError {
    pub fn invalid_data(data: &str) -> Self {
        ParsingError::InvalidData(data.to_string())
    }
    pub fn from_bytes(data: &[u8]) -> Self {
        let str = match data.utf8_to_str() {
            Ok(str) => str,
            Err(e) => return ParsingError::Utf8Error(e),
        };
        ParsingError::InvalidData(str.to_string())
    }
}
