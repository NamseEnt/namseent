use super::*;
use crate::{draw_rendering_tree, web::wait_web_event, Matrix3x3, RenderingTree};

#[derive(Clone, Debug)]
pub(crate) struct TreeContext {
    render_event: Arc<Mutex<RenderEvent>>,
}

#[derive(Debug)]
struct ComponentRenderQueueItem {
    component: Box<dyn Component>,
    parent_component_id: usize,
}
unsafe impl Send for ComponentRenderQueueItem {}
unsafe impl Sync for ComponentRenderQueueItem {}

const ROOT_COMPONENT_ID: usize = usize::MAX;

impl TreeContext {
    pub(crate) fn new() -> Self {
        Self {
            render_event: Arc::new(Mutex::new(RenderEvent::Mount)),
        }
    }

    pub(crate) fn start<C: Component>(self, component: impl Fn() -> C) {
        let this = Arc::new(self);
        init_render_event(RenderEvent::Mount);
        let root_instance = Arc::new(ComponentInstance::new(component().static_type_name()));
        let mut updated_sigs = None;

        loop {
            crate::system::futures::execute_async_tasks();

            let rendering_tree = this.render(
                component(),
                root_instance.clone(),
                updated_sigs.take().unwrap_or_default(),
                Matrix3x3::identity(),
            );
            draw_rendering_tree(&rendering_tree);

            // TODO: Maybe use web_event & channel event at once..?

            let mut channel_events = channel::drain();
            if !channel_events.is_empty() {
                crate::log!("channel_events: {:?}", channel_events);
                updated_sigs = {
                    let mut updated_sigs = Default::default();
                    handle_atom_events(&mut channel_events, &mut updated_sigs);
                    Some(updated_sigs)
                };
                set_render_event(RenderEvent::ChannelEvents { channel_events });
                continue;
            }

            let web_event = wait_web_event();
            set_render_event(RenderEvent::WebEvent { web_event });
        }
    }

    pub(crate) fn render(
        self: &Arc<Self>,
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
        self: &Arc<Self>,
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        matrix: Matrix3x3,
    ) -> RenderCtx {
        RenderCtx::new(instance, updated_sigs, self.clone(), matrix)
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
