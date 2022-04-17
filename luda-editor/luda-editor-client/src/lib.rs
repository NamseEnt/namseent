use wasm_bindgen::prelude::*;
mod app;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
pub async fn start() {
    app::main().await;
} //
