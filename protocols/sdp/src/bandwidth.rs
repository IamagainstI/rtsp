use abstractions::{extensions::vector_extensions::VecExt, parsing::payload_parser::PayloadParser};

use crate::{TRIM, TRIM_REF};

pub struct Bandwidth {
    index: String, 
    element: u32 
}

impl PayloadParser<Bandwidth> for Bandwidth {
    fn parse(data: &[u8]) -> Result<Bandwidth, std::io::Error> {
        if let Some((first, second)) = data.separate_trimmed(elem, TRIM_REF) {
            let index = String::from_utf8(first)?;
            let element = u32::from_str(second)?;
            Ok(Self::new(index, element))
        }
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data"))
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
