use std::str::Utf8Error;
use thiserror::Error;

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