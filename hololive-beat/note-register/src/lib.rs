mod app;

use app::App;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub async fn main() {
    let namui_context = namui::init().await;

    namui_context.start(|| App {}).await;
}
