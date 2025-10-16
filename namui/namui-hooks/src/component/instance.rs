use super::*;
use crate::*;
use std::{
    cell::{RefCell, UnsafeCell},
    ops::Deref,
    sync::atomic::AtomicBool,
};

/// the state of component.
pub(crate) struct Instance {
    pub(crate) id: InstanceId,
    rendered_flag: AtomicBool,
    pub(crate) state_list: UnsafeCell<Vec<Box<dyn Value>>>,
    pub(crate) memo_list: UnsafeCell<Vec<Memo>>,
    pub(crate) track_eq_list: UnsafeCell<Vec<Box<dyn Value>>>,
    pub(crate) track_eq_tuple_list: RefCell<Vec<()>>,
    pub(crate) effect_list: RefCell<Vec<Effect>>,
    pub(crate) interval_called_list: RefCell<Vec<Instant>>,
    pub(crate) abort_handle_list: RefCell<Vec<tokio::task::AbortHandle>>,
    frozen_instance: Option<FrozenInstance>,
}
impl Instance {
    pub(crate) fn new(id: InstanceId, frozen_instance: Option<FrozenInstance>) -> Self {
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
            frozen_instance,
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

    pub(crate) fn freeze(self) -> Vec<u8> {
        let mut buf = vec![];
        FrozenInstance::from_instance(self).serialize(&mut buf);
        buf
    }

    pub(crate) fn state_value<State: crate::State>(
        &self,
        state_index: usize,
        init: impl FnOnce() -> State,
    ) -> &dyn Value {
        unsafe {
            let state_list = &mut *self.state_list.get();

            let no_state = state_list.len() <= state_index;

            if no_state {
                let state = if let Some(frozen_instance) = &self.frozen_instance
                    && let Some(bytes) = frozen_instance.state_list.get(state_index)
                {
                    State::deserialize(&mut bytes.as_slice()).unwrap()
                } else {
                    init()
                };
                state_list.push(Box::new(state));
                assert_eq!(state_list.len(), state_index + 1);
            };

            state_list.get(state_index).unwrap().deref()
        }
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

#[derive(OurSer)]
pub(crate) struct Memo {
    pub(crate) value: Box<dyn Value>,
    pub(crate) used_sig_ids: Vec<SigId>,
}

#[derive(Default)]
pub(crate) struct Effect {
    pub(crate) used_sig_ids: Vec<SigId>,
    pub(crate) clean_up: EffectCleanUp,
}

#[derive(OurSerde)]
pub(crate) struct FrozenInstance {
    pub(crate) id: InstanceId,
    pub(crate) state_list: Vec<Vec<u8>>,
}
impl FrozenInstance {
    pub(crate) fn from_instance(mut instance: Instance) -> Self {
        let state_list = std::mem::take(&mut instance.state_list)
            .into_inner()
            .into_iter()
            .map(|state| {
                let mut bytes = vec![];
                state.serialize(&mut bytes);
                bytes
            })
            .collect();

        Self {
            id: instance.id,
            state_list,
        }
    }
    pub(crate) fn from_bytes(mut bytes: &[u8]) -> Self {
        Self::deserialize(&mut bytes).unwrap()
    }
}
