#[cfg(not(test))]
mod web;

#[cfg(test)]
mod mock;

#[cfg(not(test))]
pub use web::*;

#[cfg(test)]
pub use mock::*;
