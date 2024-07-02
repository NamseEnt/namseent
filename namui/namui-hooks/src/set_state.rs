use crate::*;
use std::{fmt::Debug, sync::mpsc};

#[derive(Clone, Copy, Debug)]
pub struct SetState<State: 'static + Send> {
    sig_id: SigId,
    set_state_tx: &'static mpsc::Sender<SetStateItem>,
    _state: std::marker::PhantomData<State>,
}

impl<State: 'static + Send> SetState<State> {
    pub(crate) fn new(sig_id: SigId, set_state_tx: &'static mpsc::Sender<SetStateItem>) -> Self {
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

    pub fn mutate(&self, mutate: impl FnOnce(&mut State) + 'static + Send) {
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
        value: Box<dyn Value + Send>,
    },
    Mutate {
        sig_id: SigId,
        mutate: MutateFnOnce,
    },
}
pub(crate) type MutateFnOnce = Box<dyn FnOnce(&mut (dyn Value)) + Send>;
