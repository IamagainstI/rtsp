use crate::{media::codec::Codec, parsing::ParsingError};

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

    fn from_fmtp(fmtp: &[u8]) -> Result<Self, ParsingError> {
        todo!()
    }
}