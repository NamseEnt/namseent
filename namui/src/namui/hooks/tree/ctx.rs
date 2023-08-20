use super::*;
use crate::{Matrix3x3, RenderingTree};

#[derive(Clone)]
pub(crate) struct TreeContext {
    pub(crate) channel_events: Arc<Mutex<Vec<Item>>>,
    pub(crate) raw_event: Arc<Mutex<Option<Arc<RawEvent>>>>,
    call_root_render: Arc<dyn Fn(HashSet<SigId>) -> RenderingTree>,
}

unsafe impl Send for TreeContext {}
unsafe impl Sync for TreeContext {}

impl Debug for TreeContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeContext")
            .field("channel_events", &self.channel_events)
            .field("raw_event", &self.raw_event)
            .finish()
    }
}

impl TreeContext {
    pub(crate) fn new<C: Component>(
        root_component: impl Send + Sync + 'static + Fn() -> C,
    ) -> Self {
        let root_instance = Arc::new(ComponentInstance::new(root_component().static_type_name()));
        let mut ctx = Self {
            channel_events: Default::default(),
            raw_event: Default::default(),
            call_root_render: Arc::new(|_| {
                unreachable!();
            }),
        };

        ctx.call_root_render = Arc::new({
            let ctx = ctx.clone();
            move |updated_sigs| {
                ctx.render(
                    root_component(),
                    root_instance.clone(),
                    updated_sigs,
                    Matrix3x3::identity(),
                )
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
        let mut channel_events = channel::drain();

        let mut updated_sigs = Default::default();
        handle_atom_events(&mut channel_events, &mut updated_sigs);

        self.channel_events.lock().unwrap().extend(channel_events);

        let rendering_tree = (self.call_root_render)(updated_sigs);
        crate::system::drawer::request_draw_rendering_tree(rendering_tree);
    }

    pub(crate) fn render(
        &self,
        component: impl Component,
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        matrix: Matrix3x3,
    ) -> RenderingTree {
        let render_ctx = self.spawn_render_ctx(instance, updated_sigs, matrix);

        let render_done = Box::new(component).render(&render_ctx);

        render_done.rendering_tree
    }
    pub(crate) fn spawn_render_ctx(
        &self,
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        matrix: Matrix3x3,
    ) -> RenderCtx {
        RenderCtx::new(
            instance,
            updated_sigs,
            self.clone(),
            matrix,
            self.raw_event.clone(),
        )
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
