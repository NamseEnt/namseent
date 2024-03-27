mod clip_in;
mod method_impl;
mod public;
mod renderer;

use super::{hook_tree::HookNodeWrapper, *};
use crate::*;
pub use method_impl::*;
use namui_type::*;
pub use public::*;
use std::{
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

/// Component Render Context
pub struct RenderCtx {
    hook_node: HookNodeWrapper,
    pub(crate) state_index: AtomicUsize,
    pub(crate) mut_state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) interval_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) track_eq_index: AtomicUsize,
    next_component_index: AtomicUsize,
}

impl Drop for RenderCtx {
    fn drop(&mut self) {
        self.instance().after_render();
    }
}

impl RenderCtx {
    pub(crate) fn new(hook_node: HookNodeWrapper) -> Self {
        global_state::before_render_component();
        Self {
            hook_node,
            state_index: Default::default(),
            mut_state_index: Default::default(),
            effect_index: Default::default(),
            interval_index: Default::default(),
            memo_index: Default::default(),
            track_eq_index: Default::default(),
            next_component_index: Default::default(),
        }
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        global_state::updated_sigs().get(sig_id).is_some()
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        global_state::updated_sigs().insert(sig_id);
    }

    fn get_next_child_index(&self) -> usize {
        self.next_component_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn get_channel_events_items_for(&self, sig_id: SigId) -> Vec<Item> {
        let mut ret = vec![];
        let mut channel_events = global_state::tree_ctx().channel_events.lock().unwrap();

        let mut temp_channel_events = vec![];
        std::mem::swap(&mut temp_channel_events, channel_events.as_mut());

        let (equals, not_equals) = temp_channel_events
            .into_iter()
            .partition(|x| x.sig_id() == sig_id);

        ret.extend(equals);
        *channel_events = not_equals;

        ret
    }

    pub(crate) fn event_handling_enabled(&self) -> bool {
        global_state::tree_ctx().event_handling_enabled()
    }

    fn instance(&self) -> &'static mut ComponentInstance {
        self.hook_node.get_component_instance()
    }
}
