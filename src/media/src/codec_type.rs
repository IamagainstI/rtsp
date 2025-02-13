use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt},
    parsing::{
        parsing_error::ParsingError, payload_parser::PayloadParser, NEW_LINE, SLASH, TRIM_NEW_LINE, WHITESPACE
    },
};

use crate::{
    audio::aac_codec::{self, AacCodec},
    codec::Codec,
    video::{h264_codec::{self, H264Codec}, h265_codec::{self, H265Codec}},
};

#[derive(Debug, PartialEq, Eq)]
pub enum CodecType {
    Aac(AacCodec),
    H265(H265Codec),
    H264(H264Codec),
    Unsupported(String),
}

impl CodecType {
    pub fn get_codec(&self) -> Option<&dyn Codec> {
        match self {
            CodecType::Aac(codec) => Some(codec),
            CodecType::H265(codec) => Some(codec),
            CodecType::H264(codec) => Some(codec),
            CodecType::Unsupported(_) => None,
        }
    }

    pub fn is_supported(&self) -> bool {
        match self {
            CodecType::Unsupported(_) => false,
            _ => true,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            CodecType::Aac(_) => aac_codec::NAME,
            CodecType::H265(_) => h265_codec::NAME,
            CodecType::H264(_) => h264_codec::NAME,
            CodecType::Unsupported(name) => name,
        }
    }
}

impl PayloadParser for CodecType {
    fn parse(data: &[u8]) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        let (rtpmap, bot) = data
            .separate_trimmed(NEW_LINE, TRIM_NEW_LINE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let (_, codec_info) = rtpmap
            .separate_trimmed(WHITESPACE, TRIM_NEW_LINE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let (name, clock_rate, chanel_count) = get_codec_info(codec_info)?;

        Ok(get_codec(
            name,
            clock_rate,
            chanel_count,
            bot,
        )?)
    }
}

fn get_codec_info(data: &[u8]) -> Result<(&str, u32, Option<u8>), ParsingError> {
    let (name, other) = data
        .separate(SLASH)
        .ok_or_else(|| ParsingError::from_bytes(data))?;
    let name = name.utf8_to_str()?;

    if let Some((clock_rate, channel_count)) = other.separate(SLASH) {
        let clock_rate = clock_rate.utf8_to_number::<u32>()?;
        let channel_count = channel_count.utf8_to_number::<u8>()?;
        return Ok((name, clock_rate, Some(channel_count)));
    }
    Ok((name, other.utf8_to_number::<u32>()?, None))
}

fn get_codec(name: &str, clock_rate: u32, chanel_count: Option<u8>,data: &[u8]) -> Result<CodecType, ParsingError> {
    let codec_type = match name.to_uppercase().as_str() {
        aac_codec::NAME => CodecType::Aac(AacCodec::parse(clock_rate, chanel_count, data)?),
        h265_codec::NAME => CodecType::H265(H265Codec::parse(clock_rate, chanel_count, data)?),
        h264_codec::NAME => CodecType::H264(H264Codec::parse(clock_rate, chanel_count, data)?),
        _ => CodecType::Unsupported(name.to_string()),
    };
    Ok(codec_type)
}
