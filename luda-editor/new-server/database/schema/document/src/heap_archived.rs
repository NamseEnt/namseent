use crate::*;
use rkyv::Archived;
use std::{
    fmt::{Debug, Pointer},
    ops::Deref,
};

pub struct HeapArchived<T> {
    bytes: Bytes,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> HeapArchived<T> {
    pub fn new(bytes: Bytes) -> Self {
        Self {
            bytes,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: rkyv::Archive> Deref for HeapArchived<T> {
    type Target = Archived<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { rkyv::access_unchecked(self.bytes.as_ref()) }
    }
}

impl<T: Debug + rkyv::Archive> Debug for HeapArchived<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
