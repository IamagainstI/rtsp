use abstractions::{
    extensions::{
        array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt
    }, 
    parsing::{payload_parser::PayloadParser, ParsingError}
};

use crate::{TRIM, TRIM_REF};

pub struct Bandwidth {
    index: String, 
    element: u32 
}

impl PayloadParser<Bandwidth> for Bandwidth {
    fn parse(data: &[u8]) -> Result<Bandwidth, ParsingError> {
        if let Some((first, second)) = data.separate_trimmed(TRIM, TRIM_REF) {
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
