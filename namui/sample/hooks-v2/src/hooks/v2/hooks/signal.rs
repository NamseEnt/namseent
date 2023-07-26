use std::{cell::RefCell, collections::HashSet, sync::Arc};

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
pub(crate) enum SignalId {
    State(StateSignalId),
    Memo(MemoSignalId),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct StateSignalId {
    pub component_id: usize,
    pub state_index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct MemoSignalId {
    pub component_id: usize,
    pub memo_index: usize,
}

#[derive(Clone, Debug)]
pub struct Signal<T> {
    id: SignalId,
    value: Arc<T>,
}

impl<T> Signal<T> {
    pub(crate) fn new(value: Arc<T>, id: SignalId) -> Self {
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

impl<T> std::ops::Deref for Signal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.use_it();
        self.value.as_ref()
    }
}
