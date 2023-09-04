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
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc, Mutex,
    },
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
    event_handling_disabled: AtomicBool,
    raw_event: RawEventContainer,
    clippings: Vec<Clipping>,
}

impl Drop for RenderCtx {
    fn drop(&mut self) {
        self.instance.after_render();
    }
}

impl<'a> RenderCtx {
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
            event_handling_disabled: Default::default(),
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

    pub(crate) fn return_internal(&self) -> RenderDone {
        let mut vec = vec![];
        std::mem::swap(self.children.lock().unwrap().as_mut(), &mut vec);

        RenderDone {
            rendering_tree: crate::render(vec),
        }
    }

    fn renderer(&self) -> Renderer {
        Renderer {
            instance: self.instance.clone(),
            updated_sigs: self.updated_sigs.lock().unwrap().clone(),
            tree_ctx: self.tree_ctx.clone(),
        }
    }

    fn render_children(&self, key_vec: KeyVec, component: impl Component) -> RenderingTree {
        self.renderer().render(
            key_vec,
            component,
            *self.matrix.lock().unwrap(),
            self.clippings.clone(),
        )
    }

    fn get_next_component_index(&self) -> usize {
        self.component_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn add(&'a self, key: KeyVec, component: impl Component) {
        let rendering_tree = self.render_children(key, component);
        self.children.lock().unwrap().push(rendering_tree);
    }

    pub(crate) fn inverse_matrix(&self) -> Matrix3x3 {
        self.matrix.lock().unwrap().inverse().unwrap()
    }

    fn disable_event_handling(&self) {
        self.event_handling_disabled
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
    pub(crate) fn event_handling_disabled(&self) -> bool {
        self.event_handling_disabled
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    fn enable_event_handling(&self) {
        self.event_handling_disabled
            .store(false, std::sync::atomic::Ordering::SeqCst);
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

    fn compose_inner(&self, compose: impl FnOnce(&mut ComposeCtx)) -> RenderingTree {
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        {
            let mut compose_ctx = ComposeCtx::new(
                self.tree_ctx.clone(),
                KeyVec::new_child(self.get_next_component_index()),
                *self.matrix.lock().unwrap(),
                self.renderer(),
                lazy.clone(),
                self.raw_event.clone(),
                self.clippings.clone(),
            );

            compose(&mut compose_ctx);
        }
        let rendering_tree = lazy.lock().unwrap().take().unwrap().into_rendering_tree();
        rendering_tree
    }
}
