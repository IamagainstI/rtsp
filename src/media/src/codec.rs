use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt}, parsing::{
        parsing_error::ParsingError, NEW_LINE, TRIM_NEW_LINE, WHITESPACE
    }
};

pub const FMTP_KEY: &[u8] = b"a=fmtp:";
pub const RTPMAP_KEY: &[u8] = b"a=rtpmap:";

pub(crate) trait CodecHelper where Self: Sized + Codec {
    fn from_fmtp_internal(format: u16, clock_rate: u32, channel_count: Option<u8>, data: &[u8]) -> Result<Self, ParsingError>;

    fn from_fmtp(clock_rate: u32, channel_count: Option<u8>, data: &[u8]) -> Result<Self, ParsingError> 
        where Self: Codec
    {
        if let Some((_, bot)) = data.separate_trimmed(FMTP_KEY, WHITESPACE) {
            let mut fmtp = if let Some((top, _)) = bot.separate_trimmed(NEW_LINE, TRIM_NEW_LINE) {
                top
            }
            else {
                bot
            };
            let (format, right) = fmtp
                .separate(WHITESPACE)
                .ok_or_else(|| ParsingError::from_bytes(data))?;
            fmtp = right;
            
            return Ok(Self::from_fmtp_internal(format.utf8_to_number::<u16>()?, clock_rate, channel_count, fmtp)?);
        }
        Err(ParsingError::from_bytes(data))
    }
}

/// Trait representing a codec.
///
/// The `Codec` trait defines the interface for working with various codecs. It includes methods
/// for retrieving the sample frequency and payload type, as well as a method for parsing codec-specific
/// data from a byte slice.
pub trait Codec {

    /// Returns the chanel count of the codec.
    ///
    /// # Returns
    ///
    /// An `i32` representing the sample frequency of the codec.
    fn channel_count(&self) -> &Option<u8>;

    /// Returns the sample frequency of the codec.
    ///
    /// # Returns
    ///
    /// An `i32` representing the sample frequency of the codec.
    fn clock_rate(&self) -> u32;

    /// Returns the payload type of the codec.
    ///
    /// # Returns
    ///
    /// A `u16` representing the payload type of the codec.
    fn format(&self) -> u16;

    /// Returns the name of codec.
    ///
    /// # Returns
    ///
    /// A `&str` representing the name of codec.
    fn name(&self) -> &'static str;

    /// Parses codec-specific data from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `clock_rate` - A `u16` representing the clock rate of the codec.
    /// * `channel_count` - An `Option<u8>` representing the number of channels (if applicable).
    /// * `data` - A byte slice containing the codec-specific data to be parsed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed codec instance if successful, or a `ParsingError` if the parsing fails.
    fn parse(clock_rate: u32, channel_count: Option<u8>, data: &[u8]) -> Result<Self, ParsingError>
        where Self: Sized;
}