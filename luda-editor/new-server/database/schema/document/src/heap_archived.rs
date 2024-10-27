use crate::*;
use rkyv::{de::deserializers::SharedDeserializeMap, Archived, Deserialize};
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
    #[allow(dead_code)]
    pub fn deserialize(&self) -> T
    where
        T: rkyv::Archive,
        T::Archived: Deserialize<T, SharedDeserializeMap>,
    {
        unsafe { rkyv::from_bytes_unchecked(self.bytes.as_ref()).unwrap() }
    }
}

impl<T: rkyv::Archive> Deref for HeapArchived<T> {
    type Target = Archived<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { rkyv::archived_root::<T>(self.bytes.as_ref()) }
    }
}

impl<T: Debug + rkyv::Archive> Debug for HeapArchived<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
