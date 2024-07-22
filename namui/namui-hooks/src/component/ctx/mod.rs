mod effect_clean_up;
mod public_crate;

use crate::*;
pub use effect_clean_up::*;
use std::sync::atomic::AtomicUsize;

/// Component state management
pub struct ComponentCtx<'a> {
    world: &'a World,
    instance: &'a Instance,
    state_index: AtomicUsize,
    memo_index: AtomicUsize,
    track_eq_index: AtomicUsize,
    effect_index: AtomicUsize,
    interval_index: AtomicUsize,
}
impl<'a> ComponentCtx<'a> {
    pub(crate) fn new(world: &'a World, instance: &'a Instance) -> ComponentCtx<'a> {
        instance.set_rendered_flag();
        Self {
            world,
            instance,
            state_index: Default::default(),
            memo_index: Default::default(),
            track_eq_index: Default::default(),
            effect_index: Default::default(),
            interval_index: Default::default(),
        }
    }

    pub(crate) fn is_sig_updated(&self, target_sig_id: &SigId) -> bool {
        self.world.is_sig_updated(target_sig_id)
    }

    fn add_sig_updated(&self, sig_id: SigId) {
        self.world.add_sig_updated(sig_id)
    }
}
