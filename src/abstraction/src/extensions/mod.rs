pub mod array_extensions;
pub mod utf8_array_extensions;

use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CastError {
    #[error("Invalid data: {0:?}")]
    InvalidData(String),
    #[error("UTF-8 error")]
    Utf8Error(#[from] Utf8Error),
}