#![allow(dead_code)]

mod app;
mod ecs;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut app::App::new(), &()).await
}
