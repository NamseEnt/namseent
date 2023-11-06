use super::super::{any_clone_partial_eq::AnyPartialEq, component_tree::Key};
use crate::hooks::{component_tree, update::invoke_update};
use std::{
    any::Any,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, OnceLock},
};

static COMPONENT_KEY: OnceLock<Key> = OnceLock::new();
static mut STORED_STATES: OnceLock<Vec<Arc<dyn AnyPartialEq>>> = OnceLock::new();
static STORED_STATE_INDEX: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn set_up_state_before_render(key: Key, states: Vec<Arc<dyn AnyPartialEq>>) {
    let _ = COMPONENT_KEY.set(key);
    unsafe {
        let _ = STORED_STATES.set(states);
    }
    STORED_STATE_INDEX.store(0, std::sync::atomic::Ordering::SeqCst);
}

pub(crate) fn get_back_states() -> Vec<Arc<dyn AnyPartialEq>> {
    unsafe { STORED_STATES.take().unwrap() }
}

pub fn state<'a, T: 'static + Any + Clone + PartialEq + Debug>(initial: T) -> (&'a T, SetState<T>) {
    let component_key = COMPONENT_KEY.get().unwrap().clone();
    let state_index: usize = STORED_STATE_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let state = {
        let states = unsafe { STORED_STATES.get().unwrap() };
        if states.get(state_index).is_none() {
            unsafe {
                STORED_STATES
                    .get_mut()
                    .unwrap()
                    .insert(state_index, Arc::new(initial.clone()));
            }
        }
        states[state_index].clone()
    };
    let set_state = SetState::new(component_key, state_index);

    let state_ptr = Arc::as_ptr(&state);
    let state_ref = unsafe { &*state_ptr };
    (state_ref.as_any().downcast_ref::<T>().unwrap(), set_state)
}

#[derive(Clone, PartialEq, Debug)]
pub struct SetState<T> {
    _marker: std::marker::PhantomData<T>,
    component_key: Key,
    state_index: usize,
}

impl<T: Clone> Copy for SetState<T> {}

impl<T: 'static + Any + Clone + PartialEq + Debug> SetState<T> {
    pub fn invoke(&self, next_state_fn: impl FnOnce(&mut T)) {
        component_tree::update_component_state(self.component_key, self.state_index, next_state_fn);
        invoke_update(self.component_key, Arc::new(()))
    }

    fn new(component_key: Key, state_index: usize) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            component_key,
            state_index,
        }
    }
}
