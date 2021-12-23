use tokio::sync::RwLock;

pub struct Bundle {
    pub js: RwLock<Vec<u8>>,
    pub wasm: RwLock<Vec<u8>>,
}

impl Bundle {
    pub fn new() -> Self {
        Self {
            js: RwLock::new(Vec::new()),
            wasm: RwLock::new(Vec::new()),
        }
    }
}
