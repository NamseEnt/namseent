mod app;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn start() {
    app::main().await;
}
