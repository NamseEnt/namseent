use super::*;
use crate::*;
use derivative::Derivative;
use std::{rc::Rc, sync::OnceLock};

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct TreeContext {
    pub(crate) channel_events: Vec<Item>,
    pub(crate) is_stop_event_propagation: bool,
    pub(crate) is_cursor_determined: bool,
    pub(crate) enable_event_handling: bool,
}

static mut TREE_CTX: OnceLock<TreeContext> = OnceLock::new();
pub(crate) fn tree_ctx() -> &'static TreeContext {
    unsafe { TREE_CTX.get().unwrap() }
}

pub(crate) fn tree_ctx_mut() -> &'static mut TreeContext {
    unsafe { TREE_CTX.get_mut().unwrap() }
}

static mut RAW_EVENT: Option<RawEvent> = None;

impl TreeContext {
    pub(crate) fn init() {
        unsafe {
            TREE_CTX
                .set(Self {
                    channel_events: Default::default(),
                    is_stop_event_propagation: Default::default(),
                    is_cursor_determined: Default::default(),
                    enable_event_handling: true,
                })
                .expect("TreeContext is already initialized");
        }
    }

    pub(crate) fn start<C: Component>(
        &mut self,
        channel_rx: &std::sync::mpsc::Receiver<Item>,
        root_instance: Rc<ComponentInstance>,
        root_component: impl Send + Sync + Fn() -> C,
    ) {
        self.render_and_draw(&None, channel_rx, root_instance, root_component);
    }

    pub(crate) fn on_raw_event<C: Component>(
        &mut self,
        event: RawEvent,
        channel_rx: &std::sync::mpsc::Receiver<Item>,
        root_instance: Rc<ComponentInstance>,
        root_component: impl Send + Sync + Fn() -> C,
    ) {
        let raw_event = unsafe {
            RAW_EVENT = Some(event);
            &RAW_EVENT
        };
        self.render_and_draw(raw_event, channel_rx, root_instance, root_component);

        unsafe {
            RAW_EVENT = None;
        }
    }

    pub(crate) fn render_and_draw<C: Component>(
        &mut self,
        raw_event: RawEventContainer,
        channel_rx: &std::sync::mpsc::Receiver<Item>,
        root_instance: Rc<ComponentInstance>,
        root_component: impl Send + Sync + Fn() -> C,
    ) {
        if !system::panick::is_panicked() {
            self.is_stop_event_propagation = false;
            self.is_cursor_determined = false;

            let channel_events = channel_rx.try_iter().collect::<Vec<_>>();
            let mut updated_sigs = Default::default();

            let channel_events = filter_handle_atom_events(channel_events, &mut updated_sigs);

            self.channel_events.extend(channel_events);

            let now = std::time::Instant::now();

            let rendering_tree = self.render(
                root_component(),
                root_instance.clone(),
                updated_sigs,
                Matrix3x3::identity(),
                vec![],
                raw_event,
            );

            println!("Rendering took {:?}", now.elapsed());

            crate::system::drawer::request_draw_rendering_tree(rendering_tree);

            #[cfg(target_family = "wasm")]
            if !self
                .is_cursor_determined
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                system::mouse::set_mouse_cursor(&MouseCursor::Default);
            }

            root_instance.clear_unrendered_chidlren();
        }

        #[cfg(target_arch = "wasm32")]
        self.root_instance.inspect();
    }

    pub(crate) fn render(
        &self,
        component: impl Component,
        instance: Rc<ComponentInstance>,
        updated_sigs: Vec<SigId>,
        matrix: Matrix3x3,
        clippings: Vec<Clipping>,
        raw_event: RawEventContainer,
    ) -> RenderingTree {
        let render_ctx = RenderCtx::new(instance, updated_sigs, matrix, raw_event, clippings);

        component.render(&render_ctx);

        render_ctx.finish()
    }

    pub(crate) fn swap_enable_event_handling(&mut self, enable: bool) -> bool {
        std::mem::replace(&mut self.enable_event_handling, enable)
    }

    pub(crate) fn event_handling_enabled(&self) -> bool {
        self.enable_event_handling
    }
}

fn filter_handle_atom_events(
    channel_events: Vec<Item>,
    updated_sigs: &mut Vec<SigId>,
) -> Vec<Item> {
    let (atom_events, non_atom_events) =
        channel_events
            .into_iter()
            .partition(|channel_event| match channel_event {
                Item::SetStateItem(x) => x.sig_id().id_type == SigIdType::Atom,
            });

    for channel_event in atom_events {
        match channel_event {
            Item::SetStateItem(x) => {
                updated_sigs.push(x.sig_id());
                match x {
                    SetStateItem::Set { sig_id, value } => {
                        set_atom_value(sig_id.index, value);
                    }
                    SetStateItem::Mutate { sig_id, mutate } => {
                        mutate_atom_value(sig_id.index, mutate);
                    }
                }
            }
        }
    }

    non_atom_events
}
