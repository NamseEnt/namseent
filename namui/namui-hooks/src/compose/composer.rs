use crate::*;
use elsa::FrozenIndexMap;
use std::sync::atomic::{AtomicBool, AtomicUsize};

/// For Compose
pub(crate) struct Composer {
    pub(crate) compose_id_map: FrozenIndexMap<ChildKey, Box<ComposerId>>,
    pub(crate) instance_id_map: FrozenIndexMap<ChildKey, Box<InstanceId>>,
    rendered_flag: AtomicBool,
    next_component_index: AtomicUsize,
    next_compose_index: AtomicUsize,
    pub(crate) child_key_chain: ChildKeyChain,
}

impl Composer {
    pub(crate) fn new(child_key_chain: ChildKeyChain) -> Self {
        Self {
            compose_id_map: Default::default(),
            instance_id_map: Default::default(),
            rendered_flag: Default::default(),
            next_component_index: Default::default(),
            next_compose_index: Default::default(),
            child_key_chain,
        }
    }

    pub(crate) fn set_rendered_flag(&self) {
        self.rendered_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub(crate) fn take_rendered_flag(&self) -> bool {
        self.next_component_index
            .store(0, std::sync::atomic::Ordering::Relaxed);
        self.next_compose_index
            .store(0, std::sync::atomic::Ordering::Relaxed);
        self.rendered_flag
            .swap(false, std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn get_next_compose_index(&self) -> usize {
        self.next_compose_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn get_next_component_index(&self) -> usize {
        self.next_component_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}
