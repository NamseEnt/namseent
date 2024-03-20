mod clip_in;
mod compose_ctx;
mod method_impl;
mod public;
mod renderer;

use super::*;
use crate::{hooks::key::KeyVec, *};
pub use compose_ctx::*;
pub use method_impl::*;
use namui_type::*;
pub use public::*;
use renderer::*;
use std::{
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

pub struct RenderCtx {
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) track_eq_index: AtomicUsize,
    children: Arc<Mutex<Vec<RenderingTree>>>,
    component_index: AtomicUsize,
}

impl Drop for RenderCtx {
    fn drop(&mut self) {
        self.instance.after_render();
    }
}

impl RenderCtx {
    pub(crate) fn new(instance: Arc<ComponentInstance>) -> Self {
        instance.before_render();
        Self {
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            track_eq_index: Default::default(),
            children: Default::default(),
            component_index: Default::default(),
        }
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        global_state::updated_sigs().get(sig_id).is_some()
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        global_state::updated_sigs().insert(sig_id);
    }

    fn renderer(&self) -> Renderer {
        Renderer {
            instance: self.instance.clone(),
        }
    }

    fn render_children(&self, key_vec: KeyVec, component: impl Component) -> RenderingTree {
        self.renderer().render(key_vec, component)
    }

    fn get_next_component_index(&self) -> usize {
        self.component_index
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
}
