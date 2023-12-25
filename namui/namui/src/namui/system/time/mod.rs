#[cfg(target_family = "wasm")]
#[cfg(not(test))]
mod web;

#[cfg(not(target_family = "wasm"))]
#[cfg(not(test))]
mod non_wasm;

#[cfg(test)]
mod mock;

#[cfg(target_family = "wasm")]
#[cfg(not(test))]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[cfg(not(test))]
pub use non_wasm::*;

#[cfg(test)]
pub use mock::*;
