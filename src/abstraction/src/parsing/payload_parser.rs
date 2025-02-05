use super::ParsingError;

pub trait PayloadParser {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> where Self: Sized;
}