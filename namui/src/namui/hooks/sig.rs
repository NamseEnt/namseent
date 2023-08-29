use super::*;
use std::{cell::RefCell, collections::HashSet, fmt::Display};

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

impl<T: ?Sized + Debug> Debug for Sig<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sig")
            .field("id", &self.id)
            .field("value", &self.value)
            .finish()
    }
}

impl<T: ?Sized + Display> Display for Sig<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

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
    pub fn clone_inner(&self) -> T
    where
        T: Clone,
    {
        self.use_it();
        self.value.clone()
    }
}

impl<T> std::ops::Deref for Sig<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.use_it();
        self.value
    }
}

impl<T> AsRef<T> for Sig<'_, T> {
    fn as_ref(&self) -> &T {
        self.use_it();
        self.value
    }
}

impl<'a, T0, T1> Sig<'a, (T0, T1)> {
    pub fn map_0(self) -> Sig<'a, T0> {
        Sig {
            id: self.id,
            value: &self.value.0,
        }
    }
    pub fn map_1(self) -> Sig<'a, T1> {
        Sig {
            id: self.id,
            value: &self.value.1,
        }
    }
}
