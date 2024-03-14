use crate::*;
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
};

pub struct Sig<'world, T: ?Sized, Ref: Borrow<T>> {
    id: SigId,
    value: Ref,
    world: &'world World,
    _phantom: std::marker::PhantomData<T>,
}

impl<'world, T: ?Sized, Ref: Borrow<T>> Sig<'world, T, Ref> {
    pub(crate) fn new(value: Ref, id: SigId, world: &'world World) -> Self {
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
}

impl<T: ?Sized + Debug, Ref: Borrow<T>> Debug for Sig<'_, T, Ref> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sig")
            .field("id", &self.id)
            .field("value", &self.value.borrow())
            .finish()
    }
}

impl<T: ?Sized + Display, Ref: Borrow<T>> Display for Sig<'_, T, Ref> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.borrow().fmt(f)
    }
}

impl<T: ?Sized, Ref: Borrow<T>> std::ops::Deref for Sig<'_, T, Ref> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.record_as_used();
        self.value.borrow()
    }
}

impl<T: ?Sized, Ref: Borrow<T>> AsRef<T> for Sig<'_, T, Ref> {
    fn as_ref(&self) -> &T {
        self.record_as_used();
        self.value.borrow()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum SigId {
    State {
        instance_id: InstanceId,
        index: usize,
    },
    Memo {
        instance_id: InstanceId,
        index: usize,
    },
    Atom {
        index: usize,
    },
}
