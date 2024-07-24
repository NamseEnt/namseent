use crate::*;
use std::{
    cell::{RefCell, UnsafeCell},
    rc::Rc,
    sync::atomic::AtomicBool,
};

/// the state of component.
pub(crate) struct Instance {
    pub(crate) id: InstanceId,
    rendered_flag: AtomicBool,
    pub(crate) state_list: UnsafeCell<Vec<Box<dyn Value>>>,
    pub(crate) memo_list: RefCell<Vec<Memo>>,
    pub(crate) track_eq_list: RefCell<Vec<Rc<dyn Value>>>,
    pub(crate) track_eq_tuple_list: RefCell<Vec<()>>,
    pub(crate) effect_list: RefCell<Vec<Effect>>,
    pub(crate) interval_called_list: RefCell<Vec<Instant>>,
    pub(crate) abort_handle_list: RefCell<Vec<tokio::task::AbortHandle>>,
}
impl Instance {
    pub(crate) fn new(id: InstanceId) -> Self {
        Self {
            id,
            rendered_flag: Default::default(),
            state_list: Default::default(),
            memo_list: Default::default(),
            track_eq_list: Default::default(),
            track_eq_tuple_list: Default::default(),
            effect_list: Default::default(),
            interval_called_list: Default::default(),
            abort_handle_list: Default::default(),
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

impl Drop for Instance {
    fn drop(&mut self) {
        let effect_list = self.effect_list.take();
        for effect in effect_list {
            effect.clean_up.call();
        }
        let abort_handle_list = self.abort_handle_list.take();
        for abort_handle in abort_handle_list {
            abort_handle.abort();
        }
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
