use rkyv::{de::deserializers::SharedDeserializeMap, Archived, Deserialize};
use std::{ops::Deref, sync::Arc};

pub struct HeapArchived<T> {
    buffer: HeapBuffer,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> HeapArchived<T> {
    pub fn new(buffer: impl Into<HeapBuffer>) -> Self {
        Self {
            buffer: buffer.into(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn deserialize(&self) -> T
    where
        T: rkyv::Archive,
        T::Archived: Deserialize<T, SharedDeserializeMap>,
    {
        unsafe { rkyv::from_bytes_unchecked(self.buffer.as_slice()).unwrap() }
    }
    pub(super) fn get_arc_vec(&self) -> Arc<Vec<u8>> {
        match &self.buffer {
            HeapBuffer::Vec(vec) => Arc::new(vec.clone()),
            HeapBuffer::Arc(arc) => arc.clone(),
        }
    }
}

impl<T: rkyv::Archive> Deref for HeapArchived<T> {
    type Target = Archived<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { rkyv::archived_root::<T>(self.buffer.as_slice()) }
    }
}

pub enum HeapBuffer {
    Vec(Vec<u8>),
    Arc(Arc<Vec<u8>>),
}

impl From<Vec<u8>> for HeapBuffer {
    fn from(vec: Vec<u8>) -> Self {
        Self::Vec(vec)
    }
}

impl From<Arc<Vec<u8>>> for HeapBuffer {
    fn from(arc: Arc<Vec<u8>>) -> Self {
        Self::Arc(arc)
    }
}

impl HeapBuffer {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Vec(vec) => vec.as_slice(),
            Self::Arc(arc) => arc.as_slice(),
        }
    }
}
