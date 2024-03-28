use crate::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SetState<'a, State: 'static + Debug + Send + Sync> {
    sig_id: SigId,
    set_state_tx: &'a std::sync::mpsc::Sender<SetStateItem>,
    _state: std::marker::PhantomData<State>,
}

// This doesn't work with derive(Clone) so I have to implement it manually.
#[allow(clippy::non_canonical_clone_impl)]
impl<'a, State: 'static + Debug + Send + Sync> Clone for SetState<'a, State> {
    fn clone(&self) -> Self {
        Self {
            sig_id: self.sig_id,
            set_state_tx: self.set_state_tx,
            _state: self._state,
        }
    }
}
impl<'a, State: 'static + Debug + Send + Sync> Copy for SetState<'a, State> {}

impl<'a, State: 'static + Debug + Send + Sync> SetState<'a, State> {
    pub(crate) fn new(
        sig_id: SigId,
        set_state_tx: &'a std::sync::mpsc::Sender<SetStateItem>,
    ) -> Self {
        Self {
            sig_id,
            set_state_tx,
            _state: std::marker::PhantomData,
        }
    }
    pub fn set(&self, state: State) {
        self.set_state_tx
            .send(SetStateItem::Set {
                sig_id: self.sig_id,
                value: Box::new(state),
            })
            .unwrap();
    }

    pub fn mutate(&self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        self.set_state_tx
            .send(SetStateItem::Mutate {
                sig_id: self.sig_id,
                mutate: Box::new(move |value| {
                    let value = value.as_any_mut().downcast_mut::<State>().unwrap();
                    mutate(value);
                }),
            })
            .unwrap();
    }

    pub fn cloned(&self) -> StaticSetState<State> {
        StaticSetState {
            sig_id: self.sig_id,
            set_state_tx: self.set_state_tx.clone(),
            _state: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StaticSetState<State: 'static + Debug + Send + Sync> {
    sig_id: SigId,
    set_state_tx: std::sync::mpsc::Sender<SetStateItem>,
    _state: std::marker::PhantomData<State>,
}

impl<State: 'static + Debug + Send + Sync> StaticSetState<State> {
    pub fn set(&self, state: State) {
        self.set_state_tx
            .send(SetStateItem::Set {
                sig_id: self.sig_id,
                value: Box::new(state),
            })
            .unwrap();
    }

    pub fn mutate(&self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        self.set_state_tx
            .send(SetStateItem::Mutate {
                sig_id: self.sig_id,
                mutate: Box::new(move |value| {
                    let value = value.as_any_mut().downcast_mut::<State>().unwrap();
                    mutate(value);
                }),
            })
            .unwrap();
    }
}

pub(crate) enum SetStateItem {
    Set {
        sig_id: SigId,
        value: Box<dyn Value>,
    },
    Mutate {
        sig_id: SigId,
        mutate: MutateFnOnce,
    },
}
pub(crate) type MutateFnOnce = Box<dyn FnOnce(&mut (dyn Value)) + Send + Sync>;
