use crate::*;
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        OnceLock,
    },
};

#[derive(Debug)]
pub struct Atom<State: 'static + Send + Sync> {
    initialized: AtomicBool,
    index: AtomicUsize,
    sig_id: OnceLock<SigId>,
    set_state_tx: OnceLock<std::sync::mpsc::Sender<SendSyncSetStateItem>>,
    _phantom: std::marker::PhantomData<State>,
}
static NEXT_INDEX: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) fn reset_next_index() {
    NEXT_INDEX.store(0, Ordering::Relaxed);
}

impl<State: 'static + Send + Sync> Atom<State> {
    pub const fn uninitialized() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            index: AtomicUsize::new(0),
            sig_id: OnceLock::new(),
            set_state_tx: OnceLock::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub(crate) fn get_index(&self) -> usize {
        assert!(self.initialized.load(Ordering::Relaxed));
        self.index.load(Ordering::Relaxed)
    }

    pub(crate) fn init(&self, set_state: &std::sync::mpsc::Sender<SendSyncSetStateItem>) -> usize {
        if self.initialized.load(Ordering::Relaxed) {
            return self.index.load(Ordering::Relaxed);
        }

        self.set_state_tx.set(set_state.clone()).unwrap();

        let next_index = NEXT_INDEX.fetch_add(1, Ordering::Relaxed);
        self.index.store(next_index, Ordering::Relaxed);
        self.initialized.store(true, Ordering::Relaxed);

        self.sig_id.set(SigId::Atom { index: next_index }).unwrap();

        next_index
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
