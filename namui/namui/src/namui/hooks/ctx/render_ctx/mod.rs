mod clip_in;
mod compose_ctx;
mod method_impl;
mod public;
mod public_impl_inner;
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
    fmt::Debug,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub(crate) type RawEventContainer = &'static Option<RawEvent>;

pub struct RenderCtx {
    // TODO: Should inner be RC?
    inner: UnsafeCell<RenderCtxInner>,
}

pub(crate) struct RenderCtxInner {
    pub(crate) instance: Rc<ComponentInstance>,
    pub(crate) state_index: usize,
    pub(crate) effect_index: usize,
    pub(crate) memo_index: usize,
    pub(crate) track_eq_index: usize,
    pub(crate) updated_sigs: Vec<SigId>,
    children: Vec<RenderingTree>,
    pub(crate) matrix: Matrix3x3,
    component_index: usize,
    clippings: Vec<Clipping>,
    raw_event: &'static Option<RawEvent>,
    pub(crate) rendered: Option<RenderingTree>,
}

impl RenderCtx {
    pub(crate) fn new(
        instance: Rc<ComponentInstance>,
        updated_sigs: Vec<SigId>,
        matrix: Matrix3x3,
        raw_event: RawEventContainer,
        clippings: Vec<Clipping>,
    ) -> Self {
        instance.before_render();
        Self {
            inner: UnsafeCell::new(RenderCtxInner::new(
                instance,
                updated_sigs,
                matrix,
                raw_event,
                clippings,
            )),
        }
    }

    pub(crate) fn inverse_matrix(&self) -> Matrix3x3 {
        self.inner().matrix.inverse().unwrap()
    }

    pub(crate) fn event_handling_enabled(&self) -> bool {
        tree_ctx_mut().event_handling_enabled()
    }

    #[allow(clippy::mut_from_ref)]
    pub(crate) fn inner(&self) -> &mut RenderCtxInner {
        unsafe { &mut *self.inner.get() }
    }

    pub(crate) fn finish(self) -> RenderingTree {
        let inner = UnsafeCell::into_inner(self.inner);
        inner.instance.after_render();
        inner.rendered.unwrap()
    }
}

impl RenderCtxInner {
    pub(crate) fn new(
        instance: Rc<ComponentInstance>,
        updated_sigs: Vec<SigId>,
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
            updated_sigs,
            children: Default::default(),
            matrix,
            component_index: Default::default(),
            raw_event,
            clippings,
            rendered: Default::default(),
        }
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.updated_sigs.contains(sig_id)
    }

    pub(crate) fn add_sig_updated(&mut self, sig_id: SigId) {
        self.updated_sigs.push(sig_id);
    }

    fn renderer(&self) -> Renderer {
        Renderer {
            instance: self.instance.clone(),
            updated_sigs: self.updated_sigs.clone(),
        }
    }

    fn render_children(&self, key_vec: KeyVec, component: impl Component) -> RenderingTree {
        self.renderer().render(
            key_vec,
            component,
            self.matrix,
            self.clippings.clone(),
            self.raw_event,
        )
    }

    fn get_next_component_index(&mut self) -> usize {
        let index = self.component_index;
        self.component_index += 1;
        index
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

    pub(crate) fn set_rendered(&mut self, rendering_tree: RenderingTree) {
        self.rendered = Some(rendering_tree);
    }
}
