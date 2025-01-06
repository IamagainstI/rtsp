use std::io;

pub trait PayloadParser<T>{
    fn parse(data: &[u8]) -> Result<T, io::Error>;
}