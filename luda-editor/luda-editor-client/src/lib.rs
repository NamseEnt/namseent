mod app;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub async fn main() {
    app::main().await;
}
