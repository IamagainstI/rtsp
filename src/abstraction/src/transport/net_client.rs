pub trait NetCLient {
    fn connect(&mut self) -> Result<(), std::io::Error>;
}
