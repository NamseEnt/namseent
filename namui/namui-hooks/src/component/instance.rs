use crate::*;
use elsa::FrozenVec;
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};

/// the state of component.
pub(crate) struct Instance {
    pub(crate) id: InstanceId,
    rendered_flag: AtomicBool,
    pub(crate) state_list: FrozenVec<Box<dyn Value>>,
    pub(crate) memo_list: RefCell<Vec<Memo>>,
    pub(crate) track_eq_list: RefCell<Vec<Rc<dyn Value>>>,
    pub(crate) effect_list: RefCell<Vec<Effect>>,
    pub(crate) interval_called_list: RefCell<Vec<Instant>>,
}
impl Instance {
    pub(crate) fn new(id: InstanceId) -> Self {
        Self {
            id,
            rendered_flag: Default::default(),
            state_list: Default::default(),
            memo_list: Default::default(),
            track_eq_list: Default::default(),
            effect_list: Default::default(),
            interval_called_list: Default::default(),
        }
    }

    pub(crate) fn set_rendered_flag(&self) {
        self.rendered_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub(crate) fn take_rendered_flag(&mut self) -> bool {
        self.rendered_flag
            .swap(false, std::sync::atomic::Ordering::Relaxed)
    }
}

pub(crate) struct Memo {
    pub(crate) value: Rc<dyn Value>,
    pub(crate) used_sig_ids: Vec<SigId>,
}

#[derive(Default)]
pub(crate) struct Effect {
    pub(crate) used_sig_ids: Vec<SigId>,
    pub(crate) clean_up: EffectCleanUp,
}
