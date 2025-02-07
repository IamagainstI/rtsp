use abstractions::parsing::parsing_error::ParsingError;

use crate::codec::Codec;

#[derive(Debug, PartialEq, Eq)]
pub struct H265Codec {
    size_length: i32,
    index_length: i32,
    index_delta_length: i32,
    
    samples_frequency: i32,
    format: i32,
}

impl Codec for H265Codec {

    fn samples_frequency(&self) -> i32 {
        todo!()
    }

    fn format(&self) -> i32 {
        todo!()
    }
    
    fn parse_fmtp(&self, fmtp: &[u8]) -> Result<(), ParsingError> {
        todo!()
    }
}