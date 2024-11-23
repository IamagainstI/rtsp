use std::io;

pub trait PayloadParser<T>{
    fn parse() -> Result<T, io::Error>;
}

pub struct Test;

impl PayloadParser<Test> for Test {
    fn parse() -> Result<Test, io::Error> {
        todo!()
    }
}
