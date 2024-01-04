pub(crate) mod channel;
mod clipping;
mod component;
mod ctx;
mod event;
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
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc, OnceLock},
};
pub use value::*;

static RAW_EVENT_TX: OnceLock<std::sync::mpsc::Sender<RawEvent>> = OnceLock::new();
pub(crate) fn run_loop<C: Component>(root_component: impl Send + Sync + 'static + Fn() -> C) {
    init_component_instances();
    let (raw_event_tx, raw_event_rx) = std::sync::mpsc::channel();
    RAW_EVENT_TX.set(raw_event_tx).unwrap();
    let (channel_tx, channel_rx) = std::sync::mpsc::channel();
    crate::hooks::channel::init(channel_tx);

    let root_instance = Rc::new(ComponentInstance::new(root_component().static_type_name()));
    TreeContext::init();

    tree_ctx_mut().start(&channel_rx, root_instance.clone(), &root_component);

    while let Ok(event) = raw_event_rx.recv() {
        let instant = std::time::Instant::now();

        tree_ctx_mut().on_raw_event(event, &channel_rx, root_instance.clone(), &root_component);

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
