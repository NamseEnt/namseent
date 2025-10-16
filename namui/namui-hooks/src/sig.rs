use crate::*;
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy)]
pub struct Sig<'a, T: ?Sized> {
    pub(crate) id: SigId,
    value: &'a T,
    world: &'a World,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: ?Sized> Sig<'a, T> {
    pub(crate) fn new(value: &'a T, id: SigId, world: &'a World) -> Self {
        Self {
            value,
            id,
            world,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn record_as_used(&self) {
        self.world.record_used_sig(self.id);
    }
    pub fn clone_inner(&self) -> T
    where
        T: Clone,
    {
        self.value.borrow().clone()
    }
    pub fn map<U: ?Sized, F: FnOnce(&T) -> &U>(&self, f: F) -> Sig<'a, U> {
        Sig::new(f(self.value), self.id, self.world)
    }
    pub fn is_updated(&self) -> bool {
        self.world.is_sig_updated(&self.id)
    }
}

impl<T: ?Sized + Debug> Debug for Sig<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sig")
            .field("id", &self.id)
            .field("value", &self.value.borrow())
            .finish()
    }
}

impl<T: ?Sized + Display> Display for Sig<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.borrow().fmt(f)
    }
}

impl<T: ?Sized> std::ops::Deref for Sig<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.record_as_used();
        self.value.borrow()
    }
}

impl<T: ?Sized> AsRef<T> for Sig<'_, T> {
    fn as_ref(&self) -> &T {
        self.record_as_used();
        self.value.borrow()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum SigId {
    State { instance_id: usize, index: usize },
    Memo { instance_id: usize, index: usize },
    TrackEq { instance_id: usize, index: usize },
    Atom { index: usize },
}
