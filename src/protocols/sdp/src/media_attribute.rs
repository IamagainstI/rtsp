#[derive(Debug, PartialEq)]
pub struct UnknownMediaAttribute {
    name: String,
    value: Option<String>,
}

impl UnknownMediaAttribute {
    /// Creates a new [`UnknownMediaAttribute`].
    pub fn new(name: String, value: Option<String>) -> Self {
        Self { name, value }
    }
    
    fn name(&self) -> &String {
        &self.name
    }
    fn value(&self) -> &Option<String> {
        &self.value
    }
}