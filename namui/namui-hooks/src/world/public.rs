use namui_type::bytes::Buf;

use super::*;

impl World {
    pub fn init(get_now: impl Fn() -> Instant + 'static) -> Self {
        let (set_state_tx, set_state_rx) = std::sync::mpsc::channel();
        Self {
            composers: Default::default(),
            instances: Default::default(),
            frozen_instances: Default::default(),
            set_state_tx: Box::leak(Box::new(set_state_tx)),
            set_state_rx,
            updated_sig_ids: Default::default(),
            get_now: Box::new(get_now),
            record_used_sig_ids: Default::default(),
            atom_list: Default::default(),
            atom_index: Default::default(),
            raw_event: Default::default(),
            is_stop_event_propagation: Default::default(),
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

    pub fn set_frozen_states(&mut self, mut bytes: &[u8]) {
        let mut frozen_instances = self.frozen_instances.borrow_mut();
        while !bytes.is_empty() {
            let len = bytes.get_u32() as usize;
            let (slice, rest) = bytes.split_at(len);
            bytes = rest;

            let frozen_instance = FrozenInstance::from_bytes(slice);
            frozen_instances.insert(frozen_instance.id, frozen_instance);
        }
    }

    pub fn freeze_states(self) -> Vec<u8> {
        let frozen_instance_bytes = self
            .instances
            .into_map()
            .into_values()
            .map(|instance| instance.freeze())
            .collect::<Vec<Vec<u8>>>();

        let mut buffer = Vec::with_capacity(
            frozen_instance_bytes.iter().map(|x| x.len()).sum::<usize>()
                + frozen_instance_bytes.len() * 4,
        );

        use bytes::BufMut;
        for bytes in frozen_instance_bytes {
            buffer.put_u32(bytes.len() as u32);
            buffer.put_slice(&bytes);
        }

        buffer
    }
}
