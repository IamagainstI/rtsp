pub trait Codec {
    fn samples_frequency(&self) -> i32;
    fn format(&self) -> i32;
    fn parse_fmtp(&mut self, fmtp: &[u8]) -> Result<(), std::io::Error>;
    fn new() -> Self where Self: Sized;
}