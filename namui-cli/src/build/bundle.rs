pub struct Bundle {
    pub js: Vec<u8>,
    pub wasm: Vec<u8>,
}

impl Bundle {
    pub fn new() -> Self {
        Self {
            js: Vec::new(),
            wasm: Vec::new(),
        }
    }
}
