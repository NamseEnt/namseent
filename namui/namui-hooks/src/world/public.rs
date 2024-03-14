use super::*;
use std::borrow::Cow;

impl World {
    pub fn init(
        get_now: impl Fn() -> Instant + 'static,
        sk_calculate: &'static dyn SkCalculate,
    ) -> Self {
        let (set_state_tx, set_state_rx) = std::sync::mpsc::channel();
        Self {
            composers: Default::default(),
            instances: Default::default(),
            set_state_tx,
            set_state_rx,
            updated_sig_ids: Default::default(),
            get_now: Box::new(get_now),
            record_used_sig_ids: Default::default(),
            atom_list: Default::default(),
            raw_event: Default::default(),
            is_stop_event_propagation: Default::default(),
            sk_calculate,
        }
    }

    pub fn run(&mut self, root_component: impl Component) -> RenderingTree {
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
            None => self
                .instances
                .insert(InstanceId::root(), Instance::new(InstanceId::root()).into()),
        };

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
