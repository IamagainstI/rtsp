use super::ParsingError;

pub trait PayloadParser<T>{
    fn parse(data: &[u8]) -> Result<T, ParsingError>;
}