pub(crate) mod channel;
mod clipping;
mod component;
mod ctx;
mod event;
mod global_state;
mod instance;
mod key;
mod macros;
mod native;
mod sig;
mod value;

use crate::RawEvent;
pub(crate) use channel::*;
pub(crate) use clipping::*;
pub use component::*;
pub use ctx::*;
pub(crate) use event::*;
pub use hooks_macro::*;
pub(crate) use instance::*;
use key::*;
pub use macros::*;
pub use native::*;
pub use sig::*;
use std::{
    any::TypeId,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex, OnceLock},
};
pub use value::*;

static RAW_EVENT_TX: OnceLock<std::sync::mpsc::Sender<RawEvent>> = OnceLock::new();
pub(crate) fn run_loop<C: Component>(root_component: impl Send + Sync + 'static + Fn() -> C) {
    let (raw_event_tx, raw_event_rx) = std::sync::mpsc::channel();
    RAW_EVENT_TX.set(raw_event_tx).unwrap();
    let (channel_tx, channel_rx) = std::sync::mpsc::channel();
    crate::hooks::channel::init(channel_tx);

    let tree_ctx = TreeContext::new(root_component);
    global_state::init(tree_ctx);

    global_state::tree_ctx().start(&channel_rx);

    while let Ok(event) = raw_event_rx.recv() {
        let instant = std::time::Instant::now();

        global_state::reset();
        global_state::set_raw_event(event);

        global_state::tree_ctx().on_raw_event(&channel_rx);

        let elapsed = instant.elapsed();
        if elapsed.as_millis() > 1 {
            println!(
                "Warning: Rendering took {}ms. Keep it short as possible.",
                elapsed.as_millis()
            );
        }
    }
}

pub(crate) fn on_raw_event(event: RawEvent) {
    if let Some(tx) = RAW_EVENT_TX.get() {
        tx.send(event).unwrap()
    }
}
