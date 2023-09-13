use super::*;
use crate::*;
use derivative::Derivative;
use std::{
    collections::HashSet,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub(crate) struct TreeContext {
    pub(crate) channel_events: Arc<Mutex<Vec<Item>>>,
    pub(crate) raw_event: Arc<Mutex<Option<Arc<RawEvent>>>>,
    pub(crate) is_stop_event_propagation: Arc<AtomicBool>,
    pub(crate) is_cursor_determined: Arc<AtomicBool>,
    pub(crate) enable_event_handling: Arc<AtomicBool>,
    root_instance: Arc<ComponentInstance>,
    #[derivative(Debug = "ignore")]
    call_root_render: Arc<dyn Fn(HashSet<SigId>) -> RenderingTree>,
    #[derivative(Debug = "ignore")]
    clear_unrendered_components: Arc<dyn Fn()>,
}

unsafe impl Send for TreeContext {}
unsafe impl Sync for TreeContext {}

impl TreeContext {
    pub(crate) fn new<C: Component>(
        root_component: impl Send + Sync + 'static + Fn() -> C,
    ) -> Self {
        let root_instance = Arc::new(ComponentInstance::new(root_component().static_type_name()));
        let mut ctx = Self {
            channel_events: Default::default(),
            raw_event: Default::default(),
            is_stop_event_propagation: Default::default(),
            is_cursor_determined: Default::default(),
            enable_event_handling: Arc::new(AtomicBool::new(true)),
            root_instance: root_instance.clone(),
            call_root_render: Arc::new(|_| {
                unreachable!();
            }),
            clear_unrendered_components: Arc::new(|| {
                unreachable!();
            }),
        };

        ctx.call_root_render = Arc::new({
            let ctx = ctx.clone();
            let root_instance = root_instance.clone();
            move |updated_sigs| {
                ctx.render(
                    root_component(),
                    root_instance.clone(),
                    updated_sigs,
                    Matrix3x3::identity(),
                    vec![],
                )
            }
        });
        ctx.clear_unrendered_components = Arc::new({
            let root_instance = root_instance.clone();
            move || {
                root_instance.clear_unrendered_chidlren();
            }
        });

        ctx
    }

    pub(crate) async fn start(&self) {
        self.render_and_draw();
    }

    pub(crate) fn on_raw_event(&self, event: RawEvent) {
        self.raw_event.lock().unwrap().replace(Arc::new(event));
        self.render_and_draw();
        self.raw_event.lock().unwrap().take();
    }

    pub(crate) fn render_and_draw(&self) {
        self.is_stop_event_propagation
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.is_cursor_determined
            .store(false, std::sync::atomic::Ordering::Relaxed);

        let mut channel_events = channel::drain();

        let mut updated_sigs = Default::default();
        handle_atom_events(&mut channel_events, &mut updated_sigs);

        self.channel_events.lock().unwrap().extend(channel_events);

        let rendering_tree = (self.call_root_render)(updated_sigs);
        crate::system::drawer::request_draw_rendering_tree(rendering_tree);

        if !self
            .is_cursor_determined
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            system::mouse::set_mouse_cursor(&MouseCursor::Default);
        }

        (self.clear_unrendered_components)();

        self.root_instance.inspect();
    }

    pub(crate) fn render(
        &self,
        component: impl Component,
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        matrix: Matrix3x3,
        clippings: Vec<Clipping>,
    ) -> RenderingTree {
        let render_ctx = self.spawn_render_ctx(instance, updated_sigs, matrix, clippings);

        let render_done = Box::new(component).render(&render_ctx);

        render_done.rendering_tree
    }
    pub(crate) fn spawn_render_ctx(
        &self,
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        matrix: Matrix3x3,
        clippings: Vec<Clipping>,
    ) -> RenderCtx {
        RenderCtx::new(
            instance,
            updated_sigs,
            self.clone(),
            matrix,
            self.raw_event.clone(),
            clippings,
        )
    }

    pub(crate) fn stop_event_propagation(&self) {
        self.is_stop_event_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
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

fn handle_atom_events(channel_events: &mut Vec<Item>, updated_sigs: &mut HashSet<SigId>) {
    channel_events.retain(|x| match x {
        Item::SetStateItem(x) => {
            if x.sig_id().id_type == SigIdType::Atom {
                updated_sigs.insert(x.sig_id());
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
    });
}
