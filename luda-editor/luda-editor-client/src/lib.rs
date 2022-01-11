use wasm_bindgen::prelude::*;
mod app;

#[wasm_bindgen]
pub async fn start() {
    app::main().await;
}
