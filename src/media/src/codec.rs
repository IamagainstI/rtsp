use abstractions::parsing::parsing_error::ParsingError;

pub trait Codec {
    fn samples_frequency(&self) -> i32;
    fn format(&self) -> i32;
    fn parse_fmtp(&self, fmtp: &[u8]) -> Result<(), ParsingError>;
}