pub trait PropertyMediaAttribute {
    fn name(&self) -> &str;
}

pub trait ValueMediaAttribute : PropertyMediaAttribute {
    fn value(&self) -> &str;
}

pub struct UnknownPropertyAttribute<'a> {
    name: &'a str,
}

pub struct UnknownValueAttribute<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> PropertyMediaAttribute for UnknownValueAttribute<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> ValueMediaAttribute for UnknownValueAttribute<'a> {
    fn value(&self) -> &str {
        self.value
    }
}

impl<'a> PropertyMediaAttribute for UnknownPropertyAttribute<'a> {
    fn name(&self) -> &str {
        self.name
    }
}