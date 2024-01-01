#![allow(dead_code)]

mod app;
mod component;
mod ecs;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub fn main() {
    namui::start(&mut app::App::new(), &())
}
