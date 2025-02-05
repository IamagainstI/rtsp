use abstractions::{
    extensions::{
        array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt
    }, 
    parsing::{payload_parser::PayloadParser, ParsingError}
};

use crate::{COLON_SEPARATOR, TRIM, TRIM_REF};


/// Represents the bandwidth information for a session or media description.
/// 
/// The `Bandwidth` struct corresponds to the `b=` field in SDP, which specifies
/// the bandwidth information for a session or media description.
/// 
/// According to RFC 4566, the `b=` field has the following syntax:
/// 
/// ```text
/// b=<bwtype>:<bandwidth>
/// ```
/// 
/// - `<bwtype>`: The bandwidth modifier, which specifies the interpretation of the bandwidth value.
/// - `<bandwidth>`: The bandwidth value in kilobits per second.
/// 
/// Example:
/// 
/// ```text
/// b=AS:128
/// ```
/// 
/// # Fields
/// 
/// * `index` - The bandwidth modifier, represented as a `String`.
/// * `element` - The bandwidth value, represented as a `u32`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bandwidth {
    index: String, 
    element: u32 
}

impl PayloadParser for Bandwidth {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> {
        if let Some((first, second)) = data.separate_trimmed(COLON_SEPARATOR, TRIM_REF) {
            let index = first.utf8_to_str()?.to_string();
            let element = second.utf8_to_number::<u32>()?;
            return Ok(Self::new(index, element));
        }
        Err(ParsingError::from_bytes(data))
    }
}

impl Bandwidth {
    pub fn new(index: String, element: u32) -> Self {
        Self { index, element }
    }
    
    pub fn index(&self) -> &str {
        &self.index
    }
    
    pub fn element(&self) -> u32 {
        self.element
    }
}

impl Default for Bandwidth {
    fn default() -> Self {
        Self { index: Default::default(), element: Default::default() }
    }
}
