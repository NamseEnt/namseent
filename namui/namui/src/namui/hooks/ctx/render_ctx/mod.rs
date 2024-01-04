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
    cell::UnsafeCell,
    collections::HashSet,
    fmt::Debug,
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

pub(crate) type RawEventContainer = Arc<Option<RawEvent>>;

pub struct RenderCtx {
    pub(crate) instance: Rc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) track_eq_index: AtomicUsize,
    pub(crate) updated_sigs: UnsafeCell<HashSet<SigId>>,
    children: UnsafeCell<Vec<RenderingTree>>,
    pub(crate) matrix: Matrix3x3,
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
        instance: Rc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
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
            updated_sigs: updated_sigs.into(),
            children: Default::default(),
            matrix,
            component_index: Default::default(),
            raw_event,
            clippings,
        }
    }

    #[allow(clippy::mut_from_ref)]
    fn updated_sigs(&self) -> &mut HashSet<SigId> {
        unsafe { &mut *self.updated_sigs.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn children(&self) -> &mut Vec<RenderingTree> {
        unsafe { &mut *self.children.get() }
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.updated_sigs().contains(sig_id)
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        self.updated_sigs().insert(sig_id);
    }

    fn renderer(&self) -> Renderer {
        Renderer {
            instance: self.instance.clone(),
            updated_sigs: self.updated_sigs().clone(),
        }
    }

    fn render_children(&self, key_vec: KeyVec, component: impl Component) -> RenderingTree {
        self.renderer().render(
            key_vec,
            component,
            self.matrix(),
            self.clippings.clone(),
            self.raw_event.clone(),
        )
    }

    fn get_next_component_index(&self) -> usize {
        self.component_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn matrix(&self) -> Matrix3x3 {
        self.matrix
    }

    pub(crate) fn inverse_matrix(&self) -> Matrix3x3 {
        self.matrix.inverse().unwrap()
    }

    pub(crate) fn get_channel_events_items_for(&self, sig_id: SigId) -> Vec<Item> {
        let mut ret = vec![];
        let channel_events = &mut tree_ctx_mut().channel_events;

        let mut temp_channel_events = vec![];
        std::mem::swap(&mut temp_channel_events, channel_events);

        let (equals, not_equals) = temp_channel_events
            .into_iter()
            .partition(|x| x.sig_id() == sig_id);

        ret.extend(equals);
        *channel_events = not_equals;

        ret
    }

    pub(crate) fn event_handling_enabled(&self) -> bool {
        tree_ctx_mut().event_handling_enabled()
    }
}
