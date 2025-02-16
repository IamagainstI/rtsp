use std::str::Utf8Error;
use thiserror::Error;

use super::utf8_array_extensions::U8ArrayExt;

/// Represents errors that can occur during data casting.
///
/// The `CastError` enum defines various errors that can occur while casting data.
/// It includes errors for invalid data and UTF-8 conversion errors.
#[derive(Error, Debug)]
pub enum CastError {
    /// Represents an error for invalid data.
    ///
    /// This variant contains a string describing the invalid data.
    #[error("Invalid data: {0:?}")]
    InvalidData(String),

    /// Represents an error for UTF-8 conversion.
    ///
    /// This variant contains a `Utf8Error` that occurred during the conversion.
    #[error("UTF-8 error")]
    Utf8Error(#[from] Utf8Error),
}

impl CastError {
    /// Creates a new `CastError` from a str.
    ///
    /// # Arguments
    ///
    /// * `data` - A string slice representing the invalid data.
    ///
    /// # Returns
    ///
    /// A `CastError` instance with the `CastError` variant.
    pub fn from_str(data: &str) -> Self {
        CastError::InvalidData(data.to_string())
    }

    /// Creates a new `CastError` from a byte slice.
    ///
    /// This method attempts to convert the byte slice to a UTF-8 string. If the conversion
    /// is successful, it returns a `CastError` with the `InvalidData` variant containing
    /// the string representation of the data. If the conversion fails, it returns a
    /// `CastError` with the `Utf8Error` variant containing the `CastError`.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice representing the data.
    ///
    /// # Returns
    ///
    /// A `CastError` instance with either the `InvalidData` or `Utf8Error` variant.
    pub fn from_bytes(data: &[u8]) -> Self {
        let str = match data.utf8_to_str() {
            Ok(str) => str,
            Err(e) => return e,
        };
        CastError::InvalidData(str.to_string())
    }
}