use crate::parsing::ParsingError;

pub trait Codec {
    fn samples_frequency(&self) -> i32;
    fn format(&self) -> i32;
    fn from_fmtp(fmtp: &[u8]) -> Result<Self, ParsingError> where Self: Sized;
}