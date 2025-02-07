use abstractions::parsing::{parsing_error::ParsingError, payload_parser::PayloadParser};

use crate::{audio::aac_codec::AacCodec, codec::Codec, video::h265_codec::H265Codec};

#[derive(Debug, PartialEq, Eq)]
pub enum CodecType {
    Aac(AacCodec),
    H265(H265Codec),
    Unsupported(String),
}

impl CodecType {
    pub fn get_codec(&self) -> Option<&dyn Codec> {
        match self {
            CodecType::Aac(codec) => Some(codec),
            CodecType::H265(codec) => Some(codec),
            CodecType::Unsupported(_) => None,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            CodecType::Aac(_) => "aac".to_string(),
            CodecType::H265(_) => "h265".to_string(),
            CodecType::Unsupported(name) => name.clone(),
        }
    }
}

impl PayloadParser for CodecType {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> where Self: Sized {
        todo!()
    }
}