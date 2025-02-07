use thiserror::Error;

use crate::extensions::{cast_error::CastError, utf8_array_extensions::U8ArrayExt};

/// Represents errors that can occur during parsing.
///
/// The `ParsingError` enum defines various errors that can occur while parsing data.
/// It includes errors for invalid data and UTF-8 conversion errors.
#[derive(Error, Debug)]
pub enum ParsingError {
    /// Represents an error for invalid data.
    ///
    /// This variant contains a string describing the invalid data.
    #[error("Invalid data: {0:?}")]
    InvalidData(String),

    /// Represents an error for UTF-8 conversion.
    ///
    /// This variant contains a `CastError` that occurred during the conversion.
    #[error("UTF-8 error")]
    Utf8Error(#[from] CastError),
}

impl ParsingError {
    /// Creates a new `ParsingError` from a str.
    ///
    /// # Arguments
    ///
    /// * `data` - A string slice representing the invalid data.
    ///
    /// # Returns
    ///
    /// A `ParsingError` instance with the `InvalidData` variant.
    pub fn from_str(data: &str) -> Self {
        ParsingError::InvalidData(data.to_string())
    }

    /// Creates a new `ParsingError` from a byte slice.
    ///
    /// This method attempts to convert the byte slice to a UTF-8 string. If the conversion
    /// is successful, it returns a `ParsingError` with the `InvalidData` variant containing
    /// the string representation of the data. If the conversion fails, it returns a
    /// `ParsingError` with the `Utf8Error` variant containing the `CastError`.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice representing the data.
    ///
    /// # Returns
    ///
    /// A `ParsingError` instance with either the `InvalidData` or `Utf8Error` variant.
    pub fn from_bytes(data: &[u8]) -> Self {
        let str = match data.utf8_to_str() {
            Ok(str) => str,
            Err(e) => return ParsingError::Utf8Error(e),
        };
        ParsingError::InvalidData(str.to_string())
    }
}