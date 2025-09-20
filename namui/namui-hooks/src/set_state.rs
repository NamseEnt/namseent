use crate::*;
use std::{fmt::Debug, sync::mpsc};

#[derive(Debug)]
pub struct SetState<State: 'static + Send> {
    sig_id: SigId,
    set_state_tx: &'static mpsc::Sender<SetStateItem>,
    _state: std::marker::PhantomData<State>,
}

impl<State: 'static + Send> Clone for SetState<State> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<State: 'static + Send> Copy for SetState<State> {}

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
                mutate: Box::new(move |value| mutate(value.as_any_mut().downcast_mut().unwrap())),
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
    Mutate2 {
        sig_ids: (SigId, SigId),
        mutate: MutateFnOnce2,
    },
    Mutate3 {
        sig_ids: (SigId, SigId, SigId),
        mutate: MutateFnOnce3,
    },
    Mutate4 {
        sig_ids: (SigId, SigId, SigId, SigId),
        mutate: MutateFnOnce4,
    },
}
pub(crate) type MutateFnOnce = Box<dyn FnOnce(&mut dyn Value) + Send>;
pub(crate) type MutateFnOnce2 = Box<dyn FnOnce((&mut dyn Value, &mut dyn Value)) + Send>;
pub(crate) type MutateFnOnce3 =
    Box<dyn FnOnce((&mut dyn Value, &mut dyn Value, &mut dyn Value)) + Send>;
pub(crate) type MutateFnOnce4 = Box<
    dyn FnOnce(
            (
                &mut dyn Value,
                &mut dyn Value,
                &mut dyn Value,
                &mut dyn Value,
            ),
        ) + Send,
>;

pub trait MutateState2<S1, S2> {
    fn mutate(&self, mutate: impl FnOnce((&mut S1, &mut S2)) + 'static + Send);
}
impl<S1: 'static + Send, S2: 'static + Send> MutateState2<S1, S2> for (SetState<S1>, SetState<S2>) {
    fn mutate(&self, mutate: impl FnOnce((&mut S1, &mut S2)) + 'static + Send) {
        self.0
            .set_state_tx
            .send(SetStateItem::Mutate2 {
                sig_ids: (self.0.sig_id, self.1.sig_id),
                mutate: Box::new(move |(value1, value2)| {
                    mutate((
                        value1.as_any_mut().downcast_mut().unwrap(),
                        value2.as_any_mut().downcast_mut().unwrap(),
                    ))
                }),
            })
            .unwrap();
    }
}

pub trait MutateState3<S1, S2, S3> {
    fn mutate(&self, mutate: impl FnOnce((&mut S1, &mut S2, &mut S3)) + 'static + Send);
}
impl<S1: 'static + Send, S2: 'static + Send, S3: 'static + Send> MutateState3<S1, S2, S3>
    for (SetState<S1>, SetState<S2>, SetState<S3>)
{
    fn mutate(&self, mutate: impl FnOnce((&mut S1, &mut S2, &mut S3)) + 'static + Send) {
        self.0
            .set_state_tx
            .send(SetStateItem::Mutate3 {
                sig_ids: (self.0.sig_id, self.1.sig_id, self.2.sig_id),
                mutate: Box::new(move |(value1, value2, value3)| {
                    mutate((
                        value1.as_any_mut().downcast_mut().unwrap(),
                        value2.as_any_mut().downcast_mut().unwrap(),
                        value3.as_any_mut().downcast_mut().unwrap(),
                    ))
                }),
            })
            .unwrap();
    }
}

pub trait MutateState4<S1, S2, S3, S4> {
    fn mutate(&self, mutate: impl FnOnce((&mut S1, &mut S2, &mut S3, &mut S4)) + 'static + Send);
}
impl<S1: 'static + Send, S2: 'static + Send, S3: 'static + Send, S4: 'static + Send>
    MutateState4<S1, S2, S3, S4> for (SetState<S1>, SetState<S2>, SetState<S3>, SetState<S4>)
{
    fn mutate(&self, mutate: impl FnOnce((&mut S1, &mut S2, &mut S3, &mut S4)) + 'static + Send) {
        self.0
            .set_state_tx
            .send(SetStateItem::Mutate4 {
                sig_ids: (self.0.sig_id, self.1.sig_id, self.2.sig_id, self.3.sig_id),
                mutate: Box::new(move |(value1, value2, value3, value4)| {
                    mutate((
                        value1.as_any_mut().downcast_mut().unwrap(),
                        value2.as_any_mut().downcast_mut().unwrap(),
                        value3.as_any_mut().downcast_mut().unwrap(),
                        value4.as_any_mut().downcast_mut().unwrap(),
                    ))
                }),
            })
            .unwrap();
    }
}
