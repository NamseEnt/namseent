use crate::*;
use elsa::FrozenIndexMap;
use std::cell::Cell;

/// For Compose
pub(crate) struct Composer {
    pub(crate) compose_id_map: FrozenIndexMap<ChildKey, Box<ComposerId>>,
    pub(crate) component_child_map: FrozenIndexMap<ChildKey, Box<ComponentChildIds>>,
    last_rendered_frame: Cell<u64>,
    next_component_index: Cell<usize>,
    next_compose_index: Cell<usize>,
    pub(crate) child_key_chain: ChildKeyChain,
}

#[derive(Clone, Copy)]
pub(crate) struct ComponentChildIds {
    pub(crate) instance_id: InstanceId,
    pub(crate) composer_id: ComposerId,
}

impl Composer {
    pub(crate) fn new(child_key_chain: ChildKeyChain) -> Self {
        Self {
            compose_id_map: Default::default(),
            component_child_map: Default::default(),
            last_rendered_frame: Cell::new(0),
            next_component_index: Cell::new(0),
            next_compose_index: Cell::new(0),
            child_key_chain,
        }
    }

    pub(crate) fn mark_rendered(&self, frame: u64) -> bool {
        let first = self.last_rendered_frame.replace(frame) != frame;
        if first {
            self.next_component_index.set(0);
            self.next_compose_index.set(0);
        }
        first
    }

    pub(crate) fn is_rendered_at(&self, frame: u64) -> bool {
        self.last_rendered_frame.get() == frame
    }

    pub(crate) fn get_next_compose_index(&self) -> usize {
        let index = self.next_compose_index.get();
        self.next_compose_index.set(index + 1);
        index
    }

    pub(crate) fn get_next_component_index(&self) -> usize {
        let index = self.next_component_index.get();
        self.next_component_index.set(index + 1);
        index
    }
}
