#[cfg(not(test))]
mod web;

#[cfg(test)]
mod mock;

#[cfg(not(test))]
pub use web::*;

#[cfg(test)]
pub use mock::*;

pub async fn delay(time: crate::Time) {
    fluvio_wasm_timer::Delay::new(time.as_duration())
        .await
        .unwrap();
}
