use super::Xy;

#[cfg(target_family = "wasm")]
pub mod web;

#[cfg(target_family = "wasm")]
pub use web::*;

pub trait MouseManager {
    fn mouse_position(&self) -> Xy<i16>;
}
