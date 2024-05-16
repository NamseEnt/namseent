use crate::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SetState<'a, State: 'static> {
    sig_id: SigId,
    set_state_tx: &'a std::sync::mpsc::Sender<SetStateItem>,
    send_sync_set_state_tx: &'a std::sync::mpsc::Sender<SendSyncSetStateItem>,
    _state: std::marker::PhantomData<State>,
}

// This doesn't work with derive(Clone) so I have to implement it manually.
#[allow(clippy::non_canonical_clone_impl)]
impl<'a, State: 'static> Clone for SetState<'a, State> {
    fn clone(&self) -> Self {
        Self {
            sig_id: self.sig_id,
            set_state_tx: self.set_state_tx,
            send_sync_set_state_tx: self.send_sync_set_state_tx,
            _state: self._state,
        }
    }
}
impl<'a, State: 'static> Copy for SetState<'a, State> {}

impl<'a, State: 'static> SetState<'a, State> {
    pub(crate) fn new(
        sig_id: SigId,
        set_state_tx: &'a std::sync::mpsc::Sender<SetStateItem>,
        send_sync_set_state_tx: &'a std::sync::mpsc::Sender<SendSyncSetStateItem>,
    ) -> Self {
        Self {
            sig_id,
            set_state_tx,
            send_sync_set_state_tx,
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

    pub fn mutate(&self, mutate: impl FnOnce(&mut State) + 'static) {
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

    pub fn cloned(&self) -> StaticSetState<State>
    where
        State: Send + Sync,
    {
        StaticSetState::new(self.sig_id, self.send_sync_set_state_tx.clone())
    }
}


#[derive(Debug)]
pub struct AtomSetState<'a, State: Send + Sync + 'static> {
    sig_id: SigId,
    set_state_tx: &'a std::sync::mpsc::Sender<SendSyncSetStateItem>,
    _state: std::marker::PhantomData<State>,
}

// This doesn't work with derive(Clone) so I have to implement it manually.
#[allow(clippy::non_canonical_clone_impl)]
impl<'a, State: Send + Sync + 'static> Clone for AtomSetState<'a, State> {
    fn clone(&self) -> Self {
        Self {
            sig_id: self.sig_id,
            set_state_tx: self.set_state_tx,
            _state: self._state,
        }
    }
}
impl<'a, State: Send + Sync + 'static> Copy for AtomSetState<'a, State> {}

impl<'a, State: Send + Sync + 'static> AtomSetState<'a, State> {
    pub(crate) fn new(
        sig_id: SigId,
        set_state_tx: &'a std::sync::mpsc::Sender<SendSyncSetStateItem>,
    ) -> Self {
        Self {
            sig_id,
            set_state_tx,
            _state: std::marker::PhantomData,
        }
    }
    pub fn set(&self, state: State) {
        self.set_state_tx
            .send(SendSyncSetStateItem::Set {
                sig_id: self.sig_id,
                value: Box::new(state),
            })
            .unwrap();
    }

    pub fn mutate(&self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        self.set_state_tx
            .send(SendSyncSetStateItem::Mutate {
                sig_id: self.sig_id,
                mutate: Box::new(move |value| {
                    let value = value.as_any_mut().downcast_mut::<State>().unwrap();
                    mutate(value);
                }),
            })
            .unwrap();
    }

    pub fn cloned(&self) -> StaticSetState<State>
    {
        StaticSetState::new(self.sig_id, self.set_state_tx.clone())
    }
}

#[derive(Debug, Clone)]
pub struct StaticSetState<State: Send + Sync + 'static> {
    sig_id: SigId,
    set_state_tx: std::sync::mpsc::Sender<SendSyncSetStateItem>,
    _state: std::marker::PhantomData<State>,
}

impl<State: Send + Sync + 'static> StaticSetState<State> {
    pub(crate) fn new(
        sig_id: SigId,
        get_send_sync_set_state_tx: std::sync::mpsc::Sender<SendSyncSetStateItem>,
    ) -> StaticSetState<State> {
        StaticSetState {
            sig_id,
            set_state_tx: get_send_sync_set_state_tx,
            _state: std::marker::PhantomData,
        }
    }
    pub fn set(&self, state: State) {
        self.set_state_tx
            .send(SendSyncSetStateItem::Set {
                sig_id: self.sig_id,
                value: Box::new(state),
            })
            .unwrap();
    }

    pub fn mutate(&self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        self.set_state_tx
            .send(SendSyncSetStateItem::Mutate {
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
pub(crate) type MutateFnOnce = Box<dyn FnOnce(&mut (dyn Value))>;

pub(crate) enum SendSyncSetStateItem {
    Set {
        sig_id: SigId,
        value: Box<dyn Value + Send + Sync>,
    },
    Mutate {
        sig_id: SigId,
        mutate: SendSyncMutateFnOnce,
    },
}
pub(crate) type SendSyncMutateFnOnce = Box<dyn FnOnce(&mut (dyn Value)) + Send + Sync>;
