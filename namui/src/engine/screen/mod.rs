#[cfg(target_family = "wasm")]
mod web;
#[cfg(target_family = "wasm")]
pub use web::size;
