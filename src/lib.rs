struct Hello {
    name: String,
}

impl Hello {
    fn new(&self) -> Hello {
        Self {
            name: "World".to_string()
        }
    }
}
