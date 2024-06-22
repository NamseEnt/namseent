use crate::*;
use std::{fmt::Debug, sync::OnceLock};

#[derive(Debug)]
pub struct Atom<State: 'static + Send + Sync> {
    index: OnceLock<usize>,
    sig_id: OnceLock<SigId>,
    set_state_tx: OnceLock<std::sync::mpsc::Sender<SendSyncSetStateItem>>,
    _phantom: std::marker::PhantomData<State>,
}

impl<State: 'static + Send + Sync> Atom<State> {
    pub const fn uninitialized() -> Self {
        Self {
            index: OnceLock::new(),
            sig_id: OnceLock::new(),
            set_state_tx: OnceLock::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub(crate) fn get_index(&self) -> usize {
        *self.index.get().unwrap()
    }

    pub fn is_initialized(&self) -> bool {
        self.index.get().is_some()
    }

    pub(crate) fn sig_id(&self) -> SigId {
        *self.sig_id.get().unwrap()
    }

    pub(crate) fn init(
        &self,
        set_state: &std::sync::mpsc::Sender<SendSyncSetStateItem>,
        index: usize,
    ) {
        let _ = self.index.get_or_init(|| {
            self.set_state_tx.set(set_state.clone()).unwrap();
            self.sig_id.set(SigId::Atom { index }).unwrap();
            index
        });
    }

    pub fn set(&self, state: State) {
        self.set_state_tx
            .get()
            .unwrap()
            .send(SendSyncSetStateItem::Set {
                sig_id: *self.sig_id.get().unwrap(),
                value: Box::new(state),
            })
            .unwrap();
    }

    pub fn mutate(&self, mutate: impl FnOnce(&mut State) + Send + Sync + 'static) {
        self.set_state_tx
            .get()
            .unwrap()
            .send(SendSyncSetStateItem::Mutate {
                sig_id: *self.sig_id.get().unwrap(),
                mutate: Box::new(move |value| {
                    let value = value.as_any_mut().downcast_mut::<State>().unwrap();
                    mutate(value);
                }),
            })
            .unwrap();
    }
}
