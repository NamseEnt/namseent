use super::*;
use crate::*;
use derivative::Derivative;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub(crate) struct TreeContext {
    pub(crate) channel_events: Arc<Mutex<Vec<Item>>>,
    pub(crate) is_stop_event_propagation: Arc<AtomicBool>,
    pub(crate) is_cursor_determined: Arc<AtomicBool>,
    pub(crate) enable_event_handling: Arc<AtomicBool>,
    #[derivative(Debug = "ignore")]
    call_root_render: Arc<dyn Fn(&hook_tree::HookTree) -> RenderingTree>,
}

impl TreeContext {
    pub(crate) fn new<C: Component>(
        root_component: impl Send + Sync + 'static + Fn() -> C,
    ) -> Self {
        let hook_tree = hook_tree::HookTree::new();
        let mut ctx = Self {
            channel_events: Default::default(),
            is_stop_event_propagation: Default::default(),
            is_cursor_determined: Default::default(),
            enable_event_handling: Arc::new(AtomicBool::new(true)),
            call_root_render: Arc::new(|_| {
                unreachable!();
            }),
        };

        ctx.call_root_render = Arc::new({
            let ctx = ctx.clone();
<<<<<<< HEAD
            let root_instance = root_instance.clone();
            move |updated_sigs, raw_event| {
                ctx.render(
                    root_component(),
                    root_instance.clone(),
                    updated_sigs,
                    Matrix3x3::identity(),
                    vec![],
                    raw_event,
                )
            }
        });
        ctx.clear_unrendered_components = Arc::new({
            let root_instance = root_instance.clone();
            move || {
                root_instance.clear_unrendered_children();
            }
=======
            move |hook_tree| ctx.render_component(root_component(), hook_tree.get_root_node())
>>>>>>> 091808c9 (Init)
        });

        ctx
    }

    pub(crate) fn start(&self, channel_rx: &std::sync::mpsc::Receiver<Item>) {
        self.render_and_draw(channel_rx);
    }

    pub(crate) fn on_raw_event(&self, channel_rx: &std::sync::mpsc::Receiver<Item>) {
        self.render_and_draw(channel_rx);
    }

    pub(crate) fn render_and_draw(&self, channel_rx: &std::sync::mpsc::Receiver<Item>) {
        if !system::panick::is_panicked() {
            self.is_stop_event_propagation
                .store(false, std::sync::atomic::Ordering::Relaxed);
            self.is_cursor_determined
                .store(false, std::sync::atomic::Ordering::Relaxed);

            let mut channel_events = channel_rx.try_iter().collect::<Vec<_>>();

            handle_atom_and_mut_state_events(&mut channel_events);

            self.channel_events.lock().unwrap().extend(channel_events);

            let rendering_tree = (self.call_root_render)();

            crate::system::drawer::request_draw_rendering_tree(rendering_tree);

            #[cfg(target_family = "wasm")]
            if !self
                .is_cursor_determined
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                system::mouse::set_mouse_cursor(&MouseCursor::Default);
            }

            hook_tree::clear_unrendered();
        }

        #[cfg(target_arch = "wasm32")]
        self.root_instance.inspect();
    }

    pub(crate) fn render_component(
        &self,
        component: impl Component,
        mut hook_node: hook_tree::HookNodeWrapper,
    ) -> RenderingTree {
        hook_node.before_render();

        let render_ctx = RenderCtx::new(hook_node);

        let render_done = component.render(&render_ctx);

        render_done.rendering_tree
    }

    pub(crate) fn enable_event_handling(&self, enable: bool) -> bool {
        self.enable_event_handling
            .swap(enable, std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn event_handling_enabled(&self) -> bool {
        self.enable_event_handling
            .load(std::sync::atomic::Ordering::SeqCst)
    }
}

fn handle_atom_and_mut_state_events(channel_events: &mut Vec<Item>) {
    channel_events.retain(|x| match x {
        Item::SetStateItem(x) => {
            if x.sig_id().id_type == SigIdType::Atom {
                global_state::updated_sigs().insert(x.sig_id());
                match x {
                    SetStateItem::Set { sig_id, value } => {
                        set_atom_value(sig_id.index, value.lock().unwrap().take().unwrap());
                    }
                    SetStateItem::Mutate { sig_id, mutate } => {
                        let mutate = mutate.lock().unwrap().take().unwrap();
                        mutate_atom_value(sig_id.index, mutate);
                    }
                }

                false
            } else {
                true
            }
        }
        Item::MutStateCalled { .. } => false,
    });
}
