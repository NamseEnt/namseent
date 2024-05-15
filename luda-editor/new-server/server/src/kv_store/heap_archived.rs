use super::*;
use rkyv::{de::deserializers::SharedDeserializeMap, Archived, Deserialize};
use std::{ops::Deref, sync::Arc};

pub struct HeapArchived<T> {
    buffer: ValueBuffer,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> HeapArchived<T> {
    pub(crate) fn new(buffer: impl Into<ValueBuffer>) -> Self {
        Self {
            buffer: buffer.into(),
            _phantom: std::marker::PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn deserialize(&self) -> T
    where
        T: rkyv::Archive,
        T::Archived: Deserialize<T, SharedDeserializeMap>,
    {
        unsafe { rkyv::from_bytes_unchecked(self.buffer.as_slice()).unwrap() }
    }
    #[allow(dead_code)]
    pub(super) fn get_arc_vec(&self) -> Arc<Vec<u8>> {
        match &self.buffer {
            ValueBuffer::Vec(vec) => Arc::new(vec.clone()),
            ValueBuffer::Arc(arc) => arc.clone(),
        }
    }
}

impl<T: rkyv::Archive> Deref for HeapArchived<T> {
    type Target = Archived<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { rkyv::archived_root::<T>(self.buffer.as_slice()) }
    }
}
