pub struct MyFormat {
    pub delim: &'static str,
}

impl MyFormat {
    pub fn new() -> Self {
        Self { delim: " " }
    }
}
