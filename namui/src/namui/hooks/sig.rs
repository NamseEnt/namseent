use super::*;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
    pub(crate) static USED_SIG_IDS: RefCell<HashSet<SigId>> = RefCell::new(HashSet::new());
}

pub(crate) fn clean_used_sigs() {
    USED_SIG_IDS.with(|ids| {
        let mut ids = ids.borrow_mut();
        ids.clear();
    })
}

pub(crate) fn take_used_sigs() -> Vec<SigId> {
    USED_SIG_IDS.with(|ids| {
        let mut ids = ids.borrow_mut();
        ids.drain().collect()
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct SigId {
    pub id_type: SigIdType,
    pub component_id: usize,
    pub index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum SigIdType {
    State,
    Memo,
    TrackEq,
    Atom, // component_id = 0
}

#[derive(Debug)]
pub struct Sig<'a, T: ?Sized> {
    id: SigId,
    value: &'a T,
}

impl<T: ?Sized> Clone for Sig<'_, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: self.value,
        }
    }
}

impl<T: ?Sized> Copy for Sig<'_, T> {}

impl<'a, T> Sig<'a, T> {
    pub(crate) fn new(value: &'a T, id: SigId) -> Self {
        Self { value, id }
    }
    fn use_it(&self) {
        USED_SIG_IDS.with(|ids| {
            let mut ids = ids.borrow_mut();
            ids.insert(self.id);
        });
    }
    pub fn on_effect(&self) -> bool {
        self.use_it();
        true
    }
}

impl<T> std::ops::Deref for Sig<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.use_it();
        self.value
    }
}
