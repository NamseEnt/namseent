use super::*;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
    pub(crate) static USED_SIGNAL_IDS: RefCell<HashSet<SignalId>> = RefCell::new(HashSet::new());
}

pub(crate) fn clean_used_signals() {
    USED_SIGNAL_IDS.with(|ids| {
        let mut ids = ids.borrow_mut();
        ids.clear();
    })
}

pub(crate) fn take_used_signals() -> Vec<SignalId> {
    USED_SIGNAL_IDS.with(|ids| {
        let mut ids = ids.borrow_mut();
        ids.drain().collect()
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct SignalId {
    pub id_type: SignalIdType,
    pub component_id: usize,
    pub index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum SignalIdType {
    State,
    Memo,
    Map,
    As,
}

#[derive(Clone, Debug)]
pub struct Signal<'a, T: ?Sized> {
    id: SignalId,
    value: &'a T,
}

impl<'a, T> Signal<'a, T> {
    pub(crate) fn new(value: &'a T, id: SignalId) -> Self {
        Self { value, id }
    }
    fn use_it(&self) {
        USED_SIGNAL_IDS.with(|ids| {
            let mut ids = ids.borrow_mut();
            ids.insert(self.id);
        });
    }
    pub fn on_effect(&self) -> bool {
        self.use_it();
        true
    }
}

impl<T> std::ops::Deref for Signal<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.use_it();
        self.value
    }
}

pub trait AsSignal {
    fn as_signal<'a>(&'a self) -> Signal<'a, Self>;
}

impl<T> AsSignal for T {
    fn as_signal<'a>(&'a self) -> Signal<'a, Self> {
        let ctx = ctx::ctx();
        Signal::new(
            self,
            SignalId {
                id_type: SignalIdType::As,
                component_id: ctx.instance.as_ref().component_id,
                index: ctx
                    .as_index
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            },
        )
    }
}
