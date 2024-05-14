use rkyv::{de::deserializers::SharedDeserializeMap, Archived, Deserialize};
use std::ops::Deref;

pub struct HeapArchived<T> {
    buffer: Vec<u8>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> HeapArchived<T> {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn deserialize(&self) -> T
    where
        T: rkyv::Archive,
        T::Archived: Deserialize<T, SharedDeserializeMap>,
    {
        unsafe { rkyv::from_bytes_unchecked(&self.buffer).unwrap() }
    }
}

impl<T: rkyv::Archive> Deref for HeapArchived<T> {
    type Target = Archived<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { rkyv::archived_root::<T>(&self.buffer) }
    }
}
