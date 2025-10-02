use super::*;
use std::borrow::Cow;

impl World {
    pub fn init(get_now: impl Fn() -> Instant + 'static) -> Self {
        let (set_state_tx, set_state_rx) = std::sync::mpsc::channel();
        Self {
            composers: Default::default(),
            instances: Default::default(),
            set_state_tx: Box::leak(Box::new(set_state_tx)),
            set_state_rx,
            updated_sig_ids: Default::default(),
            get_now: Box::new(get_now),
            record_used_sig_ids: Default::default(),
            atom_list: Default::default(),
            atom_index: Default::default(),
            raw_event: Default::default(),
            is_stop_event_propagation: Default::default(),
            tokio_runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_stack_size(2 * 1024 * 1024)
                .max_blocking_threads(32)
                .build()
                .unwrap(),
        }
    }

    pub fn run(&mut self, root_component: impl Component) -> RenderingTree {
        self.run_impl(root_component, None)
    }

    pub fn run_with_event(
        &mut self,
        root_component: impl Component,
        event: RawEvent,
    ) -> RenderingTree {
        self.run_impl(root_component, Some(event))
    }

    fn run_impl(
        &mut self,
        root_component: impl Component,
        event: Option<RawEvent>,
    ) -> RenderingTree {
        self.is_stop_event_propagation
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.reset_updated_sig_ids();
        self.handle_set_states();

        let root_composer = match self.composers.get(&ComposerId::root()) {
            Some(composer) => composer,
            None => self
                .composers
                .insert(ComposerId::root(), Composer::new().into()),
        };

        let root_instance = match self.instances.get(&InstanceId::root()) {
            Some(instance) => instance,
            None => self.instances.insert(
                InstanceId::root(),
                Box::new(Instance::new(InstanceId::root())),
            ),
        };

        self.raw_event = event;

        let rendering_tree = render_ctx::run(
            self,
            root_component,
            root_composer,
            root_instance,
            Cow::Owned(vec![]),
        );

        self.remove_unused_guys();
        self.record_used_sig_ids.as_mut().clear();

        rendering_tree
    }
}
