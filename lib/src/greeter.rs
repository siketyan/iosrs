pub(crate) struct Greeter {
    name: String,
}

impl Greeter {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub(crate) fn greet(&self) -> String {
        format!("Hello to {} from Rust!", self.name)
    }
}
