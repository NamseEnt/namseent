mod effect_clean_up;
mod public_crate;

use crate::*;
pub use effect_clean_up::*;
use std::cell::Cell;

/// Component state management
pub struct ComponentCtx<'a> {
    world: &'a World,
    instance: &'a Instance,
    state_index: Cell<usize>,
    memo_index: Cell<usize>,
    track_eq_index: Cell<usize>,
    track_eq_tuple_index: Cell<usize>,
    effect_index: Cell<usize>,
    interval_index: Cell<usize>,
}
impl<'a> ComponentCtx<'a> {
    pub(crate) fn new(world: &'a World, instance: &'a Instance) -> ComponentCtx<'a> {
        if instance.mark_rendered(world.frame()) {
            world.count_rendered_instance();
        }
        Self {
            world,
            instance,
            state_index: Cell::new(0),
            memo_index: Cell::new(0),
            track_eq_index: Cell::new(0),
            track_eq_tuple_index: Cell::new(0),
            effect_index: Cell::new(0),
            interval_index: Cell::new(0),
        }
    }

    pub(crate) fn is_sig_updated(&self, target_sig_id: &SigId) -> bool {
        self.world.is_sig_updated(target_sig_id)
    }

    fn add_sig_updated(&self, sig_id: SigId) {
        self.world.add_sig_updated(sig_id)
    }
}

fn next_index(cell: &Cell<usize>) -> usize {
    let index = cell.get();
    cell.set(index + 1);
    index
}
