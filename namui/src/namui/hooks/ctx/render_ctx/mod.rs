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
    collections::HashSet,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

type RawEventContainer = Arc<Mutex<Option<Arc<RawEvent>>>>;

pub struct RenderCtx {
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) track_eq_index: AtomicUsize,
    pub(crate) updated_sigs: Mutex<HashSet<SigId>>,
    pub(crate) tree_ctx: TreeContext,
    children: Arc<Mutex<Vec<RenderingTree>>>,
    pub(crate) matrix: Mutex<Matrix3x3>,
    component_index: AtomicUsize,
    raw_event: RawEventContainer,
    clippings: Vec<Clipping>,
}

impl Drop for RenderCtx {
    fn drop(&mut self) {
        self.instance.after_render();
    }
}

impl RenderCtx {
    pub(crate) fn new(
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        tree_ctx: TreeContext,
        matrix: Matrix3x3,
        raw_event: RawEventContainer,
        clippings: Vec<Clipping>,
    ) -> Self {
        instance.before_render();
        Self {
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            track_eq_index: Default::default(),
            updated_sigs: Mutex::new(updated_sigs),
            tree_ctx,
            children: Default::default(),
            matrix: Mutex::new(matrix),
            component_index: Default::default(),
            raw_event,
            clippings,
        }
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.updated_sigs.lock().unwrap().contains(sig_id)
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        self.updated_sigs.lock().unwrap().insert(sig_id);
    }

    fn renderer(&self) -> Renderer {
        Renderer {
            instance: self.instance.clone(),
            updated_sigs: self.updated_sigs.lock().unwrap().clone(),
            tree_ctx: self.tree_ctx.clone(),
        }
    }

    fn render_children(&self, key_vec: KeyVec, component: impl Component) -> RenderingTree {
        self.renderer()
            .render(key_vec, component, self.matrix(), self.clippings.clone())
    }

    fn get_next_component_index(&self) -> usize {
        self.component_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn matrix(&self) -> Matrix3x3 {
        self.matrix.lock().unwrap().clone()
    }

    pub(crate) fn inverse_matrix(&self) -> Matrix3x3 {
        self.matrix.lock().unwrap().inverse().unwrap()
    }

    pub(crate) fn get_channel_events_items_for(&self, sig_id: SigId) -> Vec<Item> {
        let mut ret = vec![];
        let mut channel_events = self.tree_ctx.channel_events.lock().unwrap();

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
        self.tree_ctx.event_handling_enabled()
    }
}
